mod create_broadcast_ok;
mod create_echo_ok;
mod create_generate_ok;
mod create_gossip_ok;
mod create_init_ok;
mod create_read_ok;
mod create_topology_ok;

use std::sync::{mpsc::Sender, Arc, Mutex};

pub use create_broadcast_ok::*;
pub use create_echo_ok::*;
pub use create_generate_ok::*;
pub use create_gossip_ok::*;
pub use create_init_ok::*;
pub use create_read_ok::*;
pub use create_topology_ok::*;

use crate::{
    message::Message,
    messages::{MessageType, State},
};

pub trait UseCase<T> {
    fn new(state: Arc<Mutex<State>>, sender: Sender<Message<MessageType>>) -> Self;
    fn execute(&self, msg: Message<MessageType>, msg_type: T);
}
