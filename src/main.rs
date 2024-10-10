#![allow(dead_code)]

use std::io::BufRead;

mod model;

// {
//   "src": "c1",
//   "dest": "n1",
//   "body": {
//     "type": "echo",
//     "msg_id": 1,
//     "echo": "Please echo 35"
//   }
// }
//
fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    // Read from stdin
    let stdin = std::io::stdin();
    let reader = std::io::BufReader::new(stdin);

    // let stdout = std::io::stdout();
    // let mut writer = std::io::BufWriter::new(stdout);

    for line in reader.lines() {
        let mut raw_incoming_message = String::default();
        match serde_json::to_string(&line?) {
            Ok(msg) => raw_incoming_message = msg,
            Err(e) => log::error!("{e}"),
        };

        match serde_json::from_str::<model::Message>(&raw_incoming_message) {
            Ok(incoming_message) => {
                let response = incoming_message.reply();
                let response = serde_json::to_string(&response).unwrap();
                // writer.write_all(response.as_bytes())?;
                println!("{}", &response);
            }
            Err(e) => {
                log::error!("Failed to serialize incoming data into Message model with error: {e}")
            }
        };
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}
