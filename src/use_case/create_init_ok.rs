use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use crate::body::Body;
use crate::message::Message;
use crate::messages::*;

use super::UseCase;

#[derive(Debug, Clone)]
pub struct CreateInitOkUseCase {
    sender: Sender<Message<MessageType>>,
    state: Arc<Mutex<State>>,
}

impl UseCase<Init> for CreateInitOkUseCase {
    fn new(state: Arc<Mutex<State>>, sender: Sender<Message<MessageType>>) -> Self {
        Self { state, sender }
    }
    fn execute(&self, msg: Message<MessageType>, init: Init) {
        let mut state = self.state.lock().expect("failed to lock shared state");
        let Init { node_id, node_ids } = init;

        state.gossip_nodes = node_ids;
        state.gossip_nodes.retain(|id| id != &node_id);

        state.node_id = node_id;

        let msg = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: msg.body.msg_id,
                in_reply_to: msg.body.msg_id,
                payload: MessageType::InitOk,
            },
        };

        self.sender.send(msg).unwrap();
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    use serde_json::json;

    use crate::{
        body::Body,
        controller::Controller,
        message::Message,
        messages::{Init, MessageType, State},
    };

    #[test]
    fn it_makes_an_init_ok_resp() {
        let (tx, rx) = std::sync::mpsc::channel();
        let state = State {
            node_id: String::from("n1"),
            ..Default::default()
        };
        let controller = Controller::new(Arc::new(Mutex::new(state)), tx);

        let body = Body::new(
            Some(0),
            None,
            MessageType::Init(Init {
                node_id: "n3".to_string(),
                node_ids: vec!["n1".to_string(), "n2".to_string(), "n3".to_string()],
            }),
        );
        let req = json!({
            "src": "c1", "dest": "n1", "body": { "msg_id": 0, "type": "init", "node_id": "n1", "node_ids": ["n1","n2"] }
        });
        let req = serde_json::from_value(req).unwrap();
        let res = Message::new(
            "n1",
            "c1",
            &Body {
                in_reply_to: body.msg_id,
                payload: MessageType::InitOk {},
                ..body
            },
        );

        let result = controller.handle_request(req);
        assert!(result.is_ok());
        assert_eq!(res, rx.recv().unwrap());
    }
}
