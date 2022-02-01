//! Manages registry connections and reading/writing to them

use std::convert::TryInto;
use std::future::Future;
use std::time::Duration;
use std::{io, str, u32};

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tracing::{debug, info};

use crate::error::Error;

/// EPP Connection struct with some metadata for the connection
pub(crate) struct EppConnection<IO> {
    registry: String,
    stream: IO,
    pub greeting: String,
    timeout: Duration,
}

impl<IO: AsyncRead + AsyncWrite + Unpin> EppConnection<IO> {
    pub(crate) async fn new(
        registry: String,
        stream: IO,
        timeout: Duration,
    ) -> Result<Self, Error> {
        let mut this = Self {
            registry,
            stream,
            greeting: String::new(),
            timeout,
        };

        this.greeting = this.get_epp_response().await?;
        Ok(this)
    }

    /// Constructs an EPP XML request in the required form and sends it to the server
    async fn send_epp_request(&mut self, content: &str) -> Result<(), Error> {
        let len = content.len();

        let buf_size = len + 4;
        let mut buf: Vec<u8> = vec![0u8; buf_size];

        let len = len + 4;
        let len_u32: [u8; 4] = u32::to_be_bytes(len.try_into()?);

        buf[..4].clone_from_slice(&len_u32);
        buf[4..].clone_from_slice(content.as_bytes());

        let wrote = timeout(self.timeout, self.stream.write(&buf)).await?;
        debug!("{}: Wrote {} bytes", self.registry, wrote);
        Ok(())
    }

    /// Receives response from the socket and converts it into an EPP XML string
    async fn get_epp_response(&mut self) -> Result<String, Error> {
        let mut buf = [0u8; 4];
        timeout(self.timeout, self.stream.read_exact(&mut buf)).await?;

        let buf_size: usize = u32::from_be_bytes(buf).try_into()?;

        let message_size = buf_size - 4;
        debug!("{}: Response buffer size: {}", self.registry, message_size);

        let mut buf = vec![0; message_size];
        let mut read_size: usize = 0;

        loop {
            let read = timeout(self.timeout, self.stream.read(&mut buf[read_size..])).await?;
            debug!("{}: Read: {} bytes", self.registry, read);

            read_size += read;
            debug!("{}: Total read: {} bytes", self.registry, read_size);

            if read == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    format!("{}: unexpected eof", self.registry),
                )
                .into());
            } else if read_size >= message_size {
                break;
            }
        }

        Ok(String::from_utf8(buf)?)
    }

    /// Sends an EPP XML request to the registry and return the response
    /// receieved to the request
    pub(crate) async fn transact(&mut self, content: &str) -> Result<String, Error> {
        debug!("{}: request: {}", self.registry, content);
        self.send_epp_request(content).await?;

        let response = self.get_epp_response().await?;
        debug!("{}: response: {}", self.registry, response);

        Ok(response)
    }

    /// Closes the socket and shuts the connection
    pub(crate) async fn shutdown(&mut self) -> Result<(), Error> {
        info!("{}: Closing connection", self.registry);

        timeout(self.timeout, self.stream.shutdown()).await?;
        Ok(())
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
