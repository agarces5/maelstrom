use serde::{Deserialize, Serialize};

use crate::{application::Type, domain::*};

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MsgType {
    Echo(Echo),
    EchoOk(EchoOk),
    Generate(Generate),
    GenerateOk(GenerateOk),
    Gossip(Gossip),
    Init(Init),
    InitOk(InitOk),
    Topology(Topology),
    TopologyOk(TopologyOk),
}

impl MsgType {
    pub fn msg_type(&self) -> Box<dyn Type> {
        match self {
            MsgType::Echo(inner) => Box::new(inner.clone()),
            // MsgType::EchoOk(inner) => Box::new(inner.clone()),
            MsgType::Init(inner) => Box::new(inner.clone()),
            // MsgType::InitOk(inner) => Box::new(inner.clone()),
            MsgType::Generate(inner) => Box::new(inner.clone()),
            // MsgType::GenerateOk(inner) => Box::new(inner.clone()),
            MsgType::Topology(inner) => Box::new(inner.clone()),
            // MsgType::TopologyOk(inner) => Box::new(inner.clone()),
            MsgType::Gossip(inner) => Box::new(inner.clone()),
            _ => panic!("Got unsupported message"),
        }
    }
}
