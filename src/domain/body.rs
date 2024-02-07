use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<T> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: T,
}

impl<T> Body<T> {
    pub fn new(msg_id: Option<usize>, in_reply_to: Option<usize>, payload: T) -> Self {
        Self {
            msg_id,
            in_reply_to,
            payload,
        }
    }
    pub fn msg_id(&self) -> &Option<usize> {
        &self.msg_id
    }
    pub fn in_reply_to(&self) -> &Option<usize> {
        &self.in_reply_to
    }
    pub fn payload(&self) -> &T {
        &self.payload
    }
    pub fn set_payload(&mut self, payload: T) {
        self.payload = payload;
    }
}
