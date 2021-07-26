use crate::epp::object::EppObject;
use crate::epp::response::CommandResponse;

pub type EppMessageAckResponse = EppObject<CommandResponse<String>>;
