#![allow(dead_code)]

use std::io::{BufRead, Write};

use crate::model::{Message, Node};

mod model;

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock().lines();
    let mut stdout = std::io::stdout().lock();

    let mut node = Node::new();

    for line in stdin {
        let msg = serde_json::from_str::<Message>(&line?)?;
        let resp = node.reply(msg)?;
        let _output = serde_json::to_writer(&mut stdout, &resp);
        let _output = stdout.write_all(b"\n");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}
