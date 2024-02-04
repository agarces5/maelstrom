use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Default)]
pub struct State {
    pub node_id: String,
    pub id: usize,
    pub messages: HashSet<usize>,
    pub gossip_nodes: Vec<String>,
    pub topology: HashMap<String, Vec<String>>,
}

impl State {
    pub fn new(
        node_id: String,
        id: usize,
        messages: HashSet<usize>,
        gossip_nodes: Vec<String>,
        topology: HashMap<String, Vec<String>>,
    ) -> Self {
        Self {
            node_id,
            id,
            messages,
            gossip_nodes,
            topology,
        }
    }
    pub fn new_shared() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::default()))
    }
}
