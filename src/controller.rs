use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

use crate::message::Message;
use crate::messages::*;
use crate::use_case::*;

#[derive(Clone, Debug)]
pub struct Controller {
    sender: Sender<Message<MessageType>>,
    state: Arc<Mutex<State>>,
}

impl Controller {
    pub fn write(
        &self,
        res: Message<MessageType>,
        mut channel: impl std::io::Write,
    ) -> anyhow::Result<()> {
        serde_json::to_writer(&mut channel, &res)?;
        channel.write_all(&[b'\n'])?;
        Ok(())
    }
    /// Creates a new [`Controller`].
    pub fn new(state: Arc<Mutex<State>>, sender: Sender<Message<MessageType>>) -> Self {
        Self { state, sender }
    }
    pub fn handle_request(&self, msg: Message<MessageType>) -> anyhow::Result<()> {
        match msg.clone().body.payload {
            MessageType::Echo(echo) => {
                CreateEchoOkUseCase::new(self.state.clone(), self.sender.clone()).execute(msg, echo)
            }
            MessageType::Init(init) => {
                CreateInitOkUseCase::new(self.state.clone(), self.sender.clone()).execute(msg, init)
            }
            MessageType::Generate(generate) => {
                CreateGenerateOkUseCase::new(self.state.clone(), self.sender.clone())
                    .execute(msg, generate)
            }
            MessageType::Topology(topology) => {
                CreateTopologyOkUseCase::new(self.state.clone(), self.sender.clone())
                    .execute(msg, topology)
            }
            MessageType::Broadcast(broadcast) => {
                CreateBroadcastOkUseCase::new(self.state.clone(), self.sender.clone())
                    .execute(msg, broadcast)
            }
            MessageType::Read(read) => {
                CreateReadOkUseCase::new(self.state.clone(), self.sender.clone()).execute(msg, read)
            }
            MessageType::Gossip(gossip) => {
                CreateGossipOkUseCase::new(self.state.clone(), self.sender.clone())
                    .execute(msg, gossip)
            }
            _ => (),
        };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;

    use crate::{
        body::Body,
        controller::Controller,
        message::Message,
        messages::{Echo, MessageType, State},
    };

    #[test]
    fn it_should_write_resp_into_channel_provided() {
        let (tx, _rx) = channel();
        let body = Body::new(
            Some(1),
            None,
            MessageType::Echo(Echo {
                echo: "Please echo 35".to_owned(),
            }),
        );
        // Make Message to send into "Channel"
        let req = Message::new("c1", "n1", &body);
        // Response should be JSON in bytes
        let mut response = serde_json::to_string(&req).unwrap().into_bytes();
        // Add a newline at the end
        response.push(b'\n');

        // Create shared state
        let state = State::new_shared();

        // Create controller
        let controller = Controller::new(state.clone(), tx);

        // As send() accept a Writer, use a Cursor to mock an in-memory buffer that is cheaper than other Writer
        let mut channel = std::io::Cursor::new(Vec::new());
        // Send the message into buffer
        let result = controller.write(req, &mut channel);

        assert!(result.is_ok());
        // Channel should have the JSON as bytes + a newline.
        assert_eq!(channel.get_ref().to_owned(), response);
    }
}
