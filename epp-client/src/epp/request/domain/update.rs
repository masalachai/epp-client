//! Types for EPP domain check request

use epp_client_macros::*;

use crate::epp::object::data::{AuthInfo, DomainContact, DomainStatus, HostAttrList, HostObjList};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for domain &lt;update&gt; command
/// with &lt;hostObj&gt; elements in the request for &lt;ns&gt; list
///
/// ## Usage
///
/// ```ignore
/// use epp_client::EppClient;
/// use epp_client::epp::object::data::{DomainStatus, DomainContact};
/// use epp_client::epp::{EppDomainUpdate, EppDomainUpdateResponse, DomainAddRemove};
/// use epp_client::epp::generate_client_tr_id;
///
/// #[tokio::main]
/// async fn main() {
///     // Create an instance of EppClient, specifying the name of the registry as in
///     // the config file
///     let mut client = match EppClient::new("verisign").await {
///         Ok(client) => client,
///         Err(e) => panic!("Failed to create EppClient: {}",  e)
///     };
///
///     // Create an EppDomainUpdate instance
///     let mut domain_update = EppDomainUpdate::new("eppdev-100.com", generate_client_tr_id(&client).as_str());
///
///     let add = DomainAddRemove {
///         ns: None,
///         contacts: None,
///         statuses: Some(vec![
///             DomainStatus {
///                 status: "clientUpdateProhibited".to_string()
///             }
///         ])
///     };
///
///     let remove = DomainAddRemove {
///         ns: None,
///         contacts: Some(vec![
///             DomainContact {
///                 contact_type: "billing".to_string(),
///                 id: "eppdev-contact-2".to_string()
///             }
///         ]),
///         statuses: None,
///     };
///
///     domain_update.add(add);
///     domain_update.remove(remove);
///
///     // send it to the registry and receive a response of type EppDomainUpdateResponse
///     let response = client.transact::<_, EppDomainUpdateResponse>(&domain_update).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppDomainUpdate = EppObject<Command<DomainUpdate<HostObjList>>>;
/// Type that represents the &lt;epp&gt; request for domain &lt;update&gt; command
/// with &lt;hostAttr&gt; elements in the request for &lt;ns&gt; list
pub type EppDomainUpdateWithHostAttr = EppObject<Command<DomainUpdate<HostAttrList>>>;

/// Type for elements under the &lt;chg&gt; tag for domain update
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainChangeInfo {
    /// The new registrant contact for the domain
    pub registrant: Option<StringValue>,
    /// The new auth info for the domain
    #[serde(rename = "authInfo")]
    pub auth_info: Option<AuthInfo>,
}

/// Type for elements under the &lt;add&gt; and &lt;rem&gt; tags for domain update
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainAddRemove<T> {
    /// The list of nameservers to add or remove
    /// Type T can be either a `HostObjList` or `HostAttrList`
    #[serde(rename = "ns")]
    pub ns: Option<T>,
    /// The list of contacts to add to or remove from the domain
    #[serde(rename = "contact")]
    pub contacts: Option<Vec<DomainContact>>,
    /// The list of statuses to add to or remove from the domain
    #[serde(rename = "status")]
    pub statuses: Option<Vec<DomainStatus>>,
}

/// Type for elements under the &lt;update&gt; tag for domain update
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainUpdateData<T> {
    /// XML namespace for domain commands
    xmlns: String,
    /// The name of the domain to update
    name: StringValue,
    /// `DomainAddRemove` Object containing the list of elements to be added
    /// to the domain
    add: Option<DomainAddRemove<T>>,
    /// `DomainAddRemove` Object containing the list of elements to be removed
    /// from the domain
    #[serde(rename = "rem")]
    remove: Option<DomainAddRemove<T>>,
    /// The data under the &lt;chg&gt; tag for domain update
    #[serde(rename = "chg")]
    change_info: Option<DomainChangeInfo>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "update")]
/// Type for EPP XML &lt;update&gt; command for domains
pub struct DomainUpdate<T> {
    #[serde(rename = "update")]
    domain: DomainUpdateData<T>,
}

impl EppDomainUpdate {
    /// Creates a new EppObject for domain update corresponding to the &lt;epp&gt; tag in EPP XML
    /// with the &lt;ns&gt; tag containing &lt;hostObj&gt; tags
    pub fn new(name: &str, client_tr_id: &str) -> EppDomainUpdate {
        EppObject::build(Command::<DomainUpdate<HostObjList>> {
            command: DomainUpdate {
                domain: DomainUpdateData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    add: None,
                    remove: None,
                    change_info: None,
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    /// Sets the data for the &lt;chg&gt; tag
    pub fn info(&mut self, info: DomainChangeInfo) {
        self.data.command.domain.change_info = Some(info);
    }

    /// Sets the data for the &lt;add&gt; tag
    pub fn add(&mut self, add: DomainAddRemove<HostObjList>) {
        self.data.command.domain.add = Some(add);
    }

    /// Sets the data for the &lt;rem&gt; tag
    pub fn remove(&mut self, remove: DomainAddRemove<HostObjList>) {
        self.data.command.domain.remove = Some(remove);
    }
}

impl EppDomainUpdateWithHostAttr {
    /// Creates a new EppObject for domain update corresponding to the &lt;epp&gt; tag in EPP XML
    /// with the &lt;ns&gt; tag containing &lt;hostAttr&gt; tags
    pub fn new(name: &str, client_tr_id: &str) -> EppDomainUpdateWithHostAttr {
        EppObject::build(Command::<DomainUpdate<HostAttrList>> {
            command: DomainUpdate {
                domain: DomainUpdateData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    add: None,
                    remove: None,
                    change_info: None,
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    /// Sets the data for the &lt;chg&gt; tag
    pub fn info(&mut self, info: DomainChangeInfo) {
        self.data.command.domain.change_info = Some(info);
    }

    /// Sets the data for the &lt;add&gt; tag
    pub fn add(&mut self, add: DomainAddRemove<HostAttrList>) {
        self.data.command.domain.add = Some(add);
    }

    /// Sets the data for the &lt;rem&gt; tag
    pub fn remove(&mut self, remove: DomainAddRemove<HostAttrList>) {
        self.data.command.domain.remove = Some(remove);
    }
}
