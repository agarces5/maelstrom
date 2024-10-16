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

#[cfg(test)]
mod tests {
    // use super::*;

    use crate::model::{Message, Node};

    #[test]
    fn init_msg() {
        let init_msg = r#"{"body":{"type":"init","msg_id":1,"node_id":"n3","node_ids":["n1","n2","n3"]},"dest":"n1","src":"c1"}"#;
        let mut node = Node::new();

        let req = serde_json::from_str::<Message>(init_msg).unwrap();
        let resp = node.reply(req).unwrap();
        let resp = serde_json::to_string(&resp).unwrap();

        let init_ok_msg =
            r#"{"src":"n1","dest":"c1","body":{"type":"init_ok","msg_id":1,"in_reply_to":1}}"#;
        assert_eq!(init_ok_msg, resp);
    }

    #[test]
    fn echo_msg() {
        let echo_msg =
            r#"{"body":{"type":"echo","msg_id":1,"echo":"Please echo 35"},"dest":"n1","src":"c1"}"#;
        let mut node = Node::new();

        let req = serde_json::from_str::<Message>(echo_msg).unwrap();
        let resp = node.reply(req).unwrap();
        let resp = serde_json::to_string(&resp).unwrap();

        let echo_ok_msg = r#"{"src":"n1","dest":"c1","body":{"type":"echo_ok","echo":"Please echo 35","msg_id":1,"in_reply_to":1}}"#;
        assert_eq!(echo_ok_msg, resp);
    }

    #[test]
    fn generate_msg() {
        let generate_msg = r#"{"body":{"type":"generate","msg_id":1},"dest":"n1","src":"c1"}"#;
        let mut node = Node::new();
        node.set_node_id("n1"); // Avoid to send init message to set node id.

        let req = serde_json::from_str::<Message>(generate_msg).unwrap();
        let resp = node.reply(req).unwrap();
        let resp = serde_json::to_string(&resp).unwrap();

        let generate_ok_msg = r#"{"src":"n1","dest":"c1","body":{"type":"generate_ok","id":"n1-1","msg_id":1,"in_reply_to":1}}"#;
        assert_eq!(generate_ok_msg, resp);
    }
}
