use crate::epp::object::EppObject;
use crate::epp::response::CommandResponseWithExtension;
use crate::epp::DomainCheckResult;

use super::object::EppNamestoreDomainCheckResult;

/// Type that represents the &lt;epp&gt; tag for the EPP XML NameStore check response
pub type EppNamestoreDomainCheckResponse =
    EppObject<CommandResponseWithExtension<DomainCheckResult, EppNamestoreDomainCheckResult>>;
