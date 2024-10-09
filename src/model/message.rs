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

    pub fn reply(&self) -> Message {
        let reply = Message::new(
            self.dest.clone(),
            self.src.clone(),
            Body::new(
                MessageType::EchoOk {
                    in_reply_to: self.body.msg_id(),
                },
                self.body.msg_id(),
                self.body().echo().to_string(),
            ),
        );

        reply
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
        let req =
            r#"{"body":{"echo":"Please echo 35","msg_id":1,"type":"echo"},"dest":"n1","src":"c1"}"#;
        let msj: Message = serde_json::from_str(req).unwrap();
        let reply = msj.reply();
        let resp = serde_json::to_string(&reply).unwrap();

        assert_eq!(
            resp,
            r#"{"src":"n1","dest":"c1","body":{"type":"echo_ok","in_reply_to":1,"msg_id":1,"echo":"Please echo 35"}}"#
        );
    }
}
