use epp_client::{epp::request, connection, epp::xml::EppXml, epp::response::EppResponse};

#[tokio::main]
async fn main() {
    let mut client = match connection::connect("hexonet").await {
        Ok(client) => {
            let greeting = client.greeting();
            let greeting_object = EppResponse::deserialize(&greeting).unwrap();
            println!("{:?}", greeting_object);
            client
        },
        Err(e) => panic!("Error: {}",  e)
    };

    let epp_hello = request::Hello::new();

    client.transact(&epp_hello).await.unwrap();
}
