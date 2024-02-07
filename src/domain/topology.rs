use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::application::{TopologyUseCase, Type, UseCase};

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Topology {
    pub topology: HashMap<String, Vec<String>>,
}

impl Topology {
    pub fn new(topology: HashMap<String, Vec<String>>) -> Self {
        Self { topology }
    }
}

impl Type for Topology {
    fn create_use_case(&self, node: super::Node) -> Box<dyn UseCase> {
        Box::new(TopologyUseCase::new(node))
    }
}
