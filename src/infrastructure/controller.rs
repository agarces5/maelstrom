use std::{ops::Deref, sync::mpsc::Sender};

use crate::{
    // application::*,
    application::Type,
    domain::{Echo, Message, MsgType, Node},
};

pub struct Controller {
    sender: Sender<String>,
    node: Node,
}

impl Controller {
    pub fn new(sender: Sender<String>, node: Node) -> Self {
        Self { sender, node }
    }
    pub fn handle_request(&self, req: String) -> anyhow::Result<()> {
        let msg = serde_json::from_str::<Message<MsgType>>(&req)?;
        let payload = msg.body().payload();
        payload
            .msg_type()
            .create_use_case(self.node.clone())
            .execute(msg.clone(), self.sender.clone());
        Ok(())
    }
}
