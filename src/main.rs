use std::{
    io::BufRead,
    str::FromStr,
    sync::{Arc, Mutex},
};

use maelstrom::{controller::Controller, message::Message, messages::State};

// fn main() -> anyhow::Result<()> {
//     let (tx, rx) = std::sync::mpsc::channel();

//     let node = Arc::new(Mutex::new(Node {
//         tx: Some(tx.clone()),
//         ..Default::default()
//     }));
//     let mut stdout = std::io::stdout().lock();

//     let resp_node = node.clone();
//     let resp_thread = std::thread::spawn(move || {
//         let node = resp_node;
//         let stdin = std::io::stdin().lock().lines();
//         for msg in stdin {
//             let mut node = node.lock().expect("Failed to adquire lock in node.");
//             let msg: Message<_> = Message::from_str(&msg?)?;
//             node.handle_message(msg)?;
//         }
//         anyhow::Ok(())
//     });
//     while let Ok(res) = rx.recv() {
//         node.lock().unwrap().write(res, &mut stdout)?;
//     }
//     resp_thread
//         .join()
//         .expect("Can't join response thread")
//         .expect("Some error inside resp thread");
//     Ok(())
// }
fn main() -> anyhow::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let state = State::new_shared();
    let controller = Arc::new(Mutex::new(Controller::new(state.clone(), tx.clone())));

    let mut stdout = std::io::stdout().lock();

    let thread_controller = controller.clone();
    let resp_thread = std::thread::spawn(move || {
        let controller = thread_controller;
        let stdin = std::io::stdin().lock().lines();
        for msg in stdin {
            let controller = controller.lock().unwrap();
            let msg: Message<_> = Message::from_str(&msg?)?;
            controller.handle_request(msg)?;
        }
        anyhow::Ok(())
    });
    while let Ok(res) = rx.recv() {
        controller.lock().unwrap().write(res, &mut stdout)?;
    }
    resp_thread
        .join()
        .expect("Can't join response thread")
        .expect("Some error inside resp thread");
    Ok(())
}
