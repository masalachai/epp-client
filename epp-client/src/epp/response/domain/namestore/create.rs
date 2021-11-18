use crate::epp::object::EppObject;
use crate::epp::response::CommandResponseWithExtension;
use crate::epp::DomainCreateResult;

use super::object::EppNamestoreDomainCheckResult;

/// Type that represents the &lt;epp&gt; tag for the EPP XML NameStore create response
pub type EppNamestoreDomainCreateResponse =
    EppObject<CommandResponseWithExtension<DomainCreateResult, EppNamestoreDomainCheckResult>>;
