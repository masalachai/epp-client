//! Types for EPP contact info response with Neustar extension

use crate::epp::ext::neustar::object::data::Extension;
use crate::epp::object::EppObject;
use crate::epp::response::contact::info::ContactInfoResult;
use crate::epp::response::CommandResponseWithExtension;

/// Type that represents the &lt;epp&gt; tag for the EPP XML contact info response for Neustar
pub type EppNeustarContactInfoResponse =
    EppObject<CommandResponseWithExtension<ContactInfoResult, Extension>>;
