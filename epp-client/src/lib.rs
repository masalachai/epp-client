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
//! ## Prerequisites
//!
//! To use the library, you must have an `epp-client/epp-client.toml` config file with the relevant registry
//! credentials in your default user configuration directory on your OS. For Linux, this is the `XDG user directory`,
//! usually located at `$HOME/.config` or defined by the `XDG_CONFIG_HOME` environment variable.
//!
//! An example config looks like this:
//!
//! ```toml
//! [registry.verisign]
//! host = 'epp.verisign-grs.com'
//! port = 700
//! username = 'username'
//! password = 'password'
//! # service extensions
//! ext_uris = []
//!
//! [registry.verisign.tls_files]
//! # the full client certificate chain in PEM format
//! cert_chain = '/path/to/certificate/chain/pemfile'
//! # the RSA private key for your certificate
//! key = '/path/to/private/key/pemfile'
//! ```
//!
//! ## Operation
//!
//! Once the config is set correctly, you can create a mut variable of type [`EppClient`] to transact
//! with the domain registry
//!
//! ```rust
//! use epp_client::EppClient;
//! use epp_client::epp::{EppDomainCheck, EppDomainCheckResponse};
//! use epp_client::epp::generate_client_tr_id;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create an instance of EppClient, specifying the name of the registry as in
//!     // the config file
//!     let mut client = match EppClient::new("verisign").await {
//!         Ok(client) => client,
//!         Err(e) => panic!("Failed to create EppClient: {}",  e)
//!     };
//!
//!     // Make a domain check call, which returns an object of type EppDomainCheckResponse
//!     // that contains the result of the call
//!     let domain_check = EppDomainCheck::new(
//!         vec!["eppdev.com", "eppdev.net"],
//!         generate_client_tr_id(&client).as_str()
//!     );
//!
//!     let response = client.transact::<_, EppDomainCheckResponse>(&domain_check).await.unwrap();
//!
//!     // print the availability results
//!     response.data.res_data.unwrap().check_data.domain_list
//!         .iter()
//!         .for_each(|chk| println!("Domain: {}, Available: {}", chk.domain.name, chk.domain.available));
//! }
//! ```

#[macro_use]
extern crate log;

pub mod config;
pub mod connection;
pub mod epp;
pub mod error;
pub use connection::client::EppClient;

#[cfg(test)]
pub mod tests;
