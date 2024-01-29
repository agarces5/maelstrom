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
