use crate::message::Message;
use crate::messages::*;
use crate::use_case::*;

#[derive(Default, Clone, Debug)]
pub struct Controller;

impl Controller {
    /// Creates a new [`Controller`].
    pub fn new() -> Self {
        Self
    }
    pub fn handle_request(msg: Message<MessageType>) {
        match msg.body.payload {
            MessageType::Echo(_) => CreateEchoOkUseCase::new().execute(msg),
            MessageType::Init(_) => CreateInitOkUseCase::new().execute(msg),
            MessageType::Generate(_) => CreateGenerateOkUseCase::new().execute(msg),
            MessageType::Topology(_) => CreateTopologyOkUseCase::new().execute(msg),
            MessageType::Broadcast(_) => CreateBroadcastOkUseCase::new().execute(msg),
            MessageType::Read(_) => CreateReadOkUseCase::new().execute(msg),
            MessageType::Gossip(_) => CreateGossipOkUseCase::new().execute(msg),
            _ => (),
        };
    }
}
