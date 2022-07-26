# EPP (Extensible Provisioning Protocol) Library for Domain Registration and Management

[![Build](https://ci.masalachai.net/api/badges/masalachai/epp-client/status.svg)](https://ci.masalachai.net/masalachai/epp-client)
[![Documentation](https://docs.rs/epp-client/badge.svg)](https://docs.rs/epp-client/)

## Description

epp-client is a client library written in Rust for Internet domain registration
and management for domain registrars.

It supports the following basic Domain, Contact, Host, and Message management
calls, with plans to add more calls and other EPP extensions in the future, and
to eventually be RFC compliant with the EPP protocol.

- Domain Check
- Domain Create
- Domain Info
- Domain Update
- Domain Delete
- Domain Renew
- Domain Transfer

- Contact Check
- Contact Create
- Contact Info
- Contact Update
- Contact Delete

- Host Check
- Host Create
- Host Info
- Host Update
- Host Delete

- Message Poll
- Message Ack

- RGP Restore Request
- RGP Restore Report

## Usage

Just add the following to your project's `Cargo.toml`

```toml
epp-client = "0.4"
```

## Operation

You can create a mut variable of type `EppClient` with the domain registry config.

```rust
use std::net::ToSocketAddrs;
use std::time::Duration;

use epp_client::EppClient;
use epp_client::domain::DomainCheck;
use epp_client::login::Login;

#[tokio::main]
async fn main() {
    // Create an instance of EppClient
    let host = "example.com";
    let addr = (host, 700).to_socket_addrs().unwrap().next().unwrap();
    let timeout = Duration::from_secs(5);
    let mut client = match EppClient::connect("registry_name".to_string(), addr, host, None, timeout).await {
        Ok(client) => client,
        Err(e) => panic!("Failed to create EppClient: {}",  e)
    };

    let login = Login::new("username", "password", None);
    client.transact(&login, "transaction-id").await.unwrap();

    // Execute an EPP Command against the registry with distinct request and response objects
    let domain_check = DomainCheck { domains: &["eppdev.com", "eppdev.net"] };
    let response = client.transact(&domain_check, "transaction-id").await.unwrap();

    response.res_data.unwrap().list
        .iter()
        .for_each(|chk| println!("Domain: {}, Available: {}", chk.id, chk.available));
}
```

The output would look like this:

```text
Domain: eppdev.com, Available: 1
Domain: eppdev.net, Available: 1
```

## Request

Currently I don't have access to a registry's OT&E account to do extensive
testing. I am using
[hexonet's EPP Gateway](https://wiki.hexonet.net/wiki/EPP_Gateway) for testing,
but access to a registry's OT&E account would be very helpful, so if anyone
could help me out with one I would be very grateful!
