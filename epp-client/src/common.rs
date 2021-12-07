//! Common data types included in EPP Requests and Responses

use std::error::Error as StdError;
use std::{fmt::Display, str::FromStr};

use epp_client_macros::ElementName;
use serde::{de::DeserializeOwned, ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::error::Error;
use crate::request::EppExtension;

const EPP_XMLNS: &str = "urn:ietf:params:xml:ns:epp-1.0";

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

/// Trait to set correct value for xml tags when tags are being generated from generic types
pub trait ElementName {
    const ELEMENT: &'static str;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "empty")]
/// An empty placeholder tag. To be refactored to something more compliant later.
pub struct NoExtension;

impl EppExtension for NoExtension {
    type Response = NoExtension;
}

/// An EPP XML Document that is used either as an EPP XML request or
/// an EPP XML response
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename = "epp")]
pub struct EppObject<T> {
    /// XML namespace for the &lt;epp&gt; tag
    pub xmlns: String,
    /// the request or response object that is set or received in the EPP XML document
    #[serde(alias = "greeting", alias = "response")]
    pub data: T,
    // TODO: save serialized xml in the instance for debugging or client logging purposes
    // #[serde(skip)]
    // pub xml: Option<String>,
}

impl<T: DeserializeOwned> FromStr for EppObject<T> {
    type Err = Error;

    fn from_str(xml: &str) -> Result<Self, Error> {
        quick_xml::de::from_str(xml).map_err(|e| {
            Error::EppDeserializationError(format!("epp-client Deserialization Error: {}", e))
        })
    }
}

impl<T: ElementName + Serialize> EppObject<T> {
    /// Serializes the EppObject instance to an EPP XML document
    pub fn to_string(&self) -> Result<String, Box<dyn StdError>> {
        Ok(format!(
            "{}\r\n{}",
            EPP_XML_HEADER,
            quick_xml::se::to_string(self)?
        ))
    }
}

impl<T: ElementName + Serialize> Serialize for EppObject<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("epp", 4)?;
        state.serialize_field("xmlns", &self.xmlns)?;
        state.serialize_field(T::ELEMENT, &self.data)?;
        state.end()
    }
}

const EPP_XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;

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

/// Type representing the &lt;extension&gt; tag for an EPP document
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename = "extension")]
pub struct Extension<E: ElementName> {
    /// Data under the &lt;extension&gt; tag
    #[serde(alias = "upData", alias = "namestoreExt", alias = "infData")]
    pub data: E,
}

impl<E: ElementName + Serialize> Serialize for Extension<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("extension", 1)?;
        state.serialize_field(E::ELEMENT, &self.data)?;
        state.end()
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

impl<T: ElementName> EppObject<T> {
    /// Create the enclosing EPP XML tag &lt;epp&gt; for data that represents an EPP XML request or response
    pub fn build(data: T) -> EppObject<T> {
        EppObject {
            // xml: None,
            data,
            xmlns: EPP_XMLNS.to_string(),
        }
    }
}

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
    #[serde(rename = "host:name", alias = "name")]
    pub name: StringValue,
    /// The &lt;hostAddr&gt; tags
    #[serde(rename = "host:addr", alias = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
}

/// The &lt;hostAttr&gt; type for domain transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct HostAttr {
    /// The &lt;hostName&gt; tag
    #[serde(rename = "domain:hostName", alias = "hostName")]
    pub name: StringValue,
    /// The &lt;hostAddr&gt; tags
    #[serde(rename = "domain:hostAddr", alias = "hostAddr")]
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
    #[serde(rename = "domain:hostAttr", alias = "hostAttr")]
    pub hosts: Vec<HostAttr>,
}

/// The list of &lt;hostObj&gt; types for domain transactions. Typically under an &lt;ns&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostObjList {
    /// The list of &lt;hostObj&gt; tags
    #[serde(rename = "domain:hostObj", alias = "hostObj")]
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
pub struct DomainAuthInfo {
    /// The &lt;pw&gt; tag under &lt;authInfo&gt;
    #[serde(rename = "domain:pw", alias = "pw")]
    pub password: StringValue,
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

impl DomainAuthInfo {
    /// Creates a DomainAuthInfo instance with the given password
    pub fn new(password: &str) -> DomainAuthInfo {
        DomainAuthInfo {
            password: password.into(),
        }
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
