use serde::{Deserialize, Serialize};

use super::body::Body;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

impl Message {
    pub fn new(src: &str, dest: &str, body: Body) -> Self {
        Self {
            src: src.to_owned(),
            dest: dest.to_owned(),
            body,
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::echo::payload::Payload;

    use super::*;

    #[test]
    fn it_works() {
        let req = json! {
            {
              "src": "c1",
              "dest": "n1",
              "body": {
                "msg_id": 1,
                "type": "echo",
                "echo": "Please echo 35"
              }
            }
        };
        let res = json! {
            {
              "src": "n1",
              "dest": "c1",
              "body": {
                "msg_id": 1,
                "in_reply_to": 1,
                "type": "echo_ok",
                "echo": "Please echo 35"
              }
            }
        };
        let serde_req = serde_json::from_value::<Message>(req).unwrap();
        let serde_res = serde_json::from_value::<Message>(res).unwrap();

        let msg_req = Message::new(
            "c1",
            "n1",
            Body::new(
                Some(1),
                None,
                Payload::Echo {
                    echo: "Please echo 35".to_string(),
                },
            ),
        );
        let msg_res = Message::new(
            "n1",
            "c1",
            Body::new(
                Some(1),
                Some(1),
                Payload::EchoOk {
                    echo: "Please echo 35".to_string(),
                },
            ),
        );
        assert_eq!(serde_req, msg_req);
        assert_eq!(serde_res, msg_res);
    }
}
