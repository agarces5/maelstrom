#![allow(dead_code)]

use std::io::{BufRead, Write};

use crate::model::Message;

mod model;

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock().lines();
    let mut stdout = std::io::stdout().lock();

    for line in stdin {
        let msg = serde_json::from_str::<Message>(&line?)?;
        let resp = msg.reply()?;
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
