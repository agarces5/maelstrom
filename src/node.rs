use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::mpsc::Sender,
};

use crate::{body::Body, message::Message, messages::*};

#[derive(Debug, Clone, Default)]
pub struct Node {
    pub node_id: String,
    pub id: usize,
    pub messages: HashSet<usize>,
    pub tx: Option<Sender<Message<MessageType>>>,
    pub gossip_nodes: Vec<String>,
    pub topology: HashMap<String, Vec<String>>,
}

impl Node {
    /// This function match the received message, mutating the state of the node and generating responses according with the message received.
    ///
    ///
    /// # Errors
    ///
    /// This function will return an error if the node can't send the responses to the mpsc Receiver.
    pub fn handle_message(&mut self, msg: Message<MessageType>) -> anyhow::Result<()> {
        if let Some(msgs) = self.make_responses(msg) {
            msgs.into_iter().try_for_each(|msg| -> anyhow::Result<()> {
                self.send(msg)?;
                Ok(())
            })
        } else {
            Ok(())
        }
    }
    /// Write a message as JSON in the channel provided.
    ///
    /// # Errors
    ///
    /// This function will return an error if can't serialize message provided as JSON or if can't write all the bytes into the channel.
    pub fn write(
        &self,
        res: Message<MessageType>,
        mut channel: impl std::io::Write,
    ) -> anyhow::Result<()> {
        serde_json::to_writer(&mut channel, &res)?;
        channel.write_all(&[b'\n'])?;
        Ok(())
    }

    /// Create responses according with the msg received and mutating the state of the node.
    /// With Init message, set the node_id and node_ids fields.
    /// With Topology message, set the topology field.
    /// With Broadcast message, push message received in the field messages.
    /// This function returns None if no responses are required for that msg.
    fn make_responses(&mut self, msg: Message<MessageType>) -> Option<Vec<Message<MessageType>>> {
        match msg.body.payload.clone() {
            MessageType::Echo(Echo { echo }) => {
                let res = msg
                    .to_reply()
                    .with_id(self.id)
                    .with_payload(MessageType::EchoOk(EchoOk { echo }));
                Some(vec![res])
            }
            MessageType::EchoOk(..) => None,
            MessageType::Init(Init { node_id, node_ids }) => {
                // Set node attributes
                (self.node_id, self.gossip_nodes) = (node_id, node_ids);
                self.gossip_nodes.retain(|node_id| node_id != &self.node_id);
                // Generate response
                let res = msg
                    .to_reply()
                    .with_id(self.id)
                    .with_payload(MessageType::InitOk);
                Some(vec![res])
            }
            MessageType::InitOk => None,
            MessageType::Generate(Generate) => {
                let payload = MessageType::GenerateOk(GenerateOk {
                    id: format!("{}-{}", self.node_id, self.id),
                });
                let res = msg.to_reply().with_id(self.id).with_payload(payload);
                Some(vec![res])
            }
            MessageType::GenerateOk(..) => None,
            MessageType::Topology(Topology { topology }) => {
                self.topology = topology;
                let res = msg
                    .to_reply()
                    .with_id(self.id)
                    .with_payload(MessageType::TopologyOk(TopologyOk));
                Some(vec![res])
            }
            MessageType::TopologyOk(TopologyOk) => None,
            MessageType::Broadcast(Broadcast { message }) => {
                self.messages.insert(message);
                let res = msg
                    .to_reply()
                    .with_id(self.id)
                    .with_payload(MessageType::BroadcastOk(BroadcastOk));
                let gossip_msgs: Vec<Message<MessageType>> = self
                    .gossip_nodes
                    .clone()
                    .iter()
                    .map(|node_id| Message {
                        src: self.node_id.to_string(),
                        dest: node_id.to_string(),
                        body: Body {
                            payload: MessageType::Gossip(Gossip { message }),
                            ..Default::default()
                        },
                    })
                    .collect();
                let mut res = vec![res];
                res.extend_from_slice(&gossip_msgs);
                Some(res)
            }
            MessageType::BroadcastOk(..) => None,
            MessageType::Read(..) => {
                let res = msg
                    .to_reply()
                    .with_id(self.id)
                    .with_payload(MessageType::ReadOk(ReadOk {
                        messages: self.messages.clone(),
                    }));
                Some(vec![res])
            }
            MessageType::ReadOk(..) => None,
            MessageType::Gossip(Gossip { message }) => {
                self.messages.insert(message);
                // TODO: We are not using responses right now
                // res.body.payload = Payload::GossipOk;
                // self.send(res)?;
                None
            }
            MessageType::GossipOk(..) => None,
        }
    }
    /// Send the message into the mpsc Receiver in main thread.
    ///
    /// # Panics
    ///
    /// Panics if the node have not got a mpsc Sender setted.
    ///
    /// # Errors
    ///
    /// This function will return an error if an error ocurred sending the message to the mpsc Receiver.
    fn send(&mut self, msg: Message<MessageType>) -> anyhow::Result<()> {
        let tx = self.tx.as_ref().unwrap();
        tx.send(msg)?;
        self.id += 1;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_write_resp_into_channel_provided() {
        let body = Body::new(
            Some(1),
            None,
            MessageType::Echo(Echo {
                echo: "Please echo 35".to_owned(),
            }),
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
            ..Default::default()
        };
        // As send() accept a Writer, use a Cursor to mock an in-memory buffer that is cheaper than other Writer
        let mut channel = std::io::Cursor::new(Vec::new());
        // Send the message into buffer
        let result = node.write(req, &mut channel);

        assert!(result.is_ok());
        // Channel should have the JSON as bytes + a newline.
        assert_eq!(channel.get_ref().to_owned(), response);
    }
    #[test]
    fn it_makes_an_init_ok_resp() {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut node = Node {
            node_id: String::from("n1"),
            tx: Some(tx),
            ..Default::default()
        };
        let body = Body::new(
            Some(0),
            None,
            MessageType::Init(Init {
                node_id: "n3".to_string(),
                node_ids: vec!["n1".to_string(), "n2".to_string(), "n3".to_string()],
            }),
        );
        let req = Message::new("c1", "n1", &body);
        let res = Message::new(
            "n1",
            "c1",
            &Body {
                in_reply_to: body.msg_id,
                payload: MessageType::InitOk {},
                ..body
            },
        );

        let result = node.handle_message(req);

        assert!(result.is_ok());
        assert_eq!(res, rx.recv().unwrap());
    }
    #[test]
    fn it_should_generate_differents_id() {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut node = Node {
            node_id: String::from("n1"),
            tx: Some(tx),
            ..Default::default()
        };
        let body = Body {
            payload: MessageType::Generate(Generate),
            ..Default::default()
        };

        let req = Message::new("c1", "n1", &body);
        let ids = Vec::from_iter((0..5).map(|n| format!("n1-{}", n)));

        let mut result: Vec<String> = Vec::new();

        for _i in 0..5 {
            let msg = Message { ..req.clone() };
            let _ = node.handle_message(msg);
            if let Ok(msg) = rx.recv() {
                if let MessageType::GenerateOk(GenerateOk { id }) = msg.body.payload {
                    result.push(id)
                }
            }
        }

        assert_eq!(ids, result);
    }
    #[test]
    fn it_should_return_a_broadcast_ok_message() {
        let body = Body {
            payload: MessageType::Broadcast(Broadcast { message: 1000 }),
            ..Default::default()
        };

        let req = Message::new("c1", "n1", &body);

        let (tx, rx) = std::sync::mpsc::channel();
        let mut node = Node {
            node_id: String::from("n1"),
            tx: Some(tx),
            ..Default::default()
        };

        let _ = node.handle_message(req);

        let result = rx.recv().unwrap();

        assert_eq!(result.body.payload, MessageType::BroadcastOk(BroadcastOk));
    }
    #[test]
    fn it_should_return_a_correct_read_ok_message() {
        let msgs = HashSet::from_iter([0, 25, 50, 75, 100]);
        let broadcast_messages = msgs.iter().map(|i| {
            Message::new(
                "c1",
                "n1",
                &Body {
                    payload: MessageType::Broadcast(Broadcast {
                        message: msgs.get(i).unwrap().to_owned(),
                    }),
                    ..Default::default()
                },
            )
        });

        let (tx, rx) = std::sync::mpsc::channel();
        let mut node = Node {
            node_id: String::from("n1"),
            tx: Some(tx),
            ..Default::default()
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
                payload: MessageType::Read(Read),
                ..Default::default()
            },
        );

        let result = node.handle_message(read_msg.clone());
        let read_ok_msg = rx.recv().unwrap();

        assert!(result.is_ok());
        assert_eq!(
            core::any::Any::type_id(&read_ok_msg.body.payload),
            core::any::Any::type_id(&MessageType::ReadOk(ReadOk {
                messages: HashSet::new()
            }))
        );
        assert_eq!(
            read_ok_msg.body.payload,
            MessageType::ReadOk(ReadOk { messages: msgs })
        );
    }
}
