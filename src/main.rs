use std::{
    io::BufRead,
    sync::{Arc, Mutex},
};

use maelstrom::echo::node::Node;

fn main() -> anyhow::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let node = Arc::new(Mutex::new(Node {
        id: 0,
        node_id: String::new(),
        messages: Vec::new(),
    }));
    let mut stdout = std::io::stdout().lock();

    let resp_node = node.clone();
    let resp_thread = std::thread::spawn(move || {
        let node = resp_node;
        let stdin = std::io::stdin().lock().lines();
        for msg in stdin {
            let msg = serde_json::from_str(&msg?)?;
            let res = node.lock().unwrap().reply(msg);
            eprintln!("Message received: {:?}", res);
            tx.send(res)?;
            // node.send(res, &mut stdout)?;
        }
        anyhow::Ok(())
    });
    while let Ok(res) = rx.recv() {
        eprintln!("{:?}", res);
        node.lock().unwrap().send(res, &mut stdout)?;
    }
    resp_thread
        .join()
        .expect("Can't join response thread")
        .expect("Some error inside resp thread");
    Ok(())
}
