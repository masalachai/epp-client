use epp_client::{epp::request::generate_client_tr_id, connection::EppClient, connection, epp::xml::EppXml, epp::response::EppGreeting};
use epp_client::epp::request::domain::EppDomainCheck;
use epp_client::epp::response::domain::EppDomainCheckResponse;
use epp_client::epp::request::contact::EppContactCheck;
use epp_client::epp::response::contact::EppContactCheckResponse;

async fn check_domains(client: &mut EppClient) {
    let domains = vec!["eppdev.com", "hexonet.net"];
    let domain_check = EppDomainCheck::new(domains, generate_client_tr_id("eppdev").unwrap().as_str());

    client.transact::<EppDomainCheck, EppDomainCheckResponse>(&domain_check).await.unwrap();
}

async fn check_contacts(client: &mut EppClient) {
    let contacts = vec!["eppdev-contact-1", "eppdev-contact-2"];
    let contact_check = EppContactCheck::new(contacts, generate_client_tr_id("eppdev").unwrap().as_str());

    client.transact::<EppContactCheck, EppContactCheckResponse>(&contact_check).await.unwrap();
}

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

    check_domains(&mut client).await;

    check_contacts(&mut client).await;
}
