use crate::{body::Body, message::Message, messages::*};

pub struct CreateBroadcastOkUseCase;

impl CreateBroadcastOkUseCase {
    pub fn new() -> Self {
        Self
    }
}

impl super::UseCase for CreateBroadcastOkUseCase {
    fn execute(&self, msg: Message<MessageType>) {
        let _msg = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: msg.body.msg_id,
                in_reply_to: msg.body.msg_id,
                payload: BroadcastOk,
            },
        };
    }
}
