//! Common data types included in EPP Requests and Responses

use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::request::Extension;

pub(crate) const EPP_XMLNS: &str = "urn:ietf:params:xml:ns:epp-1.0";

/// Wraps String for easier serialization to and from values that are inner text
/// for tags rather than attributes
#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StringValue(String);

impl Display for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for StringValue {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for StringValue {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// An empty placeholder tag. To be refactored to something more compliant later.
pub struct NoExtension;

impl Extension for NoExtension {
    type Response = NoExtension;
}

/// The <option> type in EPP XML login requests
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "options")]
pub struct Options {
    /// The EPP version being used
    pub version: StringValue,
    /// The language that will be used during EPP transactions
    pub lang: StringValue,
}

impl Options {
    /// Creates an Options object with version and lang data
    pub fn build(version: &str, lang: &str) -> Options {
        Options {
            version: version.into(),
            lang: lang.into(),
        }
    }
}

/// The <svcExtension> type in EPP XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "svcExtension")]
pub struct ServiceExtension {
    /// The service extension URIs being represented by <extURI> in EPP XML
    #[serde(rename = "extURI")]
    pub ext_uris: Option<Vec<StringValue>>,
}

/// The <svcs> type in EPP XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Services {
    /// The service URIs being used by this EPP session represented by <objURI> in EPP XML
    #[serde(rename = "objURI")]
    pub obj_uris: Vec<StringValue>,
    /// The <svcExtention> being used in this EPP session
    #[serde(rename = "svcExtension")]
    pub svc_ext: Option<ServiceExtension>,
}

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

/// The &lt;status&gt; type on contact transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectStatus {
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

/// The &lt;authInfo&gt; tag for domain and contact transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContactAuthInfo {
    /// The &lt;pw&gt; tag under &lt;authInfo&gt;
    #[serde(rename = "contact:pw", alias = "pw")]
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

impl ContactAuthInfo {
    /// Creates a ContactAuthInfo instance with the given password
    pub fn new(password: &str) -> ContactAuthInfo {
        ContactAuthInfo {
            password: password.into(),
        }
    }
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

/// This type contains a single DER-encoded X.509 certificate.
///
/// The rustls-pemfile crate can be used to parse a PEM file.
pub struct Certificate(pub Vec<u8>);

/// This type contains a DER-encoded ASN.1 private key in PKCS#8 or PKCS#1 format.
///
/// The rustls-pemfile crate can be used to parse a PEM file in these formats.
pub struct PrivateKey(pub Vec<u8>);
