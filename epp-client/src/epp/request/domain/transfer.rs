use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::data::{AuthInfo, Period};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppDomainTransferRequest = EppObject<Command<DomainTransfer>>;
pub type EppDomainTransferApprove = EppObject<Command<DomainTransfer>>;
pub type EppDomainTransferReject = EppObject<Command<DomainTransfer>>;
pub type EppDomainTransferCancel = EppObject<Command<DomainTransfer>>;
pub type EppDomainTransferQuery = EppObject<Command<DomainTransfer>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainTransferData {
    xmlns: String,
    name: StringValue,
    period: Option<Period>,
    #[serde(rename = "authInfo")]
    auth_info: Option<AuthInfo>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "transfer")]
pub struct DomainTransfer {
    #[serde(rename = "op")]
    operation: String,
    #[serde(rename = "transfer")]
    domain: DomainTransferData,
}

impl EppDomainTransferRequest {
    pub fn request(
        name: &str,
        years: u16,
        auth_password: &str,
        client_tr_id: &str,
    ) -> EppDomainTransferRequest {
        EppObject::build(Command::<DomainTransfer> {
            command: DomainTransfer {
                operation: "request".to_string(),
                domain: DomainTransferData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    period: Some(Period::new(years)),
                    auth_info: Some(AuthInfo::new(auth_password)),
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    pub fn set_period(&mut self, period: Period) {
        self.data.command.domain.period = Some(period);
    }
}

impl EppDomainTransferApprove {
    pub fn approve(name: &str, client_tr_id: &str) -> EppDomainTransferApprove {
        EppObject::build(Command::<DomainTransfer> {
            command: DomainTransfer {
                operation: "approve".to_string(),
                domain: DomainTransferData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    period: None,
                    auth_info: None,
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}

impl EppDomainTransferCancel {
    pub fn cancel(name: &str, client_tr_id: &str) -> EppDomainTransferCancel {
        EppObject::build(Command::<DomainTransfer> {
            command: DomainTransfer {
                operation: "cancel".to_string(),
                domain: DomainTransferData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    period: None,
                    auth_info: None,
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}

impl EppDomainTransferReject {
    pub fn reject(name: &str, client_tr_id: &str) -> EppDomainTransferReject {
        EppObject::build(Command::<DomainTransfer> {
            command: DomainTransfer {
                operation: "reject".to_string(),
                domain: DomainTransferData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    period: None,
                    auth_info: None,
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}

impl EppDomainTransferQuery {
    pub fn query(name: &str, auth_password: &str, client_tr_id: &str) -> EppDomainTransferQuery {
        EppObject::build(Command::<DomainTransfer> {
            command: DomainTransfer {
                operation: "query".to_string(),
                domain: DomainTransferData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    period: None,
                    auth_info: Some(AuthInfo::new(auth_password)),
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
