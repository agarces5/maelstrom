mod echo_use_case;
mod generate_use_case;
mod gossip_use_case;
mod init_use_case;
mod topology_use_case;

pub use echo_use_case::*;
pub use generate_use_case::*;
pub use gossip_use_case::*;
pub use init_use_case::*;
pub use topology_use_case::*;

use crate::domain::{Message, MsgType, Node};

pub trait Type {
    fn create_use_case(&self, node: Node) -> Box<dyn UseCase>;
}
pub trait UseCase {
    fn execute(
        &self,
        msg: Message<MsgType>,
        sender: std::sync::mpsc::Sender<String>,
    ) -> anyhow::Result<()>;
}
