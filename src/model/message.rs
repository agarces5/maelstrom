use serde::{Deserialize, Serialize};

use super::{Body, MessageType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    src: String,
    dest: String,
    body: Body,
}

impl Message {
    pub fn new(src: String, dest: String, body: Body) -> Message {
        Message { src, dest, body }
    }

    pub fn src(&self) -> &str {
        &self.src
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn reply(&self) -> anyhow::Result<Message> {
        match self.body()._type() {
            MessageType::Echo { echo } => Ok(Message::new(
                self.dest.clone(),
                self.src.clone(),
                Body::new(
                    MessageType::EchoOk {
                        echo: echo.to_owned(),
                    },
                    self.body.msg_id(),
                    Some(self.body.msg_id()),
                ),
            )),
            MessageType::Init {
                node_id: _,
                node_ids: _,
            } => Ok(Message {
                src: self.dest.clone(),
                dest: self.src.clone(),
                body: Body::new(
                    MessageType::InitOk,
                    self.body.msg_id(),
                    Some(self.body.msg_id()),
                ),
            }),
            MessageType::Error { code, text } => {
                eprintln!("Code: {code}, Text: {text}");
                log::error!("Code: {code}, Text: {text}");
                Err(anyhow::anyhow!(
                    "Invalid data, just node is allowed to send Ok messages!"
                ))
            }
            _ => Err(anyhow::anyhow!(
                "Invalid data, just node is allowed to send Ok messages!"
            )),
        }
    }

    pub fn dest(&self) -> &str {
        &self.dest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reply() {
        let raw_resp = r#"{"src":"n1","dest":"c1","body":{"type":"echo_ok","in_reply_to":1,"msg_id":1,"echo":"Please echo 35"}}"#;
        let req =
            r#"{"body":{"echo":"Please echo 35","msg_id":1,"type":"echo"},"dest":"n1","src":"c1"}"#;
        let msj: Message = serde_json::from_str(req).unwrap();
        let reply = msj.reply().unwrap();
        let resp = serde_json::to_string(&reply).unwrap();

        assert_eq!(resp.len(), raw_resp.len());
        assert_eq!(
            resp,
            r#"{"src":"n1","dest":"c1","body":{"type":"echo_ok","echo":"Please echo 35","in_reply_to":1,"msg_id":1}}"#
        );
    }
}
