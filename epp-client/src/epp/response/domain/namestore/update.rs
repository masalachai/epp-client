use crate::epp::object::EppObject;
use crate::epp::response::CommandResponseWithExtension;
use crate::epp::DomainCreateResult;

use super::object::EppNamestoreDomainCheckResult;

/// Type that represents the &lt;epp&gt; tag for the EPP XML NameStore update response
pub type EppNamestoreDomainUpdateResponse =
    EppObject<CommandResponseWithExtension<DomainCreateResult, EppNamestoreDomainCheckResult>>;
