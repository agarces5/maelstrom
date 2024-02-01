use std::io::BufRead;

use maelstrom::echo::node::Node;

fn main() -> anyhow::Result<()> {
    // let (tx, rx) = std::sync::mpsc::channel();

    let mut node = Node {
        id: 0,
        node_id: String::new(),
        messages: Vec::new(),
    };
    let stdin = std::io::stdin().lock().lines();
    let mut stdout = std::io::stdout().lock();

    for msg in stdin {
        let msg = serde_json::from_str(&msg?)?;
        let res = node.reply(msg);
        node.send(res, &mut stdout)?;
    }
    Ok(())
}
