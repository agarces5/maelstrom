use std::sync::{mpsc::Sender, Arc, Mutex};

use crate::{body::Body, message::Message, messages::*};

use super::UseCase;

#[derive(Debug, Clone)]
pub struct CreateReadOkUseCase {
    state: Arc<Mutex<State>>,
    sender: Sender<Message<MessageType>>,
}

impl CreateReadOkUseCase {}

impl UseCase<Read> for CreateReadOkUseCase {
    fn new(state: Arc<Mutex<State>>, sender: Sender<Message<MessageType>>) -> Self {
        Self { state, sender }
    }
    fn execute(&self, msg: Message<MessageType>, _msg_type: Read) {
        let state = self.state.lock().unwrap();
        let msg = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: msg.body.msg_id,
                in_reply_to: msg.body.msg_id,
                payload: MessageType::ReadOk(ReadOk {
                    messages: state.messages.clone(),
                }),
            },
        };
        self.sender
            .send(msg)
            .expect("failed to send read_ok message");
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{
        body::Body,
        controller::Controller,
        message::Message,
        messages::{Broadcast, MessageType, Read, ReadOk, State},
    };

    #[test]
    fn it_should_return_a_correct_read_ok_message() {
        let msgs = HashSet::from_iter([0, 25, 50, 75, 100]);
        let broadcast_messages = msgs.iter().map(|i| {
            Message::new(
                "c1",
                "n1",
                &Body {
                    payload: MessageType::Broadcast(Broadcast {
                        message: msgs.get(i).unwrap().to_owned(),
                    }),
                    ..Default::default()
                },
            )
        });

        let (tx, rx) = std::sync::mpsc::channel();
        let state = State::new_shared();
        let controller = Controller::new(state.clone(), tx);

        broadcast_messages.for_each(|msg| {
            let _ = controller.handle_request(msg);
            rx.recv().unwrap();
        });

        assert_eq!(state.lock().unwrap().messages, msgs);

        let read_msg = Message::new(
            "c1",
            "n1",
            &Body {
                payload: MessageType::Read(Read),
                ..Default::default()
            },
        );

        let result = controller.handle_request(read_msg.clone());
        let read_ok_msg = rx.recv().unwrap();

        assert!(result.is_ok());
        assert_eq!(
            core::any::Any::type_id(&read_ok_msg.body.payload),
            core::any::Any::type_id(&MessageType::ReadOk(ReadOk {
                messages: HashSet::new()
            }))
        );
        assert_eq!(
            read_ok_msg.body.payload,
            MessageType::ReadOk(ReadOk { messages: msgs })
        );
    }
}
