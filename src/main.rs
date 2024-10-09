#![allow(dead_code)]

use serde_json::json;

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
fn main() {
    // Create a new json with the given data
    let raw_incoming_message = json!({
        "src": "c1",
        "dest": "n1",
        "body": {
            "type": "echo",
            "msg_id": 1,
            "echo": "Please echo 35"
        }
    });

    let raw_incoming_message = serde_json::to_string(&raw_incoming_message).unwrap();

    // Process the message
    let incoming_message = serde_json::from_str::<model::Message>(&raw_incoming_message).unwrap();

    // Create the response
    let response = incoming_message.reply();

    let response = serde_json::to_string(&response).unwrap();

    println!("{}", raw_incoming_message);
    println!("{}", response);
}
