//! Manages registry connections and reading/writing to them

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use std::{io, mem, str, u32};

use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, ReadBuf};
use tracing::{debug, info};

use crate::error::Error;

/// EPP Connection struct with some metadata for the connection
pub(crate) struct EppConnection<C: Connector> {
    pub registry: String,
    connector: C,
    stream: C::Connection,
    pub greeting: String,
    timeout: Duration,
    // A request that is currently in flight
    //
    // Because the code here currently depends on only one request being in flight at a time,
    // this needs to be finished (written, and response read) before we start another one.
    current: Option<RequestState>,
    // The next request to be sent
    //
    // If we get a request while another request is in flight (because its future was dropped),
    // we will store it here until the current request is finished.
    next: Option<RequestState>,
}

impl<C: Connector> EppConnection<C> {
    pub(crate) async fn new(
        connector: C,
        registry: String,
        timeout: Duration,
    ) -> Result<Self, Error> {
        let mut this = Self {
            registry,
            stream: connector.connect(timeout).await?,
            connector,
            greeting: String::new(),
            timeout,
            current: None,
            next: None,
        };

        this.read_greeting().await?;
        Ok(this)
    }

    async fn read_greeting(&mut self) -> Result<(), Error> {
        assert!(self.current.is_none());
        self.current = Some(RequestState::ReadLength {
            read: 0,
            buf: vec![0; 256],
        });

        self.greeting = RequestFuture { conn: self }.await?;
        Ok(())
    }

    pub(crate) async fn reconnect(&mut self) -> Result<(), Error> {
        debug!("{}: reconnecting", self.registry);
        let _ = self.current.take();
        let _ = self.next.take();
        self.stream = self.connector.connect(self.timeout).await?;
        self.read_greeting().await?;
        Ok(())
    }

    /// Sends an EPP XML request to the registry and returns the response
    pub(crate) fn transact<'a>(&'a mut self, command: &str) -> Result<RequestFuture<'a, C>, Error> {
        let new = RequestState::new(command)?;

        // If we have a request currently in flight, finish that first
        // If another request was queued up behind the one in flight, just replace it
        match self.current.is_some() {
            true => {
                debug!(
                    "{}: Queueing up request in order to finish in-flight request",
                    self.registry
                );
                self.next = Some(new);
            }
            false => self.current = Some(new),
        }

        Ok(RequestFuture { conn: self })
    }

    /// Closes the socket and shuts down the connection
    pub(crate) async fn shutdown(&mut self) -> Result<(), Error> {
        info!("{}: Closing connection", self.registry);
        timeout(self.timeout, self.stream.shutdown()).await?;
        Ok(())
    }

    fn handle(
        &mut self,
        mut state: RequestState,
        cx: &mut Context<'_>,
    ) -> Result<Transition, Error> {
        match &mut state {
            RequestState::Writing { mut start, buf } => {
                let wrote = match Pin::new(&mut self.stream).poll_write(cx, &buf[start..]) {
                    Poll::Ready(Ok(wrote)) => wrote,
                    Poll::Ready(Err(err)) => return Err(err.into()),
                    Poll::Pending => return Ok(Transition::Pending(state)),
                };

                if wrote == 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        format!("{}: Unexpected EOF while writing", self.registry),
                    )
                    .into());
                }

                start += wrote;
                debug!(
                    "{}: Wrote {} bytes, {} out of {} done",
                    self.registry,
                    wrote,
                    start,
                    buf.len()
                );

                // Transition to reading the response's frame header once
                // we've written the entire request
                if start < buf.len() {
                    return Ok(Transition::Next(state));
                }

                Ok(Transition::Next(RequestState::ReadLength {
                    read: 0,
                    buf: vec![0; 256],
                }))
            }
            RequestState::ReadLength { mut read, buf } => {
                let mut read_buf = ReadBuf::new(&mut buf[read..]);
                match Pin::new(&mut self.stream).poll_read(cx, &mut read_buf) {
                    Poll::Ready(Ok(())) => {}
                    Poll::Ready(Err(err)) => return Err(err.into()),
                    Poll::Pending => return Ok(Transition::Pending(state)),
                };

                let filled = read_buf.filled();
                if filled.is_empty() {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        format!("{}: Unexpected EOF while reading length", self.registry),
                    )
                    .into());
                }

                // We're looking for the frame header which tells us how long the response will be.
                // The frame header is a 32-bit (4-byte) big-endian unsigned integer. If we don't
                // have 4 bytes yet, stay in the `ReadLength` state, otherwise we transition to `Reading`.

                read += filled.len();
                if read < 4 {
                    return Ok(Transition::Next(state));
                }

                let expected = u32::from_be_bytes(filled[..4].try_into()?) as usize;
                debug!("{}: Expected response length: {}", self.registry, expected);
                buf.resize(expected, 0);
                Ok(Transition::Next(RequestState::Reading {
                    read,
                    buf: mem::take(buf),
                    expected,
                }))
            }
            RequestState::Reading {
                mut read,
                buf,
                expected,
            } => {
                let mut read_buf = ReadBuf::new(&mut buf[read..]);
                match Pin::new(&mut self.stream).poll_read(cx, &mut read_buf) {
                    Poll::Ready(Ok(())) => {}
                    Poll::Ready(Err(err)) => return Err(err.into()),
                    Poll::Pending => return Ok(Transition::Pending(state)),
                }

                let filled = read_buf.filled();
                if filled.is_empty() {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        format!("{}: Unexpected EOF while reading", self.registry),
                    )
                    .into());
                }

                read += filled.len();
                debug!(
                    "{}: Read {} bytes, {} out of {} done",
                    self.registry,
                    filled.len(),
                    read,
                    expected
                );

                //

                Ok(if read < *expected {
                    // If we haven't received the entire response yet, stick to the `Reading` state.
                    Transition::Next(state)
                } else if let Some(next) = self.next.take() {
                    // Otherwise, if we were just pushing through this request because it was already
                    // in flight when we started a new one, ignore this response and move to the
                    // next request (the one this `RequestFuture` is actually for).
                    Transition::Next(next)
                } else {
                    // Otherwise, drain off the frame header and convert the rest to a `String`.
                    buf.drain(..4);
                    Transition::Done(String::from_utf8(mem::take(buf))?)
                })
            }
        }
    }
}

pub(crate) struct RequestFuture<'a, C: Connector> {
    conn: &'a mut EppConnection<C>,
}

impl<'a, C: Connector> Future for RequestFuture<'a, C> {
    type Output = Result<String, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        loop {
            let state = this.conn.current.take().unwrap();
            match this.conn.handle(state, cx) {
                Ok(Transition::Next(next)) => {
                    this.conn.current = Some(next);
                    continue;
                }
                Ok(Transition::Pending(state)) => {
                    this.conn.current = Some(state);
                    return Poll::Pending;
                }
                Ok(Transition::Done(rsp)) => return Poll::Ready(Ok(rsp)),
                Err(err) => {
                    // Assume the error means the connection can no longer be used
                    this.conn.next = None;
                    return Poll::Ready(Err(err));
                }
            }
        }
    }
}

// Transitions between `RequestState`s
enum Transition {
    Pending(RequestState),
    Next(RequestState),
    Done(String),
}

#[derive(Debug)]
enum RequestState {
    // Writing the request command out to the peer
    Writing {
        // The amount of bytes we've already written
        start: usize,
        // The full XML request
        buf: Vec<u8>,
    },
    // Reading the frame header (32-bit big-endian unsigned integer)
    ReadLength {
        // The amount of bytes we've already read
        read: usize,
        // The buffer we're using to read into
        buf: Vec<u8>,
    },
    // Reading the entire frame
    Reading {
        // The amount of bytes we've already read
        read: usize,
        // The buffer we're using to read into
        //
        // This will still have the frame header in it, needs to be cut off before
        // yielding the response to the caller.
        buf: Vec<u8>,
        // The expected length of the response according to the frame header
        expected: usize,
    },
}

impl RequestState {
    fn new(command: &str) -> Result<Self, Error> {
        let len = command.len();

        let buf_size = len + 4;
        let mut buf: Vec<u8> = vec![0u8; buf_size];

        let len = len + 4;
        let len_u32: [u8; 4] = u32::to_be_bytes(len.try_into()?);

        buf[..4].clone_from_slice(&len_u32);
        buf[4..].clone_from_slice(command.as_bytes());
        Ok(Self::Writing { start: 0, buf })
    }
}

pub(crate) async fn timeout<T, E: Into<Error>>(
    timeout: Duration,
    fut: impl Future<Output = Result<T, E>>,
) -> Result<T, Error> {
    match tokio::time::timeout(timeout, fut).await {
        Ok(Ok(t)) => Ok(t),
        Ok(Err(e)) => Err(e.into()),
        Err(_) => Err(Error::Timeout),
    }
}

#[async_trait]
pub trait Connector {
    type Connection: AsyncRead + AsyncWrite + Unpin;

    async fn connect(&self, timeout: Duration) -> Result<Self::Connection, Error>;
}
