mod broadcast;
mod broadcast_ok;
mod echo;
mod echo_ok;
mod generate;
mod generate_ok;
mod gossip;
mod gossip_ok;
mod init;
mod init_ok;
mod read;
mod read_ok;
mod state;
mod topology;
mod topology_ok;

pub use broadcast::*;
pub use broadcast_ok::*;
pub use echo::*;
pub use echo_ok::*;
pub use generate::*;
pub use generate_ok::*;
pub use gossip::*;
pub use gossip_ok::*;
pub use init::*;
pub use init_ok::*;
pub use read::*;
pub use read_ok::*;
pub use state::*;
pub use topology::*;
pub use topology_ok::*;

use serde::{Deserialize, Serialize};

use crate::message::Message;

pub trait Type {
    fn from_msg<M>(msg: Message<M>) -> Self
    where
        Self: Sized + Clone,
        M: Type + Clone;
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Echo(Echo),
    EchoOk(EchoOk),
    Init(Init),
    #[default]
    InitOk,
    Generate(Generate),
    GenerateOk(GenerateOk),
    Topology(Topology),
    TopologyOk(TopologyOk),
    Broadcast(Broadcast),
    BroadcastOk(BroadcastOk),
    Read(Read),
    ReadOk(ReadOk),
    Gossip(Gossip),
    GossipOk(GossipOk),
}

impl Type for MessageType {
    fn from_msg<M>(_msg: Message<M>) -> Self
    where
        Self: Sized + Clone,
        M: Type + Clone,
    {
        todo!()
    }
}
