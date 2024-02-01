use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::mpsc::Sender,
};

use crate::payload::Payload;

use super::{body::Body, message::Message};

#[derive(Debug, Clone)]
pub struct Node {
    pub node_id: String,
    pub id: usize,
    pub messages: HashSet<usize>,
    pub tx: Option<Sender<Message>>,
    pub gossip_nodes: Vec<String>,
    pub topology: HashMap<String, Vec<String>>,
}

impl Node {
    pub fn handle_message(&mut self, msg: Message) -> anyhow::Result<()> {
        let mut res = Message {
            src: msg.dest.clone(),
            dest: msg.src.clone(),
            body: Body {
                in_reply_to: msg.body.msg_id,
                payload: Payload::InitOk,
                ..msg.body
            },
        };
        match &msg.body.payload {
            Payload::Echo { echo } => {
                res.body.payload = Payload::EchoOk {
                    echo: echo.to_string(),
                };
                self.send(res)?
            }
            Payload::EchoOk { echo: _ } => {}
            Payload::Init {
                node_id,
                node_ids: _,
            } => {
                self.node_id = node_id.to_owned();
                self.send(res)?
            }
            Payload::InitOk => {}
            Payload::Generate => {
                self.id += 1;
                res.body.payload = Payload::GenerateOk {
                    id: format!("{}-{}", self.node_id, self.id),
                };
                self.send(res)?
            }
            Payload::GenerateOk { id: _ } => {}
            Payload::Topology { topology } => {
                self.topology = topology.clone();
                self.gossip_nodes
                    .extend_from_slice(&topology.keys().cloned().collect::<Vec<String>>());
                self.gossip_nodes.retain(|node_id| node_id != &self.node_id);
                // self.gossip_nodes.extend_from_slice(
                //     topology
                //         .get(&self.node_id)
                //         .expect("Expect self node id to be in topology"),
                // );
                res.body.payload = Payload::TopologyOk;
                self.send(res)?
            }
            Payload::TopologyOk => {}
            Payload::Broadcast { message } => {
                self.messages.insert(*message);
                res.body.payload = Payload::BroadcastOk;
                self.send(res)?;
                for node_id in &self.gossip_nodes {
                    let msg = Message {
                        src: self.node_id.to_string(),
                        dest: node_id.to_string(),
                        body: Body {
                            payload: Payload::Gossip { message: *message },
                            ..Default::default()
                        },
                    };
                    self.send(msg)?;
                }
            }
            Payload::BroadcastOk => {}
            Payload::Read => {
                res.body.payload = Payload::ReadOk {
                    messages: self.messages.clone(),
                };
                self.send(res)?
            }
            Payload::ReadOk { messages: _ } => {}
            Payload::Gossip { message } => {
                self.messages.insert(*message);
                // TODO: We are not using responses right now
                // res.body.payload = Payload::GossipOk;
                // self.send(res)?;
            }
            Payload::GossipOk => {}
        }
        Ok(())
    }
    fn send(&self, msg: Message) -> anyhow::Result<()> {
        let tx = self.tx.as_ref().unwrap();
        tx.send(msg)?;
        Ok(())
    }
    pub fn write(&self, res: Message, mut channel: impl std::io::Write) -> anyhow::Result<()> {
        serde_json::to_writer(&mut channel, &res)?;
        channel.write_all(&[b'\n'])?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::{any::Any, io::Cursor, sync::mpsc::channel};

    use crate::payload::Payload;

    use super::*;

    #[test]
    fn it_should_write_resp_into_channel_provided() {
        let body = Body::new(
            Some(1),
            None,
            Payload::Echo {
                echo: "Please echo 35".to_owned(),
            },
        );
        // Make Message to send into "Channel"
        let req = Message::new("c1", "n1", &body);
        // Response should be JSON in bytes
        let mut response = serde_json::to_string(&req).unwrap().into_bytes();
        // Add a newline at the end
        response.push(b'\n');

        // Make a node that send messages
        let node = Node {
            node_id: String::from("n1"),
            id: 0,
            messages: HashSet::new(),
            tx: None,
            gossip_nodes: Vec::new(),
            topology: HashMap::new(),
        };
        // As send() accept a Writer, use a Cursor to mock an in-memory buffer that is cheaper than other Writer
        let mut channel = Cursor::new(Vec::new());
        // Send the message into buffer
        let result = node.write(req, &mut channel);

        assert!(result.is_ok());
        // Channel should have the JSON as bytes + a newline.
        assert_eq!(channel.get_ref().to_owned(), response);
    }
    #[test]
    fn it_makes_an_echo_ok_resp() {
        let (tx, rx) = channel();
        let mut node = Node {
            node_id: String::from("n1"),
            id: 0,
            messages: HashSet::new(),
            tx: Some(tx),
            gossip_nodes: Vec::new(),
            topology: HashMap::new(),
        };
        let body = Body::new(
            Some(1),
            None,
            Payload::Echo {
                echo: "Please echo 35".to_owned(),
            },
        );
        let req = Message::new("c1", "n1", &body);
        let res = Message::new(
            "n1",
            "c1",
            &Body {
                in_reply_to: body.msg_id,
                payload: Payload::EchoOk {
                    echo: "Please echo 35".to_owned(),
                },
                ..body
            },
        );

        let result = node.handle_message(req);

        assert!(result.is_ok());
        assert_eq!(res, rx.recv().unwrap());
    }
    #[test]
    fn it_makes_an_init_ok_resp() {
        let (tx, rx) = channel();
        let mut node = Node {
            node_id: String::from("n1"),
            id: 0,
            messages: HashSet::new(),
            tx: Some(tx),
            gossip_nodes: Vec::new(),
            topology: HashMap::new(),
        };
        let body = Body::new(
            Some(1),
            None,
            Payload::Init {
                node_id: "n3".to_string(),
                node_ids: vec!["n1".to_string(), "n2".to_string(), "n3".to_string()],
            },
        );
        let req = Message::new("c1", "n1", &body);
        let res = Message::new(
            "n1",
            "c1",
            &Body {
                in_reply_to: body.msg_id,
                payload: Payload::InitOk {},
                ..body
            },
        );

        let result = node.handle_message(req);

        assert!(result.is_ok());
        assert_eq!(res, rx.recv().unwrap());
    }
    #[test]
    fn it_should_generate_differents_id() {
        let (tx, rx) = channel();
        let mut node = Node {
            node_id: String::from("n1"),
            id: 0,
            messages: HashSet::new(),
            tx: Some(tx),
            gossip_nodes: Vec::new(),
            topology: HashMap::new(),
        };
        let body = Body {
            payload: Payload::Generate {},
            ..Default::default()
        };

        let req = Message::new("c1", "n1", &body);
        let ids = Vec::from_iter((1..6).map(|n| format!("n1-{}", n)));

        let mut result: Vec<String> = Vec::new();

        for _i in 0..5 {
            let msg = Message { ..req.clone() };
            let _ = node.handle_message(msg);
            if let Ok(msg) = rx.recv() {
                if let Payload::GenerateOk { id } = msg.body.payload {
                    result.push(id)
                }
            }
        }

        assert_eq!(ids, result);
    }
    #[test]
    fn it_should_return_a_broadcast_ok_message() {
        let body = Body {
            payload: Payload::Broadcast { message: 1000 },
            ..Default::default()
        };

        let req = Message::new("c1", "n1", &body);

        let (tx, rx) = channel();
        let mut node = Node {
            node_id: String::from("n1"),
            id: 0,
            messages: HashSet::new(),
            tx: Some(tx),
            gossip_nodes: Vec::new(),
            topology: HashMap::new(),
        };

        let _ = node.handle_message(req);

        let result = rx.recv().unwrap();

        assert_eq!(result.body.payload, Payload::BroadcastOk);
    }
    #[test]
    fn it_should_return_a_correct_read_ok_message() {
        let msgs = HashSet::from_iter([0, 25, 50, 75, 100]);
        let broadcast_messages = msgs.iter().map(|i| {
            Message::new(
                "c1",
                "n1",
                &Body {
                    payload: Payload::Broadcast {
                        message: msgs.get(i).unwrap().to_owned(),
                    },
                    ..Default::default()
                },
            )
        });

        let (tx, rx) = channel();
        let mut node = Node {
            node_id: String::from("n1"),
            id: 0,
            messages: HashSet::new(),
            tx: Some(tx),
            gossip_nodes: Vec::new(),
            topology: HashMap::new(),
        };

        broadcast_messages.for_each(|msg| {
            let _ = node.handle_message(msg);
            rx.recv().unwrap();
        });

        assert_eq!(node.messages, msgs);

        let read_msg = Message::new(
            "c1",
            "n1",
            &Body {
                payload: Payload::Read,
                ..Default::default()
            },
        );

        let result = node.handle_message(read_msg.clone());
        let read_ok_msg = rx.recv().unwrap();

        assert!(result.is_ok());
        assert_eq!(
            read_ok_msg.body.payload.type_id(),
            Payload::ReadOk {
                messages: HashSet::new()
            }
            .type_id()
        );
        assert_eq!(read_ok_msg.body.payload, Payload::ReadOk { messages: msgs });
    }
}
