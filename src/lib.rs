pub mod config;
pub mod connection;
pub mod epp;
pub mod error;

#[cfg(test)]
mod tests {
    use super::config;
    use super::connection;

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
        let mut client = match aw!(connection::connect("hexonet")) {
            Ok(client) => {
                println!("{}", client.greeting());
                client
            },
            Err(e) => panic!("Error: {}",  e)
        };
    }
}
