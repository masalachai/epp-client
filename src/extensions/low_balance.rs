//! Low Balance Mapping for the Extensible Provisioning Protocol (EPP)
//!
//! https://www.verisign.com/assets/epp-sdk/verisign_epp-extension_low-balance_v01.html

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LowBalance {
    pub registrar_name: String,
    pub credit_limit: String,
    pub credit_threshold: Threshold,
    pub available_credit: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Threshold {
    pub r#type: ThresholdType,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ThresholdType {
    Fixed,
    Percent,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::poll::{MessageData, MessagePollResponse};
    use crate::message::MessagePoll;
    use crate::response::ResultCode;
    use crate::tests::{response_from_file, CLTRID, SVTRID};

    #[test]
    fn low_balance() {
        let object = response_from_file::<MessagePoll>("response/message/poll_low_balance.xml");
        dbg!(&object);

        let low_balance = match object.res_data {
            Some(MessagePollResponse {
                message_data: MessageData::LowBalance(low_balance),
            }) => low_balance,
            _ => panic!("Unexpected message data"),
        };

        assert_eq!(low_balance.registrar_name, "Foobar, Inc.");
        assert_eq!(low_balance.credit_limit, "0");
        assert_eq!(
            low_balance.credit_threshold,
            Threshold {
                r#type: ThresholdType::Fixed,
                value: "500".into(),
            }
        );
        assert_eq!(low_balance.available_credit, "491.31");

        assert_eq!(
            object.result.code,
            ResultCode::CommandCompletedSuccessfullyAckToDequeue
        );
        assert_eq!(
            object.result.message,
            "Command completed successfully; ack to dequeue".into()
        );

        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
