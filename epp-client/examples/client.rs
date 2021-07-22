use epp_client::{epp::request::generate_client_tr_id, connection::client::EppClient, connection, epp::xml::EppXml};
use epp_client::epp::object::data::ContactStatus;
use epp_client::epp::request::domain::check::EppDomainCheck;
use epp_client::epp::response::domain::check::EppDomainCheckResponse;
use epp_client::epp::request::contact::check::EppContactCheck;
use epp_client::epp::response::contact::check::EppContactCheckResponse;
use epp_client::epp::object::data::{PostalInfo, Address, Phone};
use epp_client::epp::request::contact::create::EppContactCreate;
use epp_client::epp::response::contact::create::EppContactCreateResponse;
use epp_client::epp::request::contact::info::EppContactInfo;
use epp_client::epp::response::contact::info::EppContactInfoResponse;
use epp_client::epp::request::contact::update::EppContactUpdate;
use epp_client::epp::response::contact::update::EppContactUpdateResponse;
use epp_client::epp::request::contact::delete::EppContactDelete;
use epp_client::epp::response::contact::delete::EppContactDeleteResponse;

async fn check_domains(client: &mut EppClient) {
    let domains = vec!["eppdev.com", "hexonet.net"];
    let domain_check = EppDomainCheck::new(domains, generate_client_tr_id("eppdev").unwrap().as_str());

    client.transact::<EppDomainCheck, EppDomainCheckResponse>(&domain_check).await.unwrap();
}

async fn check_contacts(client: &mut EppClient) {
    let contacts = vec!["eppdev-contact-1", "eppdev-contact-2"];
    let contact_check = EppContactCheck::new(contacts, generate_client_tr_id("eppdev").unwrap().as_str());

    client.transact::<_, EppContactCheckResponse>(&contact_check).await.unwrap();
}

async fn create_contact(client: &mut EppClient) {
    let street = vec!["58", "Orchid Road"];
    let address = Address::new(street, "Paris", "Paris", "392374", "FR");
    let postal_info = PostalInfo::new("int", "John Doe", "Acme Widgets", address);
    let mut voice = Phone::new("+47.47237942");
    voice.set_extension("123");
    let mut fax = Phone::new("+47.86698799");
    fax.set_extension("677");

    let mut contact_create = EppContactCreate::new("eppdev-contact-1", "contact@eppdev.net", postal_info, voice, "eppdev-387323", generate_client_tr_id("eppdev").unwrap().as_str());
    contact_create.set_fax(fax);

    // println!("xml: {}", contact_create.serialize().unwrap());

    client.transact::<_, EppContactCreateResponse>(&contact_create).await.unwrap();
}

async fn update_contact(client: &mut EppClient) {
    let contact_info = EppContactInfo::new("eppdev-contact-1", "eppdev-387323", generate_client_tr_id("eppdev").unwrap().as_str());
    let contact_info_response = client.transact::<_, EppContactInfoResponse>(&contact_info).await.unwrap();

    let mut contact_update = EppContactUpdate::new("eppdev-contact-1", generate_client_tr_id("eppdev").unwrap().as_str());
    let contact_info_res_data = contact_info_response.data.res_data.unwrap();
    contact_update.set_info("newemail@eppdev.net", contact_info_res_data.info_data.postal_info, contact_info_res_data.info_data.voice, contact_info_res_data.info_data.auth_info.password.to_string().as_str());
    let add_statuses = vec![ContactStatus { status: "clientTransferProhibited".to_string() }];
    contact_update.remove_statuses(add_statuses);

    // println!("{}", contact_update.serialize().unwrap());

    client.transact::<_, EppContactUpdateResponse>(&contact_update).await.unwrap();
}

async fn query_contact(client: &mut EppClient) {
    let mut contact_info = EppContactInfo::new("eppdev-contact-1", "eppdev-387323", generate_client_tr_id("eppdev").unwrap().as_str());

    client.transact::<_, EppContactInfoResponse>(&contact_info).await.unwrap();
}

async fn delete_contact(client: &mut EppClient) {
    let contact_delete = EppContactDelete::new("eppdev-contact-1", generate_client_tr_id("eppdev").unwrap().as_str());

    client.transact::<_, EppContactDeleteResponse>(&contact_delete).await.unwrap();
}

async fn hello(client: &mut EppClient) {
    let greeting = client.hello().await.unwrap();

    println!("{:?}", greeting);
}

#[tokio::main]
async fn main() {
    let mut client = match EppClient::new("hexonet").await {
        Ok(client) => {
            println!("{:?}", client.greeting());
            client
        },
        Err(e) => panic!("Error: {}",  e)
    };

    // hello(&mut client).await;

    // check_domains(&mut client).await;

    // check_contacts(&mut client).await;

    // create_contact(&mut client).await;

    // query_contact(&mut client).await;

    // update_contact(&mut client).await;

    // delete_contact(&mut client).await;
}
