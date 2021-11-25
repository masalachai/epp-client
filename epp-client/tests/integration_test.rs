use epp_client::config::EppClientConfig;
use epp_client::domain;
use epp_client::epp::object::NoExtension;
use epp_client::EppClient;
use std::fs;
use std::path::Path;

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
    let request = domain::check::Request::<NoExtension>::new(domains, None, CLTRID);

    let response = client.transact_new(request).await.unwrap();

    println!("{:?}", response);

    client.logout().await.unwrap();
}
