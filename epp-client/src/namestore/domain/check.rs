use crate::domain::check::Request;
use crate::epp::object::StringValueTrait;
use crate::epp::request::EppRequest;
use crate::epp::response::domain::check::EppDomainCheckResponse;
use crate::epp::xml::EPP_DOMAIN_NAMESTORE_EXT_XMLNS;
use crate::namestore::extension::NameStore;

#[derive(Debug)]
pub struct Check {
    request: Request,
    extension: NameStore,
}

impl EppRequest for Check {
    type Extension = NameStore;
    type Input = Request;
    type Output = EppDomainCheckResponse;

    fn into_parts(self) -> (Self::Input, Option<Self::Extension>) {
        (self.request, Some(self.extension))
    }
}

impl Check {
    pub fn new(domains: impl IntoIterator<Item = impl AsRef<str>>, subproduct: &str) -> Self {
        let request = crate::domain::check::Check::new(domains);
        Self {
            request: request.into_parts().0,
            extension: NameStore {
                xmlns: EPP_DOMAIN_NAMESTORE_EXT_XMLNS.to_string(),
                subproduct: subproduct.to_string_value(),
            },
        }
    }
}
