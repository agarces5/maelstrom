use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    sync::{Arc, Mutex},
};

use maelstrom::{message::Message, node::Node};

fn main() -> anyhow::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let node = Arc::new(Mutex::new(Node {
        id: 0,
        node_id: String::new(),
        messages: HashSet::new(),
        tx: Some(tx.clone()),
        gossip_nodes: Vec::new(),
        topology: HashMap::new(),
    }));
    let mut stdout = std::io::stdout().lock();

    let resp_node = node.clone();
    let resp_thread = std::thread::spawn(move || {
        let node = resp_node;
        let stdin = std::io::stdin().lock().lines();
        for msg in stdin {
            let mut node = node.lock().unwrap();
            let msg: Message = serde_json::from_str(&msg?)?;
            node.handle_message(msg)?;
        }
        anyhow::Ok(())
    });
    while let Ok(res) = rx.recv() {
        node.lock().unwrap().write(res, &mut stdout)?;
    }
    resp_thread
        .join()
        .expect("Can't join response thread")
        .expect("Some error inside resp thread");
    Ok(())
}
