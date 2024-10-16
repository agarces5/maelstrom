use anyhow::bail;
use serde::{Deserialize, Serialize};

use crate::model::{Message, MessageType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    id: usize,
    node_id: String,
}

impl Node {
    pub fn new() -> Self {
        Self {
            id: 0,
            node_id: String::new(),
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn set_node_id(&mut self, node_id: &str) {
        self.node_id = node_id.to_string();
    }

    pub fn increment_id(&mut self) {
        self.id += 1;
    }

    pub fn generate_id(&self) -> String {
        format!("{}-{}", self.node_id, self.id)
    }

    pub fn reply(&mut self, req: Message) -> anyhow::Result<Message> {
        match req.body()._type() {
            MessageType::Request(request) => match request {
                super::Request::Init {
                    node_id,
                    node_ids: _,
                } => self.set_node_id(node_id),
                super::Request::Generate => self.increment_id(),
                _ => {}
            },
            MessageType::Response(_response) => bail!("Invalid message"),
            MessageType::Error { .. } => bail!("Something went wrong!"),
        }
        let resp = req.make_response(self);
        Ok(resp)
    }
}
