use std::io::{BufRead, Write};

use maelstrom::{domain::State, infrastructure::Controller};

fn main() -> anyhow::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let node = State::shared_default();

    let sender_node = node.clone();
    let sender_thread = std::thread::spawn(|| {
        let node = sender_node;
        let stdin = std::io::stdin().lock();
        let controller = Controller::new(tx, node);
        for line in stdin.lines() {
            let json_input = line?;
            controller.handle_request(json_input)?;
        }
        anyhow::Ok(())
    });

    let mut stdout = std::io::stdout();
    for msg in rx {
        stdout.write_all(msg.as_bytes())?;
        stdout.write_all(b"\n")?;
        node.lock().unwrap().id += 1;
    }

    sender_thread.join().unwrap()?;
    Ok(())
}
