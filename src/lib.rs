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
//! ```no_run
//! use std::net::ToSocketAddrs;
//! use std::time::Duration;
//!
//! use instant_epp::EppClient;
//! use instant_epp::domain::check::DomainCheck;
//! use instant_epp::login::Login;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create an instance of EppClient
//!     let timeout = Duration::from_secs(5);
//!     let mut client = match EppClient::connect("registry_name".to_string(), ("example.com".to_owned(), 7000), None, timeout).await {
//!         Ok(client) => client,
//!         Err(e) => panic!("Failed to create EppClient: {}",  e)
//!     };
//!
//!     let login = Login::new("username", "password", None, None);
//!     client.transact(&login, "transaction-id").await.unwrap();
//!
//!     // Execute an EPP Command against the registry with distinct request and response objects
//!     let domain_check = DomainCheck { domains: &["eppdev.com", "eppdev.net"] };
//!     let response = client.transact(&domain_check, "transaction-id").await.unwrap();
//!
//!     response.res_data()
//!         .unwrap()
//!         .list
//!         .iter()
//!         .for_each(|chk| println!("Domain: {}, Available: {}", chk.inner.id, chk.inner.available));
//! }
//! ```
//!
//! The output would look similar to the following
//!
//! ```text
//! Domain: eppdev.com, Available: 1
//! Domain: eppdev.net, Available: 1
//! ```

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
    pub mod low_balance;
    pub mod namestore;
    pub mod rgp;
}

pub mod host {
    use std::borrow::Cow;
    use std::fmt;
    use std::net::IpAddr;

    use instant_xml::{FromXml, Serializer, ToXml};

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

    /// The &lt;status&gt; type on contact transactions
    #[derive(Debug, FromXml, ToXml)]
    #[xml(rename = "status", ns(XMLNS))]
    pub struct Status<'a> {
        /// The status name, represented by the 's' attr on &lt;status&gt; tags
        #[xml(attribute, rename = "s")]
        pub status: Cow<'a, str>,
    }

    /// The &lt;hostAddr&gt; types domain or host transactions
    #[derive(Debug, FromXml, ToXml)]
    #[xml(rename = "addr", ns(XMLNS))]
    pub(crate) struct HostAddr<'a> {
        #[xml(attribute, rename = "ip")]
        pub ip_version: Option<Cow<'a, str>>,
        #[xml(direct)]
        pub address: Cow<'a, str>,
    }

    impl From<&IpAddr> for HostAddr<'static> {
        fn from(addr: &IpAddr) -> Self {
            Self {
                ip_version: Some(match addr {
                    IpAddr::V4(_) => "v4".into(),
                    IpAddr::V6(_) => "v6".into(),
                }),
                address: addr.to_string().into(),
            }
        }
    }

    pub(crate) fn serialize_host_addrs_option<T: AsRef<[IpAddr]>, W: fmt::Write + ?Sized>(
        addrs: &Option<T>,
        serializer: &mut Serializer<'_, W>,
    ) -> Result<(), instant_xml::Error> {
        let addrs = match addrs {
            Some(addrs) => addrs.as_ref(),
            None => return Ok(()),
        };

        for addr in addrs {
            HostAddr::from(addr).serialize(None, serializer)?;
        }

        Ok(())
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
