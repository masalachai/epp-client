use crate::epp::object::EppObject;
use crate::epp::response::CommandResponseWithExtension;
use crate::epp::DomainInfoResult;

use super::object::EppNamestoreDomainCheckResult;

/// Type that represents the &lt;epp&gt; tag for the EPP XML NameStore check response
pub type EppNamestoreDomainInfoResponse =
    EppObject<CommandResponseWithExtension<DomainInfoResult, EppNamestoreDomainCheckResult>>;
