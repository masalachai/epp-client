use futures::executor::block_on;
use std::{error::Error, fmt::Debug};
use std::sync::mpsc;

use crate::config::CONFIG;
use crate::connection::registry::{epp_connect, EppConnection};
use crate::error;
use crate::epp::request::{generate_client_tr_id, EppHello, EppLogin, EppLogout};
use crate::epp::response::{EppGreeting, EppCommandResponseStatus, EppCommandResponse, EppCommandResponseError};
use crate::epp::xml::EppXml;
use crate::epp::object::{ElementName, EppObject};

async fn connect(registry: &'static str) -> Result<EppClient, Box<dyn Error>> {
    let registry_creds = match CONFIG.registry(registry) {
        Some(creds) => creds,
        None => return Err(format!("missing credentials for {}", registry).into())
    };

    let (tx, rx) = mpsc::channel();

    tokio::spawn(async move {
        let stream = epp_connect(&registry_creds).await.unwrap();
        let credentials = registry_creds.credentials();

        let connection = EppConnection::new(
            registry.to_string(),
            stream
        ).await.unwrap();

        let client = EppClient::build(connection, credentials).await.unwrap();

        tx.send(client).unwrap();
    });

    let client = rx.recv()?;

    Ok(client)
}

pub struct EppClient {
    credentials: (String, String),
    connection: EppConnection,
}

impl EppClient {
    pub async fn new(registry: &'static str) -> Result<EppClient, Box<dyn Error>> {
        connect(registry).await
    }

    async fn build(connection: EppConnection, credentials: (String, String)) -> Result<EppClient, Box<dyn Error>> {
        let mut client = EppClient {
            connection: connection,
            credentials: credentials
        };

        let client_tr_id = generate_client_tr_id(&client.credentials.0)?;
        let login_request = EppLogin::new(&client.credentials.0, &client.credentials.1, client_tr_id.as_str());

        client.transact::<EppLogin, EppCommandResponse>(&login_request).await?;

        Ok(client)
    }

    pub async fn hello(&mut self) -> Result<EppGreeting, Box<dyn Error>> {
        let hello = EppHello::new();
        let hello_xml = hello.serialize()?;

        let response = self.connection.transact(&hello_xml).await?;

        println!("hello response: {}", response);

        Ok(EppGreeting::deserialize(&response)?)
    }

    pub async fn transact<T: EppXml + Debug, E: EppXml + Debug>(&mut self, request: &T) -> Result<E::Output, error::Error> {
        let epp_xml = request.serialize()?;

        println!("Request:\r\n{}", epp_xml);

        let response = self.connection.transact(&epp_xml).await?;

        println!("Response:\r\n{}", response);

        let status = EppCommandResponseStatus::deserialize(&response)?;

        if status.data.result.code < 2000 {
            let response = E::deserialize(&response)?;
            println!("Response:\r\n{:?}", response);
            Ok(response)
        } else {
            let epp_error = EppCommandResponseError::deserialize(&response)?;
            Err(error::Error::EppCommandError(epp_error))
        }
    }

    pub async fn transact_xml(&mut self, xml: &str) -> Result<String, Box<dyn Error>> {
        self.connection.transact(&xml).await
    }

    pub fn xml_greeting(&self) -> String {
        return String::from(&self.connection.greeting)
    }

    pub fn greeting(&self) -> Result<EppGreeting, Box<dyn Error>> {
        Ok(EppGreeting::deserialize(&self.connection.greeting)?)
    }

    pub async fn logout(&mut self) {
        let client_tr_id = generate_client_tr_id(&self.credentials.0).unwrap();
        let epp_logout = EppLogout::new(client_tr_id.as_str());

        self.transact::<EppLogout, EppCommandResponse>(&epp_logout).await;
    }
}

impl Drop for EppClient {
    fn drop(&mut self) {
        block_on(self.logout());
    }
}
