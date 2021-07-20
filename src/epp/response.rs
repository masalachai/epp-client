// use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::error::Error;

use crate::epp::object::{EppObject, Options, ServiceExtension, Services, StringValue};
use crate::epp::xml::{EPP_XMLNS, EPP_XMLNS_XSI, EPP_XSI_SCHEMA_LOCATION};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseType {
    #[serde(rename = "greeting")]
    Greeting(Greeting),
}

pub type EppResponse = EppObject<ResponseType>;

#[derive(Serialize, Debug, PartialEq)]
pub struct ServiceMenu {
    pub options: Options,
    pub services: Services,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct FlattenedServiceMenu {
    pub version: StringValue,
    pub lang: StringValue,
    #[serde(rename = "objURI")]
    pub obj_uris: Vec<StringValue>,
    #[serde(rename = "svcExtension")]
    pub svc_ext: Option<ServiceExtension>,
}

impl<'de> Deserialize<'de> for ServiceMenu {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let flattened_svc_menu = FlattenedServiceMenu::deserialize(deserializer)?;

        let svc_menu = ServiceMenu {
            options: Options {
                version: flattened_svc_menu.version,
                lang: flattened_svc_menu.lang,
            },
            services: Services {
                obj_uris: flattened_svc_menu.obj_uris,
                svc_ext: flattened_svc_menu.svc_ext,
            },
        };

        Ok(svc_menu)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct All;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Access {
    all: All,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Admin;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Prov;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Purpose {
    admin: Admin,
    prov: Prov,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Ours;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Public;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Recipient {
    ours: Ours,
    public: Public,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Stated;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Retention {
    stated: Stated,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Statement {
    purpose: Purpose,
    recipient: Recipient,
    retention: Retention,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Dcp {
    access: Access,
    statement: Statement,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Greeting {
    #[serde(rename = "svID")]
    service_id: String,
    #[serde(rename = "svDate")]
    service_date: String,
    #[serde(rename = "svcMenu")]
    svc_menu: ServiceMenu,
    dcp: Dcp,
}
