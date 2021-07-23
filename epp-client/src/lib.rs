pub mod config;
pub mod connection;
pub mod epp;
pub mod error;

pub use connection::client::EppClient;

#[cfg(test)]
mod tests {
    use super::config;
    use super::connection::client::EppClient;

    #[test]
    fn config() {
        let servers = &config::CONFIG.servers;

        ()
    }

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn connect() {
        let mut client = match aw!(EppClient::new("hexonet")) {
            Ok(client) => {
                println!("{}", client.xml_greeting());
                client
            },
            Err(e) => panic!("Error: {}",  e)
        };
    }
}
