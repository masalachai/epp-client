use epp_client_macros::*;

use crate::epp::request::domain::check::EppDomainCheck;
use crate::epp::request::EppRequest;
use crate::epp::response::domain::check::EppDomainCheckResponse;
use crate::epp::xml::EppXml;

#[derive(EppRequest, Debug)]
#[response(EppDomainCheckResponse)]
pub struct Request(EppDomainCheck);

impl Request {
    /// Creates a new Request for a domain check
    pub fn new(domains: Vec<&str>, client_tr_id: &str) -> Request {
        Request(EppDomainCheck::new(domains, client_tr_id))
    }
}
