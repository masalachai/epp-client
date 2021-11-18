//! Types for EPP NameStore domain renew

use chrono::NaiveDate;

use crate::epp::object::data::Period;
use crate::epp::object::{EppObject, StringValueTrait};
use crate::epp::request::domain::renew::{DomainRenew, DomainRenewData};
use crate::epp::request::{CommandWithExtension, Extension};
use crate::epp::xml::{EPP_DOMAIN_NAMESTORE_EXT_XMLNS, EPP_DOMAIN_XMLNS};

use super::object::NameStore;

/// Type that represents the &lt;epp&gt; request for domain &lt;renew&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use chrono::NaiveDate;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::{EppNamestoreDomainRenew, EppNamestoreDomainRenewResponse};
/// use epp_client::epp::generate_client_tr_id;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, EppClientConnection> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         EppClientConnection {
///             host: "example.com".to_owned(),
///             port: 700,
///             username: "username".to_owned(),
///             password: "password".to_owned(),
///             ext_uris: None,
///             tls_files: None,
///         },
///     );
///     let config = EppClientConfig { registry };
///
///     // Create an instance of EppClient, passing the config and the registry you want to connect to
///     let mut client = match EppClient::new(&config, "registry_name").await {
///         Ok(client) => client,
///         Err(e) => panic!("Failed to create EppClient: {}",  e)
///     };
///
///     // Create a date object to set the current expiry date
///     let exp_date = NaiveDate::from_ymd(2022, 7, 27);
///
///     // Create an EppNamestoreDomainRenew instance
///     let domain_renew = EppNamestoreDomainRenew::new("eppdev-100.com", exp_date, 1, generate_client_tr_id(&client).as_str(), "com");
///
///     // send it to the registry and receive a response of type EppNamestoreDomainRenewResponse
///     let response = client.transact::<_, EppNamestoreDomainRenewResponse>(&domain_renew).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```

pub type EppNamestoreDomainRenew = EppObject<CommandWithExtension<DomainRenew, NameStore>>;

impl EppNamestoreDomainRenew {
    /// Creates a new EppObject for NameStore domain renew
    pub fn new(
        name: &str,
        current_expiry_date: NaiveDate,
        years: u16,
        client_tr_id: &str,
        subproduct: &str,
    ) -> EppNamestoreDomainRenew {
        let exp_date_str = current_expiry_date
            .format("%Y-%m-%d")
            .to_string()
            .to_string_value();

        let domain_renew = DomainRenew {
            domain: DomainRenewData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.to_string_value(),
                current_expiry_date: exp_date_str,
                period: Period::new(years),
            },
        };

        let command = CommandWithExtension::<DomainRenew, NameStore> {
            command: domain_renew,
            extension: Some(Extension {
                data: NameStore {
                    xmlns: EPP_DOMAIN_NAMESTORE_EXT_XMLNS.to_string(),
                    subproduct: subproduct.to_string_value(),
                },
            }),
            client_tr_id: client_tr_id.to_string_value(),
        };

        EppObject::build(command)
    }
}
