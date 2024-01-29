use serde::{Deserialize, Serialize};

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
        topology: String,
    },
    TopologyOk,
    Broadcast {
        message: String,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<String>,
    },
}
// {:type (eq "topology"),
//  :topology {java.lang.String [java.lang.String]},
//  :msg_id Int}
// {:type (eq "topology_ok"),
//  #schema.core.OptionalKey{:k :msg_id} Int,
//  :in_reply_to Int}
