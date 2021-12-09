use serde::{Deserialize, Serialize};

pub mod report;
pub mod request;

pub const XMLNS: &str = "urn:ietf:params:xml:ns:rgp-1.0";

#[derive(Debug, Deserialize, Serialize)]
pub struct Update<T> {
    #[serde(
        rename = "rgp:update",
        alias = "update",
        alias = "upData",
        alias = "infData"
    )]
    pub data: T,
}
