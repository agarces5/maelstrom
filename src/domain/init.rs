use serde::{Deserialize, Serialize};

use crate::application::{InitUseCase, Type};
use crate::domain::Node;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "init")]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

impl Init {
    pub fn new(node_id: String, node_ids: Vec<String>) -> Self {
        Self { node_id, node_ids }
    }
}

impl Type for Init {
    fn create_use_case(&self, node: Node) -> Box<dyn crate::application::UseCase> {
        Box::new(InitUseCase::new(node))
    }
}
