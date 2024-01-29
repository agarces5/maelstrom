use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Echo { echo: String },
    EchoOk { echo: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

impl Message {
    pub fn reply(&self) -> Self {
        let echo = match &self.body.payload {
            Payload::Echo { echo } => echo.clone(),
            Payload::EchoOk { echo } => echo.clone(),
        };
        Message {
            src: self.dest.clone(),
            dest: self.src.clone(),
            body: Body {
                payload: Payload::EchoOk { echo },
                in_reply_to: self.body.msg_id,
                ..self.body
            },
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let req = r#"{
  "src": "c1",
  "dest": "n1",
  "body": {
    "msg_id": 1,
    "type": "echo",
    "echo": "Please echo 35"
  }
}"#;
        let res = r#"{
  "src": "n1",
  "dest": "c1",
  "body": {
    "msg_id": 1,
    "in_reply_to": 1,
    "type": "echo_ok",
    "echo": "Please echo 35"
  }
}"#;
        let message: Message = serde_json::from_str(req).unwrap();
        let reply = message.reply();
        assert_eq!(res, serde_json::to_string_pretty(&reply).unwrap());
    }
}
