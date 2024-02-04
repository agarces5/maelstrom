use std::sync::{mpsc::Sender, Arc, Mutex};

use crate::{body::Body, message::Message, messages::*};

use super::UseCase;

#[derive(Debug, Clone)]
pub struct CreateEchoOkUseCase {
    state: Arc<Mutex<State>>,
    sender: Sender<Message<MessageType>>,
}

impl CreateEchoOkUseCase {}

impl UseCase<Echo> for CreateEchoOkUseCase {
    fn new(state: Arc<Mutex<State>>, sender: Sender<Message<MessageType>>) -> Self {
        Self { state, sender }
    }
    fn execute(&self, msg: Message<MessageType>, Echo { echo }: Echo) {
        let msg = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: msg.body.msg_id,
                in_reply_to: msg.body.msg_id,
                payload: MessageType::EchoOk(EchoOk { echo }),
            },
        };
        self.sender.send(msg).unwrap();
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    use crate::{
        body::Body,
        controller::Controller,
        message::Message,
        messages::{Echo, EchoOk, MessageType, State},
    };

    #[test]
    fn it_makes_an_echo_ok_resp() {
        let (tx, rx) = std::sync::mpsc::channel();
        let controller = Controller::new(Arc::new(Mutex::new(State::default())), tx);
        let body = Body::new(
            Some(0),
            None,
            MessageType::Echo(Echo {
                echo: "Please echo 35".to_owned(),
            }),
        );
        let req = Message::new("c1", "n1", &body);
        let res = Message::new(
            "n1",
            "c1",
            &Body {
                in_reply_to: body.msg_id,
                payload: MessageType::EchoOk(EchoOk {
                    echo: "Please echo 35".to_owned(),
                }),
                msg_id: Some(0),
            },
        );

        let result = controller.handle_request(req);

        assert!(result.is_ok());
        assert_eq!(res, rx.recv().unwrap());
    }
}
