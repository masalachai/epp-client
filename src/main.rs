pub mod config;
pub mod connection;
pub mod epp;
pub mod error;

use std::time::SystemTime;
use tokio::time::{sleep, Duration};
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
}
