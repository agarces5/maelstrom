use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageType {
    Request(Request),
    Response(Response),
    Error { code: u64, text: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Request {
    Echo {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    Generate,
    Topology {
        topology: HashMap<String, Vec<String>>, // NodeId, Vec<NodeId>
    },
    Read,
    Broadcast {
        message: usize,
    },
    Gossip {
        message: HashSet<usize>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum Response {
    EchoOk { echo: String },
    InitOk,
    GenerateOk { id: String },
    TopologyOk,
    ReadOk { messages: Vec<usize> },
    BroadcastOk,
}
