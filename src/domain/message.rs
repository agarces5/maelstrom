use serde::{Deserialize, Serialize};

use crate::domain::*;

use super::MsgType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<T> {
    src: String,
    dest: String,
    body: Body<T>,
}

impl<T: Clone> Message<T> {
    pub fn new(src: String, dest: String, body: Body<T>) -> Self {
        Self { src, dest, body }
    }

    pub fn src(&self) -> &str {
        &self.src
    }
    pub fn dest(&self) -> &str {
        &self.dest
    }
    pub fn body(&self) -> &Body<T> {
        &self.body
    }
    pub fn body_mut(&mut self) -> &mut Body<T> {
        &mut self.body
    }
    pub fn set_src(&mut self, src: &str) {
        self.src = src.to_owned();
    }
    pub fn set_dest(&mut self, dest: &str) {
        self.dest = dest.to_owned();
    }
    pub fn set_body(&mut self, body: Body<T>) {
        self.body = body;
    }
    pub fn generate_response(self) -> Self {
        Self {
            src: self.dest,
            dest: self.src,
            body: Body::new(
                self.body.msg_id().to_owned(),
                self.body.msg_id().to_owned(),
                self.body.payload().to_owned(),
            ),
        }
    }
}

impl<T: Default> From<Message<MsgType>> for Message<T> {
    fn from(value: Message<MsgType>) -> Self {
        Self {
            src: value.src,
            dest: value.dest,
            body: Body::new(value.body.msg_id, value.body.in_reply_to, T::default()),
        }
    }
}
