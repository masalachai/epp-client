use std::borrow::Cow;
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
pub struct ContactAuthInfo<'a> {
    /// The &lt;pw&gt; tag under &lt;authInfo&gt;
    #[serde(rename = "contact:pw", alias = "pw")]
    pub password: StringValue<'a>,
}

impl<'a> ContactAuthInfo<'a> {
    /// Creates a ContactAuthInfo instance with the given password
    pub fn new(password: &'a str) -> Self {
        Self {
            password: password.into(),
        }
    }
}

/// The data for &lt;voice&gt; and &lt;fax&gt; types on domain transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Phone<'a> {
    /// The inner text on the &lt;voice&gt; and &lt;fax&gt; tags
    #[serde(rename = "$value")]
    pub number: Cow<'a, str>,
    /// The value of the 'x' attr on &lt;voice&gt; and &lt;fax&gt; tags
    #[serde(rename = "x")]
    pub extension: Option<Cow<'a, str>>,
}

impl<'a> Phone<'a> {
    /// Creates a new Phone instance with a given phone number
    pub fn new(number: &'a str) -> Self {
        Self {
            extension: None,
            number: number.into(),
        }
    }

    /// Sets the extension value of the Phone type
    pub fn set_extension(&mut self, ext: &'a str) {
        self.extension = Some(ext.into());
    }
}

/// The &lt;addr&gt; type on contact transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address<'a> {
    /// The &lt;street&gt; tags under &lt;addr&gt;
    #[serde(rename = "contact:street", alias = "street")]
    pub street: Vec<StringValue<'a>>,
    /// The &lt;city&gt; tag under &lt;addr&gt;
    #[serde(rename = "contact:city", alias = "city")]
    pub city: StringValue<'a>,
    /// The &lt;sp&gt; tag under &lt;addr&gt;
    #[serde(rename = "contact:sp", alias = "sp")]
    pub province: StringValue<'a>,
    /// The &lt;pc&gt; tag under &lt;addr&gt;
    #[serde(rename = "contact:pc", alias = "pc")]
    pub postal_code: StringValue<'a>,
    /// The &lt;cc&gt; tag under &lt;addr&gt;
    #[serde(rename = "contact:cc", alias = "cc")]
    pub country: Country,
}

impl<'a> Address<'a> {
    /// Creates a new Address instance
    pub fn new(
        street: &[&'a str],
        city: &'a str,
        province: &'a str,
        postal_code: &'a str,
        country: Country,
    ) -> Self {
        let street = street.iter().map(|&s| s.into()).collect();

        Self {
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
pub struct PostalInfo<'a> {
    /// The 'type' attr on &lt;postalInfo&gt;
    #[serde(rename = "type")]
    pub info_type: String,
    /// The &lt;name&gt; tag under &lt;postalInfo&gt;
    #[serde(rename = "contact:name", alias = "name")]
    pub name: StringValue<'a>,
    /// The &lt;org&gt; tag under &lt;postalInfo&gt;
    #[serde(rename = "contact:org", alias = "org")]
    pub organization: StringValue<'a>,
    /// The &lt;addr&gt; tag under &lt;postalInfo&gt;
    #[serde(rename = "contact:addr", alias = "addr")]
    pub address: Address<'a>,
}

impl<'a> PostalInfo<'a> {
    /// Creates a new PostalInfo instance
    pub fn new(
        info_type: &str,
        name: &'a str,
        organization: &'a str,
        address: Address<'a>,
    ) -> Self {
        Self {
            info_type: info_type.to_string(),
            name: name.into(),
            organization: organization.into(),
            address,
        }
    }
}
