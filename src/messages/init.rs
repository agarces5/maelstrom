use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}
