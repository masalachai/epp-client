use futures::executor::block_on;
use std::{error::Error, fmt::Debug};
// use std::time::SystemTime;
use std::sync::mpsc;
// use std::sync::Arc;

use crate::config::CONFIG;
use crate::connection::registry::{epp_connect, EppConnection};
use crate::error;
use crate::epp::request::{generate_client_tr_id, EppHello, EppLogin, EppLogout};
use crate::epp::response::{EppGreeting, EppCommandResponseStatus, EppCommandResponse, EppCommandResponseError};
use crate::epp::xml::EppXml;

async fn connect(registry: &'static str) -> Result<EppClient, Box<dyn Error>> {
    let registry_creds = match CONFIG.registry(registry) {
        Some(creds) => creds,
        None => return Err(format!("missing credentials for {}", registry).into())
    };

    let (tx, rx) = mpsc::channel();

    tokio::spawn(async move {
        let stream = epp_connect(&registry_creds).await.unwrap();
        let credentials = registry_creds.credentials();
        let ext_uris = registry_creds.ext_uris();

        let ext_uris = match ext_uris {
            Some(uris) => Some(
                uris
                    .iter()
                    .map(|u| u.to_string())
                    .collect::<Vec<String>>()
            ),
            None => None,
        };

        let connection = EppConnection::new(
            registry.to_string(),
            stream
        ).await.unwrap();

        let client = EppClient::build(connection, credentials, ext_uris).await.unwrap();

        tx.send(client).unwrap();
    });

    let client = rx.recv()?;

    Ok(client)
}

pub struct EppClient {
    credentials: (String, String),
    ext_uris: Option<Vec<String>>,
    connection: EppConnection,
    // pub client_tr_id_fn: Arc<dyn Fn(&EppClient) -> String + Send + Sync>,
}

// fn default_client_tr_id_fn(client: &EppClient) -> String {
//     let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
//         Ok(time) => time,
//         Err(e) => panic!("Error in client TRID gen function: {}", e)
//     };
//     format!("{}:{}", &client.username(), timestamp.as_secs())
// }

impl EppClient {
    pub fn username(&self) -> String {
        self.credentials.0.to_string()
    }

    // pub fn set_client_tr_id_fn<F>(&mut self, func: F)
    // where F: Fn(&EppClient) -> String + Send + Sync + 'static {
    //     self.client_tr_id_fn = Arc::new(func);
    // }

    pub async fn new(registry: &'static str) -> Result<EppClient, Box<dyn Error>> {
        connect(registry).await
    }

    async fn build(connection: EppConnection, credentials: (String, String), ext_uris: Option<Vec<String>>) -> Result<EppClient, Box<dyn Error>> {
        let mut client = EppClient {
            connection: connection,
            credentials: credentials,
            ext_uris: ext_uris,
            // client_tr_id_fn: Arc::new(default_client_tr_id_fn),
        };

        let client_tr_id = generate_client_tr_id(&client.credentials.0)?;
        let login_request = EppLogin::new(&client.credentials.0, &client.credentials.1, &client.ext_uris, client_tr_id.as_str());

        client.transact::<_, EppCommandResponse>(&login_request).await?;

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

        self.transact::<_, EppCommandResponse>(&epp_logout).await;
    }
}

impl Drop for EppClient {
    fn drop(&mut self) {
        block_on(self.logout());
    }
}
