use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Echo,
    EchoOk { in_reply_to: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    #[serde(flatten)]
    _type: MessageType,
    msg_id: u64,
    echo: String,
}

impl Body {
    pub fn new(_type: MessageType, msg_id: u64, echo: String) -> Body {
        Body {
            _type,
            msg_id,
            echo,
        }
    }

    pub fn msg_id(&self) -> u64 {
        self.msg_id
    }

    pub fn echo(&self) -> &str {
        &self.echo
    }
}
