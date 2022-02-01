//! # EPP (Extensible Provisioning Protocol) Client Library for Domain Registration and Management.
//!
//! ## Description
//!
//! epp-client is a client library for Internet domain registration and management for domain
//! registrars ([RFC 5730](https://tools.ietf.org/html/rfc5730)). It supports the following basic
//! management requests.
//!
//! Typically, you will start by initializing an [`EppClient`] instance, which will connect to the EPP server.
//! From there, you can submit requests using [`EppClient::transact()`].
//!
//! ## Core requests
//!
//! - [`message::MessagePoll`]
//! - [`message::MessageAck`]
//!
//! ## Domains
//!
//! Specified in [RFC 5731](https://tools.ietf.org/html/rfc5731).
//!
//! - [`domain::DomainCheck`]
//! - [`domain::DomainCreate`]
//! - [`domain::DomainInfo`]
//! - [`domain::DomainUpdate`]
//! - [`domain::DomainRenew`]
//! - [`domain::DomainTransfer`]
//! - [`domain::DomainDelete`]
//!
//! ## Contacts
//!
//! Specified in [RFC 5732](https://tools.ietf.org/html/rfc5732).
//!
//! - [`contact::ContactCheck`]
//! - [`contact::ContactCreate`]
//! - [`contact::ContactInfo`]
//! - [`contact::ContactUpdate`]
//! - [`contact::ContactDelete`]
//!
//! ## Hosts
//!
//! Specified in [RFC 5733](https://tools.ietf.org/html/rfc5733).
//!
//! - [`host::HostCheck`]
//! - [`host::HostCreate`]
//! - [`host::HostInfo`]
//! - [`host::HostUpdate`]
//! - [`host::HostDelete`]
//!
//! ## Extensions
//!
//! - [`extensions::rgp::report::RgpRestoreReport`]
//! - [`extensions::rgp::request::RgpRestoreRequest`]
//! - [`extensions::namestore::NameStore`]
//! - [`extensions::consolidate::Update`]
//!
//! ## Operation
//!

pub mod client;
pub mod common;
pub mod connection;
pub mod contact;
pub mod domain;
mod error;
pub mod hello;
pub mod login;
pub mod logout;
pub mod request;
pub mod response;
pub mod xml;

pub mod extensions {
    pub mod consolidate;
    pub mod namestore;
    pub mod rgp;
}

pub mod host {
    pub mod check;
    pub use check::HostCheck;

    pub mod create;
    pub use create::HostCreate;

    pub mod delete;
    pub use delete::HostDelete;

    pub mod info;
    pub use info::HostInfo;

    pub mod update;
    pub use update::HostUpdate;

    pub const XMLNS: &str = "urn:ietf:params:xml:ns:host-1.0";
}

pub mod message {
    pub mod ack;
    pub use ack::MessageAck;

    pub mod poll;
    pub use poll::MessagePoll;
}

pub use client::EppClient;
pub use error::Error;

#[cfg(test)]
pub mod tests;
