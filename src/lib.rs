//! # EPP (Extensible Provisioning Protocol) client library for async Rust
//!
//! ## Description
//!
//! instant-epp is a client library written in Rust for Internet domain registration and management
//! for domain registrars. We have implemented support for the following standards:
//!
//! - [RFC 5730](https://tools.ietf.org/html/rfc5730) - Extensible Provisioning Protocol (EPP)
//! - [RFC 5731](https://tools.ietf.org/html/rfc5731) - Extensible Provisioning Protocol (EPP) Domain Name Mapping
//! - [RFC 5732](https://tools.ietf.org/html/rfc5732) - Extensible Provisioning Protocol (EPP) Host Mapping
//! - [RFC 5733](https://tools.ietf.org/html/rfc5733) - Extensible Provisioning Protocol (EPP) Contact Mapping
//! - [RFC 5734](https://tools.ietf.org/html/rfc5734) - Extensible Provisioning Protocol (EPP) Transport over TCP
//! - [RFC 3915](https://tools.ietf.org/html/rfc3915) - Domain Registry Grace Period Mapping
//! - [ConsoliDate mapping](https://www.verisign.com/assets/consolidate-mapping.txt)
//! - [Namestore Extension Mapping](https://www.verisign.com/assets/epp-sdk/verisign_epp-extension_namestoreext_v01.html)
//! - [Low Balance Mapping](https://www.verisign.com/assets/epp-sdk/verisign_epp-extension_low-balance_v01.html)
//!
//! This library is used in production with at [Instant Domains](https://instantdomains.com/).
//!
//! ## History
//!
//! instant-epp was originally created by [@masalachai](https://github.com/masalachai) as
//! [epp-client](https://github.com/masalachai/epp-client) in the summer of 2021. By fall, Instant
//! Domains employees started contributing to the project. In February of 2023, after most of the
//! contributions to epp-client had come from Instant Domains for the intervening years, we decided
//! to fork the project, replacing its dependency on quick-xml with
//! [instant-xml](https://github.com/InstantDomain/instant-xml/) in the process. Many thanks to
//! @masalachai for starting epp-client!
//!
//! ## Getting started
//!
//! You will usually want to start by initializing an [`EppClient`]. Refer to the example code
//! on that type for more information.

pub mod client;
pub mod common;
pub mod connection;
pub mod contact;
pub mod domain;
mod error;
pub mod hello;
pub mod host;
pub mod login;
pub mod logout;
pub mod request;
pub mod response;
pub mod xml;

pub mod extensions {
    pub mod consolidate;
    pub mod low_balance;
    pub mod namestore;
    pub mod rgp {
        pub mod report;
        pub mod request;

        pub const XMLNS: &str = "urn:ietf:params:xml:ns:rgp-1.0";
    }
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
