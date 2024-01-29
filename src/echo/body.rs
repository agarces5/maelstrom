use serde::{Deserialize, Serialize};

use super::payload::Payload;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Body {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

impl Body {
    pub fn new(msg_id: Option<usize>, in_reply_to: Option<usize>, payload: Payload) -> Self {
        Self {
            msg_id,
            in_reply_to,
            payload,
        }
    }
}
