pub mod contact;
pub mod domain;
pub mod host;
pub mod message;

use epp_client_macros::*;
use serde::{Deserialize, Deserializer, Serialize};

use crate::epp::object::{
    ElementName, EppObject, Options, ServiceExtension, Services, StringValue,
};

pub type EppGreeting = EppObject<Greeting>;
pub type EppCommandResponseStatus = EppObject<CommandResponseStatus>;
type CommandResponseError = CommandResponseStatus;
pub type EppCommandResponseError = EppObject<CommandResponseError>;
pub type EppCommandResponse = EppObject<CommandResponse<String>>;

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
    pub all: All,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Admin;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Prov;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Purpose {
    pub admin: Admin,
    pub prov: Prov,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Ours;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Public;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Recipient {
    pub ours: Ours,
    pub public: Public,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Stated;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Retention {
    pub stated: Stated,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Statement {
    pub purpose: Purpose,
    pub recipient: Recipient,
    pub retention: Retention,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Dcp {
    pub access: Access,
    pub statement: Statement,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[serde(rename_all = "lowercase")]
#[element_name(name = "greeting")]
pub struct Greeting {
    #[serde(rename = "svID")]
    pub service_id: String,
    #[serde(rename = "svDate")]
    pub service_date: String,
    #[serde(rename = "svcMenu")]
    pub svc_menu: ServiceMenu,
    pub dcp: Dcp,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Undef;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResultValue {
    #[serde(rename = "xmlns:epp")]
    xmlns: String,
    pub undef: Undef,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExtValue {
    pub value: ResultValue,
    pub reason: StringValue,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EppResult {
    pub code: u16,
    #[serde(rename = "msg")]
    pub message: StringValue,
    #[serde(rename = "extValue")]
    pub ext_value: Option<ExtValue>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResponseTRID {
    #[serde(rename = "clTRID")]
    pub client_tr_id: Option<StringValue>,
    #[serde(rename = "svTRID")]
    pub server_tr_id: StringValue,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
pub struct MessageQueue {
    pub count: u32,
    pub id: String,
    #[serde(rename = "qDate")]
    pub date: StringValue,
    #[serde(rename = "msg")]
    pub message: StringValue,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[serde(rename_all = "lowercase")]
#[element_name(name = "response")]
pub struct CommandResponse<T> {
    pub result: EppResult,
    #[serde(rename = "msgQ")]
    pub message_queue: Option<MessageQueue>,
    #[serde(rename = "resData")]
    pub res_data: Option<T>,
    #[serde(rename = "trID")]
    pub tr_ids: ResponseTRID,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "response")]
pub struct CommandResponseStatus {
    pub result: EppResult,
    #[serde(rename = "trID")]
    pub tr_ids: ResponseTRID,
}
