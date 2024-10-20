use anyhow::bail;
use serde::{Deserialize, Serialize};

use crate::model::{Message, MessageType};

use super::Request;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    id: usize,
    node_id: String,
    neighbors: Vec<String>,
    messages_buffer: Vec<usize>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            id: 0,
            node_id: String::new(),
            neighbors: Vec::default(),
            messages_buffer: Vec::default(),
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

    pub fn set_neighbors(&mut self, neighbors: &[String]) {
        self.neighbors = neighbors.to_vec()
    }

    pub fn neighbors(&self) -> &[String] {
        &self.neighbors
    }

    pub fn neighbors_mut(&mut self) -> &mut Vec<String> {
        &mut self.neighbors
    }

    pub fn messages_buffer_mut(&mut self) -> &mut Vec<usize> {
        &mut self.messages_buffer
    }

    pub fn reply(&mut self, req: Message) -> anyhow::Result<Message> {
        match req.body()._type() {
            MessageType::Request(request) => match request {
                Request::Init {
                    node_id,
                    node_ids: _,
                } => self.set_node_id(node_id),
                Request::Generate => self.increment_id(),
                Request::Topology { topology } => {
                    if let Some(neighbors) = topology.get(self.node_id()) {
                        self.set_neighbors(neighbors);
                    }
                }
                Request::Broadcast { message } => {
                    self.messages_buffer_mut().push(*message);
                }
                Request::Echo { .. } => {}
                Request::Read => {}
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
    use crate::model::{Message, Node};

    fn generate_response(req: &str) -> String {
        let mut node = Node::new();
        node.set_node_id("n1");
        let req = serde_json::from_str::<Message>(req).unwrap();
        let resp = node.reply(req).unwrap();

        serde_json::to_string(&resp).unwrap()
    }

    #[test]
    fn init_msg() {
        let init_msg = r#"{"body":{"type":"init","msg_id":1,"node_id":"n3","node_ids":["n1","n2","n3"]},"dest":"n1","src":"c1"}"#;
        let init_ok_msg =
            r#"{"src":"n1","dest":"c1","body":{"type":"init_ok","msg_id":1,"in_reply_to":1}}"#;
        let resp = generate_response(init_msg);
        assert_eq!(init_ok_msg, resp);
    }

    #[test]
    fn echo_msg() {
        let echo_msg =
            r#"{"body":{"type":"echo","msg_id":1,"echo":"Please echo 35"},"dest":"n1","src":"c1"}"#;
        let echo_ok_msg = r#"{"src":"n1","dest":"c1","body":{"type":"echo_ok","echo":"Please echo 35","msg_id":1,"in_reply_to":1}}"#;
        let resp = generate_response(echo_msg);
        assert_eq!(echo_ok_msg, resp);
    }

    #[test]
    fn generate_msg() {
        let generate_msg = r#"{"body":{"type":"generate","msg_id":1},"dest":"n1","src":"c1"}"#;
        let generate_ok_msg = r#"{"src":"n1","dest":"c1","body":{"type":"generate_ok","id":"n1-1","msg_id":1,"in_reply_to":1}}"#;
        let resp = generate_response(generate_msg);
        assert_eq!(generate_ok_msg, resp);
    }

    #[test]
    fn broadcast_msg() {
        let req = r#"{"src":"c1","dest":"n1","body":{"type":"broadcast","message":1,"msg_id":1}}"#;
        let ok_resp =
            r#"{"src":"n1","dest":"c1","body":{"type":"broadcast_ok","msg_id":1,"in_reply_to":1}}"#;

        let resp = generate_response(req);

        assert_eq!(ok_resp, resp);
    }

    #[test]
    fn read_msg() {
        let req = r#"{"src":"c1","dest":"n1","body":{"type":"read","msg_id":1}}"#;
        let ok_resp = r#"{"src":"n1","dest":"c1","body":{"type":"read_ok","messages":[],"msg_id":1,"in_reply_to":1}}"#;

        let resp = generate_response(req);

        assert_eq!(ok_resp, resp);
    }

    #[test]
    fn topology_msg() {
        let req = r#"{"src":"c1","dest":"n1","body":{"type":"topology","topology":{"n1":["n2","n3"],"n2":["n1"],"n3":["n1"]},"msg_id":1}}"#;
        let ok_resp =
            r#"{"src":"n1","dest":"c1","body":{"type":"topology_ok","msg_id":1,"in_reply_to":1}}"#;

        let mut node = Node::new();
        node.set_node_id("n1");

        let req = serde_json::from_str::<Message>(req).unwrap();
        let resp = node.reply(req).unwrap();
        let resp = serde_json::to_string(&resp).unwrap();

        assert_eq!(ok_resp, resp);
        assert_eq!(node.neighbors(), ["n2", "n3"]);
    }
}
