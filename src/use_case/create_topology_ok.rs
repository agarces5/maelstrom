use std::sync::{mpsc::Sender, Arc, Mutex};

use crate::{body::Body, message::Message, messages::*};

use super::UseCase;

#[derive(Debug, Clone)]
pub struct CreateTopologyOkUseCase {
    state: Arc<Mutex<State>>,
    sender: Sender<Message<MessageType>>,
}

impl CreateTopologyOkUseCase {}

impl UseCase<Topology> for CreateTopologyOkUseCase {
    fn new(state: Arc<Mutex<State>>, sender: Sender<Message<MessageType>>) -> Self {
        Self { state, sender }
    }
    fn execute(&self, msg: Message<MessageType>, _msg_type: Topology) {
        let msg = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: msg.body.msg_id,
                in_reply_to: msg.body.msg_id,
                payload: MessageType::TopologyOk(TopologyOk),
            },
        };
        self.sender.send(msg).unwrap();
    }
}
