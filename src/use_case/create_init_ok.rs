use crate::{body::Body, message::Message, messages::*};

use super::UseCase;

pub struct CreateInitOkUseCase;

impl CreateInitOkUseCase {
    pub fn new() -> Self {
        Self
    }
}

impl UseCase for CreateInitOkUseCase {
    fn execute(&self, msg: Message<MessageType>) {
        let _msg = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: msg.body.msg_id,
                in_reply_to: msg.body.msg_id,
                payload: InitOk,
            },
        };
    }
}
