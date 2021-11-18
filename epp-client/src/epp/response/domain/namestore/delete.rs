use crate::epp::object::EppObject;
use crate::epp::response::CommandResponseWithExtension;
use crate::epp::EppDomainDeleteResponse;

use super::object::EppNamestoreDomainCheckResult;

/// Type that represents the &lt;epp&gt; tag for the EPP XML NameStore create response
pub type EppNamestoreDomainDeleteResponse =
    EppObject<CommandResponseWithExtension<EppDomainDeleteResponse, EppNamestoreDomainCheckResult>>;
