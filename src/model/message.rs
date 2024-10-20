use serde::{Deserialize, Serialize};

use crate::model::{MessageType, Node, Request, Response};

use super::Body;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    src: String,
    dest: String,
    body: Body,
}

impl Message {
    pub fn new(src: &str, dest: &str, body: Body) -> Message {
        Message {
            src: src.to_string(),
            dest: dest.to_string(),
            body,
        }
    }

    pub fn src(&self) -> &str {
        &self.src
    }

    pub fn set_src(&mut self, src: &str) {
        self.src = src.to_string();
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn dest(&self) -> &str {
        &self.dest
    }

    pub fn set_dest(&mut self, dest: &str) {
        self.dest = dest.to_string();
    }

    pub fn make_response(&self, node: &mut Node) -> Self {
        let resp = match self.body._type() {
            MessageType::Request(req) => match req {
                Request::Echo { echo } => MessageType::Response(Response::EchoOk {
                    echo: echo.to_string(),
                }),
                Request::Init { .. } => MessageType::Response(Response::InitOk),
                Request::Generate => MessageType::Response(Response::GenerateOk {
                    id: node.generate_id(),
                }),
                Request::Topology { topology: _ } => MessageType::Response(Response::TopologyOk),
                Request::Read => MessageType::Response(Response::ReadOk {
                    // messages: node.messages_buffer_mut().drain(..).collect(),
                    messages: node.messages_buffer_mut().iter().cloned().collect(),
                }),
                Request::Broadcast { message: _ } => MessageType::Response(Response::BroadcastOk),
                Request::Gossip { message: _ } => MessageType::Error {
                    code: 12,
                    text: "Unexpected request".to_string(),
                },
            },
            MessageType::Response(resp) => MessageType::Error {
                code: 12,
                text: format!("{:?} is a response not a request!", resp),
            },
            MessageType::Error { code: _, text: _ } => todo!(),
        };
        let body = Body::new(resp, self.body().msg_id(), self.body().msg_id());

        Message::new(self.dest(), self.src(), body)
    }
}
