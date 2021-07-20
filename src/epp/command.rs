use crate::epp::object::{ElementName, StringValue};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq)]
pub struct Command<T: ElementName> {
    pub command: T,
    #[serde(rename = "clTRID")]
    pub client_tr_id: StringValue,
}

impl<T: ElementName> ElementName for Command<T> {
    fn element_name(&self) -> &'static str {
        "command"
    }
}

impl<T: ElementName + Serialize> Serialize for Command<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let command_name = self.command.element_name();
        let mut state = serializer.serialize_struct("command", 2)?;
        state.serialize_field(command_name, &self.command)?;
        state.serialize_field("clTRID", &self.client_tr_id)?;
        state.end()
    }
}
