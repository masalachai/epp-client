use epp_client::{epp::request, epp::request::generate_client_tr_id, epp::request::domain, connection, epp::xml::EppXml, epp::response::EppGreeting, epp::object::ElementName};
use epp_client::epp::response;

#[tokio::main]
async fn main() {
    let mut client = match connection::connect("hexonet").await {
        Ok(client) => {
            let greeting = client.greeting();
            let greeting_object = EppGreeting::deserialize(&greeting).unwrap();
            println!("{:?}", greeting_object);
            client
        },
        Err(e) => panic!("Error: {}",  e)
    };

    let domains = vec!["eppdev.com", "hexonet.net"];
    let domain_check = domain::DomainCheck::epp_new(domains, generate_client_tr_id("eppdev").unwrap().as_str());

    let response = client.transact::<domain::EppDomainCheck, response::domain::EppDomainCheckResponse>(&domain_check).await.unwrap();
}
