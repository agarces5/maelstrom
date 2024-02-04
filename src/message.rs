use std::str::FromStr;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::payload::MessageType;

use super::body::Body;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message<MessageType> {
    pub src: String,
    pub dest: String,
    pub body: Body<MessageType>,
}

impl<MessageType: Default + Clone> Message<MessageType> {
    pub fn new(src: &str, dest: &str, body: &Body<MessageType>) -> Self {
        Self {
            src: src.to_owned(),
            dest: dest.to_owned(),
            body: body.to_owned(),
        }
    }
    /// Generate a Message reply to another message, swapping received source and destination and setting "in_reply_to" field with the "msg_id" received.
    pub fn to_reply(&self) -> Self {
        Message {
            src: self.dest.clone(),
            dest: self.src.clone(),
            body: Body {
                in_reply_to: self.body.msg_id,
                ..Default::default()
            },
        }
    }

    pub fn with_id(mut self, id: usize) -> Self {
        self.body.msg_id = Some(id);
        self
    }

    pub fn with_payload(mut self, payload: MessageType) -> Self {
        self.body.payload = payload;
        self
    }
}

impl FromStr for Message<MessageType> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).context("Not a valid json provided")
    }
}
