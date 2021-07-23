use crate::epp::object::{StringValue, StringValueTrait};
use serde::{Deserialize, Serialize};

pub type DomainStatus = ContactStatus;
pub type HostStatus = ContactStatus;

#[derive(Serialize, Deserialize, Debug)]
pub enum DomainNsList {
    HostAttrList(HostAttrList),
    HostObjList(HostObjList),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostAddr {
    #[serde(rename = "ip")]
    ip_version: Option<String>,
    #[serde(rename = "$value")]
    address: String,
}

impl HostAddr {
    pub fn new(ip_version: &str, address: &str) -> HostAddr {
        HostAddr {
            ip_version: Some(ip_version.to_string()),
            address: address.to_string(),
        }
    }

    pub fn new_v4(address: &str) -> HostAddr {
        HostAddr {
            ip_version: Some("v4".to_string()),
            address: address.to_string(),
        }
    }

    pub fn new_v6(address: &str) -> HostAddr {
        HostAddr {
            ip_version: Some("v6".to_string()),
            address: address.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Host {
    pub name: StringValue,
    #[serde(rename = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostAttr {
    #[serde(rename = "hostName")]
    pub name: StringValue,
    #[serde(rename = "hostAddr")]
    pub addresses: Option<Vec<HostAddr>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostAttrList {
    #[serde(rename = "hostAttr")]
    pub hosts: Vec<HostAttr>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostObjList {
    #[serde(rename = "hostObj")]
    pub hosts: Vec<StringValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainContact {
    #[serde(rename = "$value")]
    pub id: String,
    #[serde(rename = "type")]
    pub contact_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Period {
    unit: String,
    #[serde(rename = "$value")]
    length: u16,
}

impl Period {
    pub fn new(length: u16) -> Period {
        Period {
            unit: "y".to_string(),
            length: length,
        }
    }

    pub fn set_unit(&mut self, unit: &str) {
        self.unit = unit.to_string();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactStatus {
    #[serde(rename = "s")]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Phone {
    #[serde(rename = "$value")]
    pub number: String,
    #[serde(rename = "x")]
    extension: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    street: Vec<StringValue>,
    city: StringValue,
    #[serde(rename = "sp")]
    province: StringValue,
    #[serde(rename = "pc")]
    postal_code: StringValue,
    #[serde(rename = "cc")]
    country_code: StringValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostalInfo {
    #[serde(rename = "type")]
    info_type: String,
    name: StringValue,
    #[serde(rename = "org")]
    organization: StringValue,
    #[serde(rename = "addr")]
    address: Address,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthInfo {
    #[serde(rename = "pw")]
    pub password: StringValue,
}

impl Phone {
    pub fn new(number: &str) -> Phone {
        Phone {
            extension: None,
            number: number.to_string(),
        }
    }

    pub fn set_extension(&mut self, ext: &str) {
        self.extension = Some(ext.to_string());
    }
}

impl AuthInfo {
    pub fn new(password: &str) -> AuthInfo {
        AuthInfo {
            password: password.to_string_value(),
        }
    }
}

impl Address {
    pub fn new(
        street: Vec<&str>,
        city: &str,
        province: &str,
        postal_code: &str,
        country_code: &str,
    ) -> Address {
        let street = street
            .iter()
            .filter_map(|s| Some(s.to_string_value()))
            .collect::<Vec<StringValue>>();

        Address {
            street: street,
            city: city.to_string_value(),
            province: province.to_string_value(),
            postal_code: postal_code.to_string_value(),
            country_code: country_code.to_string_value(),
        }
    }
}

impl PostalInfo {
    pub fn new(info_type: &str, name: &str, organization: &str, address: Address) -> PostalInfo {
        PostalInfo {
            info_type: info_type.to_string(),
            name: name.to_string_value(),
            organization: organization.to_string_value(),
            address: address,
        }
    }
}
