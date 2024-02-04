use std::sync::{mpsc::Sender, Arc, Mutex};

use crate::{message::Message, messages::*};

#[derive(Debug, Clone)]
pub struct CreateGossipOkUseCase {
    state: Arc<Mutex<State>>,
    sender: Sender<Message<MessageType>>,
}

impl CreateGossipOkUseCase {}

impl super::UseCase<Gossip> for CreateGossipOkUseCase {
    fn new(state: Arc<Mutex<State>>, sender: Sender<Message<MessageType>>) -> Self {
        Self { state, sender }
    }
    fn execute(&self, _msg: Message<MessageType>, msg_type: Gossip) {
        let Gossip { message } = msg_type;
        let mut state = self.state.lock().unwrap();
        state.messages.insert(message);
    }
}
