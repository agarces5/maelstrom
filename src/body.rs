use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Body<MessageType> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: MessageType,
}

impl<MessageType> Body<MessageType> {
    pub fn new(msg_id: Option<usize>, in_reply_to: Option<usize>, payload: MessageType) -> Self {
        Self {
            msg_id,
            in_reply_to,
            payload,
        }
    }
}
