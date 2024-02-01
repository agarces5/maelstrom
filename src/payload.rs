use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use super::node::Node;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    #[default]
    InitOk,
    Generate,
    GenerateOk {
        id: String,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: HashSet<usize>,
    },
    Gossip {
        messages: HashSet<usize>,
    },
    GossipOk,
}

impl Payload {
    pub fn get_response_payload(&self, node: &mut Node) -> Self {
        match self {
            Payload::Echo { echo } => Payload::EchoOk {
                echo: echo.to_string(),
            },
            Payload::EchoOk { echo } => Payload::EchoOk {
                echo: echo.to_string(),
            },
            Payload::Init {
                node_id,
                node_ids: _ids,
            } => {
                node.node_id = node_id.to_owned();
                Payload::InitOk {}
            }
            Payload::InitOk {} => Payload::InitOk {},
            Payload::Generate {} => {
                node.id += 1;
                Payload::GenerateOk {
                    id: format!("{}-{}", node.node_id, node.id),
                }
            }
            Payload::GenerateOk { id } => Payload::GenerateOk { id: id.to_string() },
            Payload::Topology { topology } => {
                node.gossip_nodes
                    .extend_from_slice(topology.get(&node.node_id).unwrap());
                Payload::TopologyOk
            }
            Payload::TopologyOk => Payload::TopologyOk,
            Payload::Broadcast { message } => {
                node.messages.insert(*message);
                Payload::BroadcastOk
            }
            Payload::BroadcastOk => Payload::BroadcastOk,
            Payload::Read => Payload::ReadOk {
                // messages: node.messages.drain(..).collect(),
                messages: node.messages.clone(),
            },
            Payload::ReadOk { messages } => Payload::ReadOk {
                messages: messages.clone(),
            },
            Payload::Gossip { messages } => {
                node.messages.extend(messages);
                Payload::GossipOk
            }
            Payload::GossipOk => Payload::GossipOk,
        }
    }
}
