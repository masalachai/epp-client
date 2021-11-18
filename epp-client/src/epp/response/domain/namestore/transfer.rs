use crate::epp::object::EppObject;
use crate::epp::response::CommandResponseWithExtension;
use crate::epp::DomainTransferResult;

use super::object::EppNamestoreDomainCheckResult;

/// Type that represents the &lt;epp&gt; tag for the EPP XML NameStore transfer request response
pub type EppNamestoreDomainTransferRequestResponse =
    EppObject<CommandResponseWithExtension<DomainTransferResult, EppNamestoreDomainCheckResult>>;

/// Type that represents the &lt;epp&gt; tag for the EPP XML namestore transfer approve response
pub type EppNamestoreDomainTransferApproveResponse =
    EppObject<CommandResponseWithExtension<DomainTransferResult, EppNamestoreDomainCheckResult>>;

/// Type that represents the &lt;epp&gt; tag for the EPP XML NameStore transfer reject response
pub type EppNamestoreDomainTransferRejectResponse =
    EppObject<CommandResponseWithExtension<DomainTransferResult, EppNamestoreDomainCheckResult>>;

/// Type that represents the &lt;epp&gt; tag for the EPP XML NameStore transfer cancel response
pub type EppNamestoreDomainTransferCancelResponse =
    EppObject<CommandResponseWithExtension<DomainTransferResult, EppNamestoreDomainCheckResult>>;

/// Type that represents the &lt;epp&gt; tag for the EPP XML NameStore transfer query response
pub type EppNamestoreDomainTransferQueryResponse =
    EppObject<CommandResponseWithExtension<DomainTransferResult, EppNamestoreDomainCheckResult>>;