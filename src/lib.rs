// pub mod config;
// pub mod connection;

// #[cfg(test)]
// mod tests {
//     use super::config;
//     use super::connection;
//     use std::str;

//     #[test]
//     fn config() {
//         let servers = &config::CONFIG.servers;

//         ()
//     }

//     macro_rules! aw {
//         ($e:expr) => {
//             tokio_test::block_on($e)
//         };
//     }

//     #[test]
//     fn connect() {
//         let mut cn = aw!(connection::connect("hexonet")).unwrap();
//         println!("lol");
//         let contents = aw!(cn.read()).unwrap();

//         match str::from_utf8(&contents) {
//             Ok(v) => println!("{}", v),
//             Err(e) => panic!("Error: {}", e)
//         }
//         aw!(cn.close());
//     }
// }
