use serde::{Deserialize, Serialize};

use crate::application::{GossipUseCase, Type, UseCase};
use crate::domain::Node;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Gossip {
    msg: usize,
}

impl Gossip {
    pub fn new(msg: usize) -> Self {
        Self { msg }
    }
    pub fn msg(&self) -> usize {
        self.msg
    }
}

impl Type for Gossip {
    fn create_use_case(&self, node: Node) -> Box<dyn UseCase> {
        Box::new(GossipUseCase::new(node))
    }
}
