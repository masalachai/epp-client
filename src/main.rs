pub mod config;
pub mod connection;
pub mod epp;
pub mod error;

use std::time::SystemTime;
use crate::{epp::request};

#[tokio::main]
async fn main() {
    let mut client = match connection::connect("hexonet").await {
        Ok(client) => {
            println!("{}", client.greeting());
            client
        },
        Err(e) => panic!("Error: {}",  e)
    };

    let epp_hello = request::Hello::new();

    client.transact(&epp_hello).await.unwrap();

    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let cl_trid = format!("eppdev:{}", timestamp.as_secs());
    let epp_login = request::Login::new("eppdev", "sh48sja#27*A", &cl_trid);

    // let response = epp_login.to_epp_xml().unwrap();

    client.transact(&epp_login).await.unwrap();

    //let response = client.transact(&epp_hello).await.unwrap();

    //println!("{}", response);
}
