use quick_xml::se;
use serde::{Deserialize, Serialize, Serializer};
use std::error::Error;

const EPP_XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;
const EPP_XMLNS: &str = "urn:ietf:params:xml:ns:epp-1.0";
const EPP_XMLNS_XSI: &str = "http://www.w3.org/2001/XMLSchema-instance";
const EPP_XSI_SCHEMA_LOCATION: &str = "urn:ietf:params:xml:ns:epp-1.0 epp-1.0.xsd";

const EPP_VERSION: &str = "1.0";
const EPP_LANG: &str = "en";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StringValue(String);

impl Default for StringValue {
    fn default() -> Self {
        Self(String::from(""))
    }
}

pub trait StringValueTrait {
    fn to_string_value(&self) -> StringValue;
}

impl StringValueTrait for &str {
    fn to_string_value(&self) -> StringValue {
        StringValue(self.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RequestType {
    Hello,
    Command {
        login: Login,
        #[serde(rename = "clTRID")]
        client_tr_id: StringValue,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "epp")]
pub struct EppObject {
    xmlns: String,
    #[serde(rename = "xmlns:xsi")]
    xmlns_xsi: String,
    #[serde(rename = "xsi:schemaLocation")]
    xsi_schema_location: String,
    data: RequestType,
}

impl EppObject {
    pub fn new(data: RequestType) -> EppObject {
        EppObject {
            data: data,
            xmlns: EPP_XMLNS.to_string(),
            xmlns_xsi: EPP_XMLNS_XSI.to_string(),
            xsi_schema_location: EPP_XSI_SCHEMA_LOCATION.to_string(),
        }
    }

    pub fn to_epp_xml(&self) -> Result<String, Box<dyn Error>> {
        let epp_xml = format!("{}\r\n{}", EPP_XML_HEADER, se::to_string(self)?);

        Ok(epp_xml)
    }
}

pub type EppRequest = EppObject;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Hello;

impl Hello {
    pub fn new() -> EppRequest {
        EppRequest::new(RequestType::Hello)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "options")]
pub struct LoginOptions {
    version: StringValue,
    lang: StringValue,
}

impl LoginOptions {
    pub fn build(version: &str, lang: &str) -> LoginOptions {
        LoginOptions {
            version: version.to_string_value(),
            lang: lang.to_string_value(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "svcExtension")]
pub struct ServiceExtension {
    #[serde(rename = "extURI")]
    ext_uris: Option<Vec<StringValue>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Services {
    #[serde(rename = "objURI")]
    obj_uris: Vec<StringValue>,
    #[serde(rename = "svcExtension")]
    svc_ext: Option<ServiceExtension>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Command {
    login: Login,
    #[serde(rename = "clTRID")]
    client_tr_id: StringValue,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "login")]
pub struct Login {
    #[serde(rename(serialize = "clID", deserialize = "clID"))]
    username: StringValue,
    #[serde(rename = "pw", default)]
    password: StringValue,
    options: LoginOptions,
    #[serde(rename = "svcs")]
    services: Services,
}

impl Login {
    pub fn new(username: &str, password: &str, client_tr_id: &str) -> EppRequest {
        let login = Login {
            username: username.to_string_value(),
            password: password.to_string_value(),
            options: LoginOptions {
                version: EPP_VERSION.to_string_value(),
                lang: EPP_LANG.to_string_value(),
            },
            services: Services {
                obj_uris: vec![
                    "urn:ietf:params:xml:ns:host-1.0".to_string_value(),
                    "urn:ietf:params:xml:ns:contact-1.0".to_string_value(),
                    "urn:ietf:params:xml:ns:domain-1.0".to_string_value(),
                ],
                svc_ext: Some(ServiceExtension {
                    ext_uris: Some(vec![
                        "http://schema.ispapi.net/epp/xml/keyvalue-1.0".to_string_value()
                    ]),
                }),
            },
        };

        EppRequest::new(RequestType::Command {
            login: login,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    pub fn set_options(&mut self, options: LoginOptions) {
        self.options = options;
    }

    pub fn set_services(&mut self, services: Services) {
        self.services = services;
    }
}
