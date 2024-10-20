use serde::{Deserialize, Serialize};

use crate::model::MessageType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    #[serde(flatten)]
    pub _type: MessageType,
    pub msg_id: Option<u32>,
    pub in_reply_to: Option<u32>,
}

impl Body {
    pub fn new(_type: MessageType, msg_id: Option<u32>, in_reply_to: Option<u32>) -> Self {
        Self {
            _type,
            msg_id,
            in_reply_to,
        }
    }

    pub fn msg_id(&self) -> Option<u32> {
        self.msg_id
    }

    pub fn _type(&self) -> &MessageType {
        &self._type
    }
}
