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
//! Once the config is set correctly, you can create a mut variable of type [`EppClient`] to transact
//! with the domain registry
//!
//! ```no_run
//! use std::collections::HashMap;
//!
//! use epp_client::config::{EppClientConfig, RegistryConfig};
//! use epp_client::EppClient;
//! use epp_client::domain::check::DomainCheck;
//! use epp_client::common::NoExtension;
//! use epp_client::login::Login;
//! use epp_client::logout::Logout;
//!
//! #[tokio::main]
//! async fn main() {
//!
//! // Create a config
//! let mut registry: HashMap<String, RegistryConfig> = HashMap::new();
//! registry.insert(
//!     "registry_name".to_owned(),
//!     RegistryConfig {
//!         host: "example.com".to_owned(),
//!         port: 700,
//!         tls_files: None,
//!     },
//! );
//! let config = EppClientConfig { registry };
//!
//! // Create an instance of EppClient, passing the config and the registry you want to connect to
//! let mut client = match EppClient::new(&config, "registry_name").await {
//!     Ok(client) => client,
//!     Err(e) => panic!("Failed to create EppClient: {}",  e)
//! };
//!
//! let login = Login::<NoExtension>::new("username", "password", None);
//! client.transact(login, "transaction-id").await.unwrap();
//!
//! // Make a domain check call, which returns an object of type EppDomainCheckResponse
//! // that contains the result of the call
//! let domain_check = DomainCheck::<NoExtension>::new(
//!     vec!["eppdev.com", "eppdev.net"],
//! );
//!
//! let response = client.transact(domain_check, "transaction-id").await.unwrap();
//!
//! // print the availability results
//! response.res_data.unwrap().check_data.domain_list
//!     .iter()
//!     .for_each(|chk| println!("Domain: {}, Available: {}", chk.domain.name, chk.domain.available));
//!
//! // Close the connection
//! let logout = Logout::<NoExtension>::new();
//! client.transact(logout, "transaction-id").await.unwrap();
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
pub mod config;
pub mod error;
pub mod hello;
pub mod login;
pub mod logout;
pub mod registry;
pub mod request;
pub mod response;
pub mod xml;

pub mod contact {
    pub mod check;
    pub mod create;
    pub mod delete;
    pub mod info;
    pub mod update;

    pub const XMLNS: &str = "urn:ietf:params:xml:ns:contact-1.0";
}

pub mod domain {
    pub mod check;
    pub mod create;
    pub mod delete;
    pub mod info;
    pub mod renew;
    pub mod transfer;
    pub mod update;

    pub const XMLNS: &str = "urn:ietf:params:xml:ns:domain-1.0";
}

pub mod extensions {
    pub mod consolidate;
    pub mod namestore;
    pub mod rgp {
        use serde::{Deserialize, Serialize};

        pub mod report;
        pub mod request;

        pub const XMLNS: &str = "urn:ietf:params:xml:ns:rgp-1.0";

        #[derive(Debug, Deserialize, Serialize)]
        pub struct Update<T> {
            #[serde(
                rename = "rgp:update",
                alias = "update",
                alias = "upData",
                alias = "infData"
            )]
            pub data: T,
        }
    }
}

pub mod host {
    pub mod check;
    pub mod create;
    pub mod delete;
    pub mod info;
    pub mod update;

    pub const XMLNS: &str = "urn:ietf:params:xml:ns:host-1.0";
}

pub mod message {
    pub mod ack;
    pub mod poll;
}

pub use client::EppClient;

#[cfg(test)]
pub mod tests;
