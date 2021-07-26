//! Types for EPP message ack response

use crate::epp::object::EppObject;
use crate::epp::response::CommandResponse;

/// Type that represents the &lt;epp&gt; tag for the EPP XML message ack response
pub type EppMessageAckResponse = EppObject<CommandResponse<String>>;
