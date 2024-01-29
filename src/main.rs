use maelstrom::echo::Message;

fn main() {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();
    let replies = inputs.flatten().map(|msg| msg.reply());
    replies.for_each(|res| {
        let _ = serde_json::to_writer_pretty(&mut stdout, &res);
    });
}
