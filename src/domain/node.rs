use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

pub type Node = Arc<Mutex<State>>;

#[derive(Debug, Default, Clone)]
pub struct State {
    pub node_id: String,
    pub id: usize,
    pub messages: HashSet<usize>,
    pub gossip_nodes: Vec<String>,
    pub topology: HashMap<String, Vec<String>>,
}

impl State {
    pub fn shared_default() -> Node {
        Arc::new(Mutex::new(Self::default()))
    }
}
