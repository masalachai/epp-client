//! Common data types included in EPP Requests and Responses

use crate::epp::object::{StringValue, StringValueTrait};
use serde::{Deserialize, Serialize};

/// The &lt;status&gt; attribute on EPP XML for domain transactions
pub type DomainStatus = ContactStatus;
/// The &lt;status&gt; attribute on EPP XML for host transactions
pub type HostStatus = ContactStatus;

/// The &lt;hostAddr&gt; types domain or host transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct HostAddr {
    #[serde(rename = "ip")]
    pub ip_version: Option<String>,
    #[serde(rename = "$value")]
    pub address: String,
}

impl HostAddr {
    /// Creates a 'v4' type HostAddr (mostly useful when you don't want to include an 'ip' attr in the XML)
    pub fn new(ip_version: &str, address: &str) -> HostAddr {
        HostAddr {
            ip_version: Some(ip_version.to_string()),
            address: address.to_string(),
        }
    }

    /// Creates a 'v4' type HostAddr
    pub fn new_v4(address: &str) -> HostAddr {
        HostAddr {
            ip_version: Some("v4".to_string()),
            address: address.to_string(),
        }
    }

    /// Creates a 'v6' type HostAddr
    pub fn new_v6(address: &str) -> HostAddr {
        HostAddr {
            ip_version: Some("v6".to_string()),
            address: address.to_string(),
        }
    }
}

/// The &lt;host&gt; type for host transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct Host {
    /// The &lt;hostName&gt; tag
    pub name: StringValue,
    /// The &lt;hostAddr&gt; tags
    #[serde(rename = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
}

/// The &lt;hostAttr&gt; type for domain transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct HostAttr {
    /// The &lt;hostName&gt; tag
    #[serde(rename = "hostName")]
    pub name: StringValue,
    /// The &lt;hostAddr&gt; tags
    #[serde(rename = "hostAddr")]
    pub addresses: Option<Vec<HostAddr>>,
}

/// Enum that can accept one type which corresponds to either the &lt;hostObj&gt; or &lt;hostAttr&gt;
/// list of tags
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum HostList {
    HostObjList(HostObjList),
    HostAttrList(HostAttrList),
}

/// The list of &lt;hostAttr&gt; types for domain transactions. Typically under an &lt;ns&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostAttrList {
    /// The list of &lt;hostAttr&gt; tags
    #[serde(rename = "hostAttr")]
    pub hosts: Vec<HostAttr>,
}

/// The list of &lt;hostObj&gt; types for domain transactions. Typically under an &lt;ns&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostObjList {
    /// The list of &lt;hostObj&gt; tags
    #[serde(rename = "hostObj")]
    pub hosts: Vec<StringValue>,
}

/// The &lt;contact&gt; type on domain creation and update requests
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainContact {
    /// The contact id
    #[serde(rename = "$value")]
    pub id: String,
    /// The contact type attr (usually admin, billing, or tech in most registries)
    #[serde(rename = "type")]
    pub contact_type: String,
}

/// The &lt;period&gt; type for registration, renewal or transfer on domain transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct Period {
    /// The interval (usually 'y' indicating years)
    unit: String,
    /// The length of the registration, renewal or transfer period (usually in years)
    #[serde(rename = "$value")]
    length: u16,
}

impl Period {
    /// Creates a new period in years
    pub fn new(length: u16) -> Period {
        Period {
            unit: "y".to_string(),
            length,
        }
    }

    /// Sets the period unit ('y' for years, most commonly)
    pub fn set_unit(&mut self, unit: &str) {
        self.unit = unit.to_string();
    }
}

/// The &lt;status&gt; type on contact transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactStatus {
    /// The status name, represented by the 's' attr on &lt;status&gt; tags
    #[serde(rename = "s")]
    pub status: String,
}

/// The data for &lt;voice&gt; and &lt;fax&gt; types on domain transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Phone {
    /// The inner text on the &lt;voice&gt; and &lt;fax&gt; tags
    #[serde(rename = "$value")]
    pub number: String,
    /// The value of the 'x' attr on &lt;voice&gt; and &lt;fax&gt; tags
    #[serde(rename = "x")]
    pub extension: Option<String>,
}

/// The &lt;addr&gt; type on contact transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    /// The &lt;street&gt; tags under &lt;addr&gt;
    pub street: Vec<StringValue>,
    /// The &lt;city&gt; tag under &lt;addr&gt;
    pub city: StringValue,
    /// The &lt;sp&gt; tag under &lt;addr&gt;
    #[serde(rename = "sp")]
    pub province: StringValue,
    /// The &lt;pc&gt; tag under &lt;addr&gt;
    #[serde(rename = "pc")]
    pub postal_code: StringValue,
    /// The &lt;cc&gt; tag under &lt;addr&gt;
    #[serde(rename = "cc")]
    pub country_code: StringValue,
}

/// The &lt;postalInfo&gt; type on contact transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostalInfo {
    /// The 'type' attr on &lt;postalInfo&gt;
    #[serde(rename = "type")]
    pub info_type: String,
    /// The &lt;name&gt; tag under &lt;postalInfo&gt;
    pub name: StringValue,
    /// The &lt;org&gt; tag under &lt;postalInfo&gt;
    #[serde(rename = "org")]
    pub organization: StringValue,
    /// The &lt;addr&gt; tag under &lt;postalInfo&gt;
    #[serde(rename = "addr")]
    pub address: Address,
}

/// The &lt;authInfo&gt; tag for domain and contact transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthInfo {
    /// The &lt;pw&gt; tag under &lt;authInfo&gt;
    #[serde(rename = "pw")]
    pub password: StringValue,
}

impl Phone {
    /// Creates a new Phone instance with a given phone number
    pub fn new(number: &str) -> Phone {
        Phone {
            extension: None,
            number: number.to_string(),
        }
    }

    /// Sets the extension value of the Phone type
    pub fn set_extension(&mut self, ext: &str) {
        self.extension = Some(ext.to_string());
    }
}

impl AuthInfo {
    /// Creates an AuthInfo instance with the given password
    pub fn new(password: &str) -> AuthInfo {
        AuthInfo {
            password: password.to_string_value(),
        }
    }
}

impl Address {
    /// Creates a new Address instance
    pub fn new(
        street: Vec<&str>,
        city: &str,
        province: &str,
        postal_code: &str,
        country_code: &str,
    ) -> Address {
        let street = street
            .iter()
            .map(|s| s.to_string_value())
            .collect::<Vec<StringValue>>();

        Address {
            street,
            city: city.to_string_value(),
            province: province.to_string_value(),
            postal_code: postal_code.to_string_value(),
            country_code: country_code.to_string_value(),
        }
    }
}

impl PostalInfo {
    /// Creates a new PostalInfo instance
    pub fn new(info_type: &str, name: &str, organization: &str, address: Address) -> PostalInfo {
        PostalInfo {
            info_type: info_type.to_string(),
            name: name.to_string_value(),
            organization: organization.to_string_value(),
            address,
        }
    }
}
