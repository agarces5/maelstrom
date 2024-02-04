use std::sync::{mpsc::Sender, Arc, Mutex};

use crate::{body::Body, message::Message, messages::*};

use super::UseCase;

#[derive(Debug, Clone)]
pub struct CreateGenerateOkUseCase {
    sender: Sender<Message<MessageType>>,
    state: Arc<Mutex<State>>,
}
impl CreateGenerateOkUseCase {}

impl UseCase<Generate> for CreateGenerateOkUseCase {
    fn new(state: Arc<Mutex<State>>, sender: Sender<Message<MessageType>>) -> Self {
        Self { state, sender }
    }
    fn execute(&self, msg: Message<MessageType>, _msg_type: Generate) {
        let mut state = self.state.lock().expect("failed to lock state");
        let response = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: msg.body.msg_id,
                in_reply_to: msg.body.msg_id,
                payload: MessageType::GenerateOk(GenerateOk {
                    id: format! {"{}-{}", state.node_id, state.id},
                }),
            },
        };
        state.id += 1;
        self.sender.send(response).unwrap();
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    use crate::{
        body::Body,
        controller::Controller,
        message::Message,
        messages::{Generate, GenerateOk, MessageType, State},
    };

    #[test]
    fn it_should_generate_differents_id() {
        let (tx, rx) = std::sync::mpsc::channel();
        let state = Arc::new(Mutex::new(State {
            node_id: "n1".to_string(),
            ..Default::default()
        }));
        let controller = Controller::new(state, tx);
        let body = Body {
            payload: MessageType::Generate(Generate),
            ..Default::default()
        };

        let req = Message::new("c1", "n1", &body);
        let ids = Vec::from_iter((0..5).map(|n| format!("n1-{}", n)));

        let mut result: Vec<String> = Vec::new();

        for _i in 0..5 {
            let msg = Message { ..req.clone() };
            let _ = controller.handle_request(msg);
            if let Ok(msg) = rx.recv() {
                if let MessageType::GenerateOk(GenerateOk { id }) = msg.body.payload {
                    result.push(id)
                }
            }
        }

        assert_eq!(ids, result);
    }
}
