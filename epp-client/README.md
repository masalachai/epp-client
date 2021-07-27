# EPP (Extensible Provisioning Protocol) Library for Domain Registration and Management

[![Build](https://ci.masalachai.net/api/badges/masalachai/epp-client/status.svg)](https://ci.masalachai.net/masalachai/epp-client)
[![Documentation](https://docs.rs/epp-client/badge.svg)](https://docs.rs/epp-client/)

## Description

epp-client is a client library written in Rust for Internet domain registration and management for domain registrars.

It supports the following basic Domain, Contact, Host, and Message management calls, with plans to add more calls
and other EPP extensions in the future, and to eventually be RFC compliant with the EPP protocol.

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

## Usage

Just add the following to your project's `Cargo.toml`

```toml
epp-client = "0.1.2"
```

## Prerequisites

To use the library, you must have an `epp-client/epp-client.toml` config file with the relevant registry
credentials in your default user configuration directory on your OS. For Linux, this is the `XDG user directory`,
usually located at `$HOME/.config` or defined by the `XDG_CONFIG_HOME` environment variable.

An example config looks like this:

```toml
[registry.verisign]
host = 'epp.verisign-grs.com'
port = 700
username = 'username'
password = 'password'
# service extensions
ext_uris = []

[registry.verisign.tls_files]
# the full client certificate chain in PEM format
cert_chain = '/path/to/certificate/chain/pemfile'
# the RSA private key for your certificate
key = '/path/to/private/key/pemfile'
```

## Operation

Once the config is set correctly, you can create a mut variable of type `EppClient`
with the domain registry

```rust
use epp_client::EppClient;
use epp_client::epp::{EppDomainCheck, EppDomainCheckResponse};
use epp_client::epp::generate_client_tr_id;

#[tokio::main]
async fn main() {
    // Create an instance of EppClient, specifying the name of the registry as in
    // the config file
    let mut client = match EppClient::new("verisign").await {
        Ok(client) => client,
        Err(e) => panic!("Failed to create EppClient: {}",  e)
    };

    // Make a domain check call, which returns an object of type EppDomainCheckResponse
    // that contains the result of the call
    let domain_check = EppDomainCheck::new(
        vec!["eppdev.com", "eppdev.net"]
        generate_client_tr_id(&client).as_str()
    );

    let response = client.transact::<_, EppDomainCheckResponse>(&domain_check).await.unwrap();

    // print the availability results
    response.data.res_data.unwrap().check_data.domain_list
        .iter()
        .for_each(|chk| println!("Domain: {}, Available: {}", chk.domain.name, chk.domain.available));
}
```

## Request

Currently I don't have access to a registry's OT&E account to do extensive testing. I am using [hexonet's EPP Gateway](https://wiki.hexonet.net/wiki/EPP_Gateway) for testing, but access to a registry's OT&E account would be very helpful, so if anyone could help me out with one I would be very grateful!