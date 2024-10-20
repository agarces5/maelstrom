use std::collections::HashSet;
use std::sync::mpsc::Sender;

use anyhow::bail;

use crate::model::{Body, Message, MessageType};

use super::Request;

#[derive(Debug, Clone)]
pub struct Node {
    id: usize,
    node_id: String,
    topology: HashSet<String>,
    neighbors: HashSet<String>,
    messages_buffer: HashSet<usize>,
    tx: Sender<Message>,
}

impl Node {
    pub fn new(tx: Sender<Message>) -> Self {
        Self {
            id: 0,
            node_id: String::new(),
            topology: HashSet::default(),
            neighbors: HashSet::default(),
            messages_buffer: HashSet::default(),
            tx,
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

    pub fn set_topology(&mut self, topology: &[String]) {
        self.topology.extend(topology.to_owned());
    }

    pub fn topology(&self) -> &HashSet<String> {
        &self.topology
    }

    pub fn topology_mut(&mut self) -> &mut HashSet<String> {
        &mut self.topology
    }

    pub fn set_neighbors(&mut self, neighbors: &[String]) {
        self.neighbors.extend(neighbors.to_owned());
    }

    pub fn neighbors(&self) -> &HashSet<String> {
        &self.neighbors
    }

    pub fn neighbors_mut(&mut self) -> &mut HashSet<String> {
        &mut self.neighbors
    }

    pub fn messages_buffer_mut(&mut self) -> &mut HashSet<usize> {
        &mut self.messages_buffer
    }

    pub fn reply(&mut self, req: Message) -> anyhow::Result<()> {
        match req.body()._type() {
            MessageType::Request(request) => match request {
                Request::Init { node_id, node_ids } => {
                    self.set_node_id(node_id);
                    let filter: Vec<String> = node_ids
                        .iter()
                        .filter(|&node| node != node_id)
                        .cloned()
                        .collect();
                    self.set_neighbors(&filter);
                }
                Request::Generate => self.increment_id(),
                Request::Topology { topology } => {
                    if let Some(topology) = topology.get(self.node_id()) {
                        self.set_topology(topology);
                    }
                }
                Request::Broadcast { message } => {
                    self.messages_buffer_mut().insert(*message);
                    for neighbour in self.neighbors() {
                        let msg = Message::new(
                            self.node_id(),
                            neighbour,
                            Body::new(
                                MessageType::Request(Request::Gossip {
                                    message: self.messages_buffer.clone(),
                                }),
                                None,
                                None,
                            ),
                        );
                        let _ = self.tx.send(msg);
                    }
                }
                Request::Gossip { message } => {
                    self.messages_buffer_mut().extend(message);
                    return Ok(());
                }
                Request::Echo { .. } => {}
                Request::Read => {}
            },
            MessageType::Response(_response) => bail!("Invalid message"),
            MessageType::Error { .. } => bail!("Something went wrong!"),
        }
        let resp = req.make_response(self);

        self.tx.send(resp.clone())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use crate::model::{Message, Node};

    fn generate_response(req: &str) -> String {
        let (tx, rx) = channel();
        let mut node = Node::new(tx);
        node.set_node_id("n1");
        let req = serde_json::from_str::<Message>(req).unwrap();
        node.reply(req).unwrap();
        let resp = rx.recv().expect("Expect to have response from node");

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

        let (tx, rx) = channel();
        let mut node = Node::new(tx);
        node.set_node_id("n1");

        let req = serde_json::from_str::<Message>(req).unwrap();
        node.reply(req).unwrap();
        let resp = rx.recv().expect("Expecto to have response from node");
        let resp = serde_json::to_string(&resp).unwrap();

        assert_eq!(ok_resp, resp);
        assert_eq!(
            node.topology().iter().cloned().collect::<Vec<String>>(),
            ["n2", "n3"].to_vec()
        );
    }
}
