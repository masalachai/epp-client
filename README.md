# EPP client library for async Rust

[![Documentation](https://docs.rs/instant-epp/badge.svg)](https://docs.rs/instant-epp)
[![Crates.io](https://img.shields.io/crates/v/instant-epp.svg)](https://crates.io/crates/instant-epp)
[![Build status](https://github.com/InstantDomain/instant-epp/workflows/CI/badge.svg)](https://github.com/InstantDomain/instant-epp/actions?query=workflow%3ACI)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)

## Description

instant-epp is a client library written in Rust for Internet domain registration and management
for domain registrars. We have implemented support for the following standards:

- [RFC 5730](https://tools.ietf.org/html/rfc5730) - Extensible Provisioning Protocol (EPP)
- [RFC 5731](https://tools.ietf.org/html/rfc5731) - Extensible Provisioning Protocol (EPP) Domain Name Mapping
- [RFC 5732](https://tools.ietf.org/html/rfc5732) - Extensible Provisioning Protocol (EPP) Host Mapping
- [RFC 5733](https://tools.ietf.org/html/rfc5733) - Extensible Provisioning Protocol (EPP) Contact Mapping
- [RFC 5734](https://tools.ietf.org/html/rfc5734) - Extensible Provisioning Protocol (EPP) Transport over TCP
- [RFC 3915](https://tools.ietf.org/html/rfc3915) - Domain Registry Grace Period Mapping
- [ConsoliDate mapping](https://www.verisign.com/assets/consolidate-mapping.txt)
- [Namestore Extension Mapping](https://www.verisign.com/assets/epp-sdk/verisign_epp-extension_namestoreext_v01.html)
- [Low Balance Mapping](https://www.verisign.com/assets/epp-sdk/verisign_epp-extension_low-balance_v01.html)

This library is used in production with at [Instant Domains](https://instantdomains.com/).

## History

instant-epp was originally created by [@masalachai](https://github.com/masalachai) as
[epp-client](https://github.com/masalachai/epp-client) in the summer of 2021. By fall, Instant
Domains employees started contributing to the project. In February of 2023, after most of the
contributions to epp-client had come from Instant Domains for the intervening years, we decided
to fork the project, replacing its dependency on quick-xml with
[instant-xml](https://github.com/InstantDomain/instant-xml/) in the process. Many thanks to
@masalachai for starting epp-client!

## Getting started

You can create a mut variable of type `EppClient` with the domain registry config.

```rust
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::time::Duration;

use instant_epp::EppClient;
use instant_epp::domain::DomainCheck;
use instant_epp::common::NoExtension;

#[tokio::main]
async fn main() {
    // Create an instance of EppClient
    let timeout = Duration::from_secs(5);
    let mut client = match EppClient::connect("registry_name".to_string(), ("example.com".to_owned(), 7000), None, timeout).await {
        Ok(client) => client,
        Err(e) => panic!("Failed to create EppClient: {}",  e)
    };

    // Make a EPP Hello call to the registry
    let greeting = client.hello().await.unwrap();
    println!("{:?}", greeting);

    // Execute an EPP Command against the registry with distinct request and response objects
    let domain_check = DomainCheck { domains: &["eppdev.com", "eppdev.net"] };
    let response = client.transact(&domain_check, "transaction-id").await.unwrap();
    response
        .res_data()
        .unwrap()
        .list
        .iter()
        .for_each(|chk| println!("Domain: {}, Available: {}", chk.inner.id, chk.inner.available));
}
```

The output would look like this:

```
Domain: eppdev.com, Available: 1
Domain: eppdev.net, Available: 1
```
