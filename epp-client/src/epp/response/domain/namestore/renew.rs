use crate::epp::object::EppObject;
use crate::epp::response::CommandResponseWithExtension;
use crate::epp::DomainRenewResult;

use super::object::EppNamestoreDomainCheckResult;

/// Type that represents the &lt;epp&gt; tag for the EPP XML NameStore renew response
pub type EppNamestoreDomainRenewResponse =
    EppObject<CommandResponseWithExtension<DomainRenewResult, EppNamestoreDomainCheckResult>>;
