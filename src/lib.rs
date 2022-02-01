//! # EPP (Extensible Provisioning Protocol) Client Library for Domain Registration and Management.
//!
//! ## Description
//!
//! epp-client is a client library for Internet domain registration and management for domain registrars.
//!
//! It supports the following basic Domain, Contact, Host, and Message management calls, with plans to add more calls
//! and other EPP extensions in the future, and to eventually be RFC compliant with the EPP protocol.
//!
//! - Domain Check - [`EppDomainCheck`](epp/request/domain/check/type.EppDomainCheck.html)
//! - Domain Create - [`EppDomainCreate`](epp/request/domain/create/type.EppDomainCreate.html)
//! - Domain Info - [`EppDomainInfo`](epp/request/domain/info/type.EppDomainInfo.html)
//! - Domain Update - [`EppDomainUpdate`](epp/request/domain/update/type.EppDomainUpdate.html)
//! - Domain Delete - [`EppDomainDelete`](epp/request/domain/delete/type.EppDomainDelete.html)
//! - Domain Renew - [`EppDomainRenew`](epp/request/domain/renew/type.EppDomainRenew.html)
//! - Domain Transfer - [`EppDomainTransferRequest`](epp/request/domain/transfer/type.EppDomainTransferRequest.html)
//!
//! - Contact Check - [`EppContactCheck`](epp/request/contact/check/type.EppContactCheck.html)
//! - Contact Create - [`EppContactCreate`](epp/request/contact/create/type.EppContactCreate.html)
//! - Contact Info - [`EppContactInfo`](epp/request/contact/info/type.EppContactInfo.html)
//! - Contact Update - [`EppContactUpdate`](epp/request/contact/update/type.EppContactUpdate.html)
//! - Contact Delete - [`EppContactDelete`](epp/request/contact/delete/type.EppContactDelete.html)
//!
//! - Host Check - [`EppHostCheck`](epp/request/host/check/type.EppHostCheck.html)
//! - Host Create - [`EppHostCreate`](epp/request/host/create/type.EppHostCreate.html)
//! - Host Info - [`EppHostInfo`](epp/request/host/info/type.EppHostInfo.html)
//! - Host Update - [`EppHostUpdate`](epp/request/host/update/type.EppHostUpdate.html)
//! - Host Delete - [`EppHostDelete`](epp/request/host/delete/type.EppHostDelete.html)
//!
//! - Message Poll - [`EppMessagePoll`](epp/request/message/poll/type.EppMessagePoll.html)
//! - Message Ack - [`EppMessageAck`](epp/request/message/ack/type.EppMessageAck.html)
//!
//! - RGP Restore Request - [`EppDomainRgpRestoreRequest`](epp/request/domain/rgp/request/type.EppDomainRgpRestoreRequest.html)
//! - RGP Restore Report - [`EppDomainRgpRestoreReport`](epp/request/domain/rgp/report/type.EppDomainRgpRestoreReport.html)
//!
//! ## Operation
//!
//! Typically, you will start by initializing an [`EppClient`] instance, which will connect to the EPP server.
//! From there, you can submit requests using [`EppClient::transact()`].

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
