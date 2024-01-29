use serde::{Deserialize, Serialize};

use super::{body::Body, message::Message, payload::Payload};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node;

impl Node {
    pub fn reply(&self, req: Message) -> Message {
        let payload = match &req.body.payload {
            Payload::Echo { echo } => Payload::EchoOk {
                echo: echo.to_string(),
            },
            Payload::EchoOk { echo } => Payload::EchoOk {
                echo: echo.to_string(),
            },
            Payload::Init {
                node_id: _id,
                node_ids: _ids,
            } => Payload::InitOk {},
            Payload::InitOk {} => Payload::InitOk {},
            Payload::Generate {} => Payload::GenerateOk {},
            Payload::GenerateOk {} => Payload::GenerateOk {},
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

        let node = Node;

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

        let node = Node;

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
        let node = Node;
        // As send() accept a Writer, use a Cursor to mock an in-memory buffer that is cheaper than other Writer
        let mut channel = std::io::Cursor::new(Vec::new());
        // Send the message into buffer
        let result = node.send(req, &mut channel);

        assert!(result.is_ok());
        // Channel should have the JSON as bytes + a newline.
        assert_eq!(channel.get_ref().to_owned(), response);
    }
}
