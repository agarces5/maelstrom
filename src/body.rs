use serde::{Deserialize, Serialize};

use crate::messages::Type;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Body<T: Type> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: T,
}

impl<T: Type> Body<T> {
    pub fn new(msg_id: Option<usize>, in_reply_to: Option<usize>, payload: T) -> Self {
        Self {
            msg_id,
            in_reply_to,
            payload,
        }
    }
}
