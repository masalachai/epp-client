use super::CLTRID;
use super::RESOURCES_DIR;
use crate::config::EppClientConfig;
use crate::domain;
use crate::EppClient;
use std::fs;
use std::path::Path;
use toml;

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
    let check = domain::check::Check::new(domains);

    client.transact_new(check, CLTRID).await.unwrap();

    client.logout().await.unwrap();
}
