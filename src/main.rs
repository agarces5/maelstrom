#![allow(dead_code)]

use std::io::{BufRead, Write};
use std::sync::mpsc::channel;

use crate::model::{Message, Node};

mod model;

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().lock();

    let (tx, rx) = channel();

    let mut node = Node::new(tx);

    std::thread::spawn(move || {
        let stdin = stdin.lock().lines();
        for line in stdin {
            let msg = serde_json::from_str::<Message>(&line.unwrap()).unwrap();
            let _res = node.reply(msg);
        }
    });

    for resp in rx {
        let _output = serde_json::to_writer(&mut stdout, &resp);
        let _output = stdout.write_all(b"\n");
    }

    Ok(())
}
