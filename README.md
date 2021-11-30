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
epp-client = "0.3"
```

## Operation

You can create a mut variable of type `EppClient` with the domain registry config.

```rust
use std::collections::HashMap;

use epp_client::config::{EppClientConfig, RegistryConfig};
use epp_client::EppClient;
use epp_client::domain::check::DomainCheck;
use epp_client::common::NoExtension;
use epp_client::login::Login;
use epp_client::logout::Logout;

#[tokio::main]
async fn main() {
    // Configure the client to connect to one of more registries
    let mut registry: HashMap<String, RegistryConfig> = HashMap::new();
    registry.insert(
        "registry_name".to_owned(),
        RegistryConfig {
            host: "example.com".to_owned(),
            port: 700,
            tls_files: None,
        },
    );
    let config = EppClientConfig { registry };

    // Create an instance of EppClient, passing the config and the registry you want to connect to
    let mut client = match EppClient::new(&config, "registry_name").await {
        Ok(client) => client,
        Err(e) => panic!("Failed to create EppClient: {}",  e)
    };

    let login = Login::<NoExtension>::new("username", "password", &None);
    client.transact(login, "transaction-id").await.unwrap();

    // Create an DomainCheck instance
    let domain_check = DomainCheck::<NoExtension>::new(
        vec!["eppdev-100.com", "eppdev-100.net"],
    );

    // send it to the registry and receive a response of type EppDomainCheckResponse
    let response = client.transact(domain_check, "transaction-id").await.unwrap();

    println!("{:?}", response);

    let logout = Logout::<NoExtension>::new();
    client.transact(logout, "transaction-id").await.unwrap();
}
```

The output would look similar to the following:

```
Domain: eppdev.com, Available: 1
Domain: eppdev.net, Available: 1
```

You may also choose to store your configuration in something like a toml file:

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

```rust
use epp_client::config::{EppClientConfig};

#[tokio::main]
async fn main() {
    // parse EppClientConfig from toml file
    let config_path = Path::new("../secrets/epp-client.toml");
    let config: EppClientConfig =
        toml::from_str(&fs::read_to_string(config_path).await.unwrap()).unwrap();
}
```

## Request

Currently I don't have access to a registry's OT&E account to do extensive
testing. I am using
[hexonet's EPP Gateway](https://wiki.hexonet.net/wiki/EPP_Gateway) for testing,
but access to a registry's OT&E account would be very helpful, so if anyone
could help me out with one I would be very grateful!
