use epp_client::{epp::request, connection, epp::xml::EppXml, epp::response::EppGreeting};

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

    let epp_hello = request::Hello::new();

    client.transact::<EppGreeting>(&epp_hello).await.unwrap();
}
