use serde::{Deserialize, Serialize};

use super::{body::Body, message::Message, payload::Payload};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub node_id: String,
    pub id: usize,
    pub messages: Vec<String>,
}

impl Node {
    pub fn reply(&mut self, req: Message) -> Message {
        let payload = match &req.body.payload {
            Payload::Echo { echo } => Payload::EchoOk {
                echo: echo.to_string(),
            },
            Payload::EchoOk { echo } => Payload::EchoOk {
                echo: echo.to_string(),
            },
            Payload::Init {
                node_id,
                node_ids: _ids,
            } => {
                self.node_id = node_id.to_owned();
                Payload::InitOk {}
            }
            Payload::InitOk {} => Payload::InitOk {},
            Payload::Generate {} => {
                self.id += 1;
                Payload::GenerateOk {
                    id: format!("{}-{}", self.node_id, self.id),
                }
            }
            Payload::GenerateOk { id } => Payload::GenerateOk { id: id.to_string() },
            Payload::Topology { topology: _ } => Payload::TopologyOk,
            Payload::TopologyOk => todo!(),
            Payload::Broadcast { message } => {
                self.messages.push(message.to_owned());
                Payload::BroadcastOk
            }
            Payload::BroadcastOk => todo!(),
            Payload::Read => Payload::ReadOk {
                messages: self.messages.drain(..).collect(),
            },
            Payload::ReadOk { messages: _ } => todo!(),
        };
        Message {
            src: req.dest.clone(),
            dest: req.src.clone(),
            body: Body {
                payload,
                in_reply_to: req.body.msg_id,
                ..req.body
            },
        }
    }
    pub fn send(&self, res: Message, mut channel: impl std::io::Write) -> anyhow::Result<()> {
        serde_json::to_writer(&mut channel, &res)?;
        channel.write_all(&[b'\n'])?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use super::*;

    #[test]
    fn it_makes_an_echo_ok_resp() {
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

        let mut node = Node {
            node_id: String::from("n1"),
            id: 0,
            messages: Vec::new(),
        };

        assert_eq!(res, node.reply(req));
    }
    #[test]
    fn it_makes_an_init_ok_resp() {
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

        let mut node = Node {
            node_id: String::from("n1"),
            id: 0,
            messages: Vec::new(),
        };

        assert_eq!(res, node.reply(req));
    }
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
            messages: Vec::new(),
        };
        // As send() accept a Writer, use a Cursor to mock an in-memory buffer that is cheaper than other Writer
        let mut channel = std::io::Cursor::new(Vec::new());
        // Send the message into buffer
        let result = node.send(req, &mut channel);

        assert!(result.is_ok());
        // Channel should have the JSON as bytes + a newline.
        assert_eq!(channel.get_ref().to_owned(), response);
    }
    #[test]
    fn it_should_generate_differents_id() {
        let body = Body {
            payload: Payload::Generate {},
            ..Default::default()
        };

        let req = Message::new("c1", "n1", &body);
        let ids = Vec::from_iter((1..6).map(|n| format!("n1-{}", n)));

        let mut node = Node {
            node_id: String::from("n1"),
            id: 0,
            messages: Vec::new(),
        };

        let generated_ids: Vec<String> = (0..ids.len())
            .map(|_| match node.reply(req.clone()).body.payload {
                Payload::GenerateOk { id } => id,
                _ => String::new(),
            })
            .collect();
        assert_eq!(ids, generated_ids);
    }
    #[test]
    fn it_should_return_a_broadcast_ok_message() {
        let body = Body {
            payload: Payload::Broadcast {
                message: "1000".to_string(),
            },
            ..Default::default()
        };

        let req = Message::new("c1", "n1", &body);

        let mut node = Node {
            node_id: String::from("n1"),
            id: 0,
            messages: Vec::new(),
        };

        assert_eq!(node.reply(req).body.payload, Payload::BroadcastOk);
    }
    #[test]
    fn it_should_return_a_correct_read_ok_message() {
        let msgs = [
            "0".to_owned(),
            "25".to_owned(),
            "50".to_owned(),
            "75".to_owned(),
            "100".to_owned(),
        ];
        let broadcast_messages = (0..msgs.len()).map(|i| {
            Message::new(
                "c1",
                "n1",
                &Body {
                    payload: Payload::Broadcast {
                        message: msgs[i].to_string(),
                    },
                    ..Default::default()
                },
            )
        });

        let mut node = Node {
            node_id: String::from("n1"),
            id: 0,
            messages: Vec::new(),
        };

        broadcast_messages.for_each(|msg| {
            node.reply(msg);
        });

        assert_eq!(node.messages.as_ref(), msgs);

        let read_msg = Message::new(
            "c1",
            "n1",
            &Body {
                payload: Payload::Read,
                ..Default::default()
            },
        );

        let read_ok_msg = node.reply(read_msg.clone());

        assert_eq!(
            read_ok_msg.body.payload.type_id(),
            Payload::ReadOk {
                messages: Vec::new()
            }
            .type_id()
        );
        assert_eq!(
            read_ok_msg.body.payload,
            Payload::ReadOk {
                messages: msgs.to_vec()
            }
        );
        assert!(node.messages.is_empty());
    }
}
