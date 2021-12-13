use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::common::StringValue;

pub mod check;
pub mod create;
pub mod delete;
pub mod info;
pub mod update;

pub const XMLNS: &str = "urn:ietf:params:xml:ns:contact-1.0";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Country(celes::Country);

impl FromStr for Country {
    type Err = <celes::Country as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(celes::Country::from_str(s)?))
    }
}

impl std::ops::Deref for Country {
    type Target = celes::Country;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The &lt;authInfo&gt; tag for domain and contact transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContactAuthInfo {
    /// The &lt;pw&gt; tag under &lt;authInfo&gt;
    #[serde(rename = "contact:pw", alias = "pw")]
    pub password: StringValue,
}

impl ContactAuthInfo {
    /// Creates a ContactAuthInfo instance with the given password
    pub fn new(password: &str) -> ContactAuthInfo {
        ContactAuthInfo {
            password: password.into(),
        }
    }
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

/// The &lt;addr&gt; type on contact transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    /// The &lt;street&gt; tags under &lt;addr&gt;
    #[serde(rename = "contact:street", alias = "street")]
    pub street: Vec<StringValue>,
    /// The &lt;city&gt; tag under &lt;addr&gt;
    #[serde(rename = "contact:city", alias = "city")]
    pub city: StringValue,
    /// The &lt;sp&gt; tag under &lt;addr&gt;
    #[serde(rename = "contact:sp", alias = "sp")]
    pub province: StringValue,
    /// The &lt;pc&gt; tag under &lt;addr&gt;
    #[serde(rename = "contact:pc", alias = "pc")]
    pub postal_code: StringValue,
    /// The &lt;cc&gt; tag under &lt;addr&gt;
    #[serde(rename = "contact:cc", alias = "cc")]
    pub country: Country,
}

impl Address {
    /// Creates a new Address instance
    pub fn new(
        street: &[&str],
        city: &str,
        province: &str,
        postal_code: &str,
        country: Country,
    ) -> Address {
        let street = street.iter().map(|&s| s.into()).collect();

        Address {
            street,
            city: city.into(),
            province: province.into(),
            postal_code: postal_code.into(),
            country,
        }
    }
}

/// The &lt;postalInfo&gt; type on contact transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostalInfo {
    /// The 'type' attr on &lt;postalInfo&gt;
    #[serde(rename = "type")]
    pub info_type: String,
    /// The &lt;name&gt; tag under &lt;postalInfo&gt;
    #[serde(rename = "contact:name", alias = "name")]
    pub name: StringValue,
    /// The &lt;org&gt; tag under &lt;postalInfo&gt;
    #[serde(rename = "contact:org", alias = "org")]
    pub organization: StringValue,
    /// The &lt;addr&gt; tag under &lt;postalInfo&gt;
    #[serde(rename = "contact:addr", alias = "addr")]
    pub address: Address,
}

impl PostalInfo {
    /// Creates a new PostalInfo instance
    pub fn new(info_type: &str, name: &str, organization: &str, address: Address) -> PostalInfo {
        PostalInfo {
            info_type: info_type.to_string(),
            name: name.into(),
            organization: organization.into(),
            address,
        }
    }
}
