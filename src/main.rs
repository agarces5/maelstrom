use maelstrom::echo::message::Message;
use maelstrom::echo::node::Node;

fn main() -> anyhow::Result<()> {
    let mut node = Node {
        id: 0,
        node_id: String::new(),
        messages: Vec::new(),
    };
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();
    for msg in inputs {
        let res = node.reply(msg?);
        node.send(res, &mut stdout)?;
    }
    Ok(())
}
