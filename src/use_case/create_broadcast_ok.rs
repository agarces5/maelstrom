use std::sync::{mpsc::Sender, Arc, Mutex};

use crate::{body::Body, message::Message, messages::*};

#[derive(Debug, Clone)]
pub struct CreateBroadcastOkUseCase {
    state: Arc<Mutex<State>>,
    sender: Sender<Message<MessageType>>,
}

impl CreateBroadcastOkUseCase {}

impl super::UseCase<Broadcast> for CreateBroadcastOkUseCase {
    fn new(state: Arc<Mutex<State>>, sender: Sender<Message<MessageType>>) -> Self {
        Self { state, sender }
    }
    fn execute(&self, msg: Message<MessageType>, msg_type: Broadcast) {
        let Broadcast { message } = msg_type;
        let mut state = self.state.lock().unwrap();
        state.messages.insert(message);
        let msg = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: msg.body.msg_id,
                in_reply_to: msg.body.msg_id,
                payload: MessageType::BroadcastOk(BroadcastOk),
            },
        };
        self.sender
            .send(msg)
            .expect("failed to send broadcast_ok message");
        for node_id in &state.gossip_nodes {
            self.sender
                .send(Message {
                    src: state.node_id.clone(),
                    dest: node_id.clone(),
                    body: Body {
                        msg_id: None,
                        in_reply_to: None,
                        payload: MessageType::Gossip(Gossip { message }),
                    },
                })
                .unwrap()
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        body::Body,
        controller::Controller,
        message::Message,
        messages::{Broadcast, BroadcastOk, MessageType, State},
    };

    #[test]
    fn it_should_return_a_broadcast_ok_message() {
        let body = Body {
            payload: MessageType::Broadcast(Broadcast { message: 1000 }),
            ..Default::default()
        };

        let req = Message::new("c1", "n1", &body);

        let (tx, rx) = std::sync::mpsc::channel();
        let controller = Controller::new(State::new_shared(), tx);
        let _ = controller.handle_request(req);

        let result = rx.recv().unwrap();

        assert_eq!(result.body.payload, MessageType::BroadcastOk(BroadcastOk));
    }
}
