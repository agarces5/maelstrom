use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Error {
        code: u64,
        text: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    #[serde(flatten)]
    _type: MessageType,
    msg_id: u32,
    in_reply_to: Option<u32>,
}

impl Body {
    pub fn new(_type: MessageType, msg_id: u32, in_reply_to: Option<u32>) -> Self {
        Self {
            _type,
            msg_id,
            in_reply_to,
        }
    }

    pub fn msg_id(&self) -> u32 {
        self.msg_id
    }

    pub fn _type(&self) -> &MessageType {
        &self._type
    }
}
