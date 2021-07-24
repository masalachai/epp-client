use std::{error::Error, time::SystemTime};
use chrono::NaiveDate;

use epp_client::EppClient;
use epp_client::epp::object::{StringValueTrait};
use epp_client::{epp::request, epp::xml::EppXml};
use epp_client::epp::object::data::{
    PostalInfo, Address, Phone, DomainContact, ContactStatus, DomainStatus, HostObjList, HostAttrList, HostAttr, HostAddr,
    Host, HostStatus
};
use epp_client::epp::*;

fn gen_client_tr_id(username: &str) -> Result<String, Box<dyn Error>> {
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    Ok(format!("{}:{}", username, timestamp.as_secs()))
}

async fn check_domains(client: &mut EppClient) {
    let domains = vec!["eppdev.com", "hexonet.net"];
    let domain_check = EppDomainCheck::new(domains, gen_client_tr_id("eppdev").unwrap().as_str());

    client.transact::<_, EppDomainCheckResponse>(&domain_check).await.unwrap();
}

async fn check_contacts(client: &mut EppClient) {
    let contacts = vec!["eppdev-contact-1", "eppdev-contact-2"];
    let contact_check = EppContactCheck::new(contacts, gen_client_tr_id("eppdev").unwrap().as_str());

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

    let mut contact_create = EppContactCreate::new("eppdev-contact-3", "contact@eppdev.net", postal_info, voice, "eppdev-387323", gen_client_tr_id("eppdev").unwrap().as_str());
    contact_create.set_fax(fax);

    // println!("xml: {}", contact_create.serialize().unwrap());

    client.transact::<_, EppContactCreateResponse>(&contact_create).await.unwrap();
}

async fn update_contact(client: &mut EppClient) {
    let contact_info = EppContactInfo::new("eppdev-contact-1", "eppdev-387323", gen_client_tr_id("eppdev").unwrap().as_str());
    let contact_info_response = client.transact::<_, EppContactInfoResponse>(&contact_info).await.unwrap();

    let mut contact_update = EppContactUpdate::new("eppdev-contact-1", gen_client_tr_id("eppdev").unwrap().as_str());
    let contact_info_res_data = contact_info_response.data.res_data.unwrap();
    contact_update.set_info("newemail@eppdev.net", contact_info_res_data.info_data.postal_info, contact_info_res_data.info_data.voice, "eppdev-387323");
    let add_statuses = vec![ContactStatus { status: "clientTransferProhibited".to_string() }];
    contact_update.remove_statuses(add_statuses);

    // println!("{}", contact_update.serialize().unwrap());

    client.transact::<_, EppContactUpdateResponse>(&contact_update).await.unwrap();
}

async fn query_contact(client: &mut EppClient) {
    let mut contact_info = EppContactInfo::new("eppdev-contact-2", "eppdev-387323", gen_client_tr_id("eppdev").unwrap().as_str());

    client.transact::<_, EppContactInfoResponse>(&contact_info).await.unwrap();
}

async fn delete_contact(client: &mut EppClient) {
    let contact_delete = EppContactDelete::new("eppdev-contact-1", gen_client_tr_id("eppdev").unwrap().as_str());

    client.transact::<_, EppContactDeleteResponse>(&contact_delete).await.unwrap();
}

async fn create_domain(client: &mut EppClient) {
    let contacts = vec![
        DomainContact {
            contact_type: "admin".to_string(),
            id: "eppdev-contact-2".to_string()
        },
        DomainContact {
            contact_type: "tech".to_string(),
            id: "eppdev-contact-2".to_string()
        },
        DomainContact {
            contact_type: "billing".to_string(),
            id: "eppdev-contact-2".to_string()
        }
    ];
    // let domain_create = EppDomainCreate::new_with_ns("eppdev.com", 1, vec!["ns1.test.com", "ns2.test.com"], "eppdev-contact-1", "eppdevauth123", contacts, gen_client_tr_id("eppdev").unwrap().as_str());

    let domain_create = EppDomainCreate::new("eppdev-1.com", 1, "eppdev-contact-2", "epP4uthd#v", contacts, gen_client_tr_id("eppdev").unwrap().as_str());

    // println!("{}", domain_create.serialize().unwrap());

    client.transact::<_, EppDomainCreateResponse>(&domain_create).await.unwrap();
}

async fn query_domain(client: &mut EppClient) {
    let domain_info = EppDomainInfo::new("eppdev-1.com", gen_client_tr_id("eppdev").unwrap().as_str());

    client.transact::<_, EppDomainInfoResponse>(&domain_info).await.unwrap();
}

async fn update_domain(client: &mut EppClient) {
    let mut domain_update = EppDomainUpdate::new("eppdev.com", gen_client_tr_id("eppdev").unwrap().as_str());

    let remove = DomainAddRemove {
        ns: None,
        // ns: Some(HostAttrList {
        //     hosts: vec![
        //         HostAttr {
        //             name: "ns1.test.com".to_string_value(),
        //             addresses: Some(vec![
        //                 HostAddr::new_v4("177.163.23.23"),
        //                 HostAddr::new_v6("213.221.54.17"),
        //             ]
        //         )},
        //         HostAttr {
        //             name: "ns2.test.com".to_string_value(),
        //             addresses: None,
        //         },
        //     ]
        // }),
        contacts: None,
        statuses: Some(vec![
            DomainStatus {
                status: "clientDeleteProhibited".to_string()
            }
        ])
    };

    let add = DomainAddRemove {
        ns: None,
        contacts: Some(vec![
            DomainContact {
                contact_type: "billing".to_string(),
                id: "eppdev-contact-2".to_string()
            }
        ]),
        statuses: None,
    };

    domain_update.add(add);
    domain_update.remove(remove);

    // println!("{}", domain_update.serialize().unwrap());

    client.transact::<_, EppDomainUpdateResponse>(&domain_update).await.unwrap();
}

async fn delete_domain(client: &mut EppClient) {
    let domain_delete = EppDomainDelete::new("eppdev.com", gen_client_tr_id("eppdev").unwrap().as_str());

    client.transact::<_, EppDomainDeleteResponse>(&domain_delete).await.unwrap();
}

async fn renew_domain(client: &mut EppClient) {
    let exp_date = NaiveDate::from_ymd(2022, 7, 23);

    let renew_domain = EppDomainRenew::new("eppdev-1.com", exp_date, 1, gen_client_tr_id("eppdev").unwrap().as_str());

    client.transact::<_, EppDomainRenewResponse>(&renew_domain).await.unwrap();
}

async fn request_transfer(client: &mut EppClient) {
    let transfer_request = EppDomainTransferRequest::request("testing.com", 1, "epP4uthd#v", gen_client_tr_id("eppdev").unwrap().as_str());

    println!("{}\n\n", transfer_request.serialize().unwrap());
}

async fn approve_transfer(client: &mut EppClient) {
    let transfer_approve = EppDomainTransferRequest::approve("testing.com", gen_client_tr_id("eppdev").unwrap().as_str());

    println!("{}\n\n", transfer_approve.serialize().unwrap());
}

async fn reject_transfer(client: &mut EppClient) {
    let transfer_reject = EppDomainTransferRequest::reject("testing.com", gen_client_tr_id("eppdev").unwrap().as_str());

    println!("{}\n\n", transfer_reject.serialize().unwrap());
}

async fn cancel_transfer(client: &mut EppClient) {
    let transfer_cancel = EppDomainTransferRequest::cancel("testing.com", gen_client_tr_id("eppdev").unwrap().as_str());

    println!("{}\n\n", transfer_cancel.serialize().unwrap());
}

async fn query_transfer(client: &mut EppClient) {
    let transfer_query = EppDomainTransferRequest::query("testing.com", "epP4uthd#v", gen_client_tr_id("eppdev").unwrap().as_str());

    println!("{}\n\n", transfer_query.serialize().unwrap());
}

async fn check_hosts(client: &mut EppClient) {
    let hosts_check = EppHostCheck::new(vec!["host1.eppdev-1.com", "ns1.testing.com"], gen_client_tr_id("eppdev").unwrap().as_str());

    client.transact::<_, EppHostCheckResponse>(&hosts_check).await.unwrap();
}

async fn create_host(client: &mut EppClient) {
    let host = Host {
        name: "host1.eppdev-1.com".to_string_value(),
        addresses: Some(vec![
            HostAddr::new("v4", "29.245.122.14"),
            HostAddr::new("v6", "2400:6180:100:d0::8d6:4001"),
        ])
    };

    let host_create = EppHostCreate::new(host, gen_client_tr_id("eppdev").unwrap().as_str());

    // println!("{}", host_create.serialize().unwrap());

    client.transact::<_, EppHostCreateResponse>(&host_create).await.unwrap();
}

async fn query_host(client: &mut EppClient) {
    let host_info = EppHostInfo::new("host2.eppdev-1.com", gen_client_tr_id("eppdev").unwrap().as_str());

    // println!("{}", host_info.serialize().unwrap());

    client.transact::<_, EppHostInfoResponse>(&host_info).await.unwrap();
}

async fn update_host(client: &mut EppClient) {
    let addr = vec![
        HostAddr::new("v6", "2400:6180:100:d0::8d6:4001"),
    ];

    let add = HostAddRemove {
        addresses: Some(addr),
        statuses: None,
    };

    let remove = HostAddRemove {
        addresses: None,
        statuses: Some(vec![
            HostStatus {
                status: "clientDeleteProhibited".to_string()
            }
        ]),
    };

    let mut host_update = EppHostUpdate::new("host1.eppdev-1.com", gen_client_tr_id("eppdev").unwrap().as_str());

    host_update.add(add);
    // host_update.remove(remove);
    host_update.info(HostChangeInfo { name: "host2.eppdev-1.com".to_string_value() });

    // println!("{}", host_update.serialize().unwrap());

    client.transact::<_, EppHostUpdateResponse>(&host_update).await.unwrap();
}

async fn delete_host(client: &mut EppClient) {
    let host_delete = EppHostDelete::new("host2.eppdev-1.com", gen_client_tr_id("eppdev").unwrap().as_str());

    client.transact::<_, EppHostDeleteResponse>(&host_delete).await.unwrap();
}

async fn poll_message(client: &mut EppClient) {
    let message_poll = EppMessagePoll::new(gen_client_tr_id("eppdev").unwrap().as_str());

    // println!("{}", message_poll.serialize().unwrap());

    client.transact::<_, EppMessagePollResponse>(&message_poll).await.unwrap();
}

async fn ack_message(client: &mut EppClient) {
    let message_ack = EppMessageAck::new(12345, gen_client_tr_id("eppdev").unwrap().as_str());

    println!("{}", message_ack.serialize().unwrap());

    // client.transact::<_, EppMessageAckResponse>(&message_ack).await.unwrap();
}

async fn hello(client: &mut EppClient) {
    let greeting = client.hello().await.unwrap();

    println!("{:?}", greeting);
}

#[tokio::main]
async fn main() {
    let mut client = match EppClient::new("verisign").await {
        Ok(client) => {
            println!("{:?}", client.greeting());
            client
        },
        Err(e) => panic!("Error: {}",  e)
    };

    // client.set_client_tr_id_fn(gen_client_tr_id);

    // hello(&mut client).await;

    let response = check_domains(&mut client).await;

    // check_contacts(&mut client).await;

    // create_contact(&mut client).await;

    // query_contact(&mut client).await;

    // update_contact(&mut client).await;

    // delete_contact(&mut client).await;

    // create_domain(&mut client).await;

    // query_domain(&mut client).await;

    // update_domain(&mut client).await;

    // delete_domain(&mut client).await;

    // renew_domain(&mut client).await;

    // request_transfer(&mut client).await;

    // query_transfer(&mut client).await;

    // approve_transfer(&mut client).await;

    // reject_transfer(&mut client).await;

    // cancel_transfer(&mut client).await;

    // check_hosts(&mut client).await;

    // create_host(&mut client).await;

    // query_host(&mut client).await;

    // update_host(&mut client).await;

    // delete_host(&mut client).await;

    // poll_message(&mut client).await;

    // ack_message(&mut client).await;
}
