use std::fs;
use std::path::Path;

use crate::config::EppClientConfig;
use crate::domain::check::DomainCheck;
use crate::epp::object::NoExtension;
use crate::epp::{RgpRestoreRequest, RgpRestoreRequestData};
use crate::extensions::command_response::namestore::NameStore;
use crate::EppClient;

const RESOURCES_DIR: &str = "./test/resources";
const CLTRID: &str = "cltrid:1626454866";

#[tokio::test]
#[ignore]
async fn domain_check() {
    let config_file = format!("{}/{}", RESOURCES_DIR, "secrets/epp-client.toml");
    let config_path = Path::new(config_file.as_str());
    let contents = &fs::read_to_string(config_path).unwrap();

    let config: EppClientConfig = toml::from_str(contents).unwrap();

    let mut client = match EppClient::new(&config, "testing").await {
        Ok(client) => client,
        Err(e) => panic!("Failed to create EppClient: {}", e),
    };

    let domains = vec!["eppdev.com", "eppdev.net"];

    let namestore_ext = NameStore::new("com");

    let request = DomainCheck::<NameStore>::new(domains).with_extension(namestore_ext);

    let response = client.transact_new(request, CLTRID).await.unwrap();

    println!("{:?}", response);

    client.logout().await.unwrap();
}
