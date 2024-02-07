use crate::domain::{self, *};
pub struct GossipUseCase {
    node: Node,
}

impl GossipUseCase {
    pub fn new(node: Node) -> Self {
        Self { node }
    }
}

impl super::UseCase for GossipUseCase {
    fn execute(
        &self,
        msg: Message<MsgType>,
        sender: std::sync::mpsc::Sender<String>,
    ) -> anyhow::Result<()> {
        if let MsgType::Gossip(gossip) = msg.body().payload() {
            self.node.lock().unwrap().messages.insert(gossip.msg());
            Ok(())
        } else {
            Err(anyhow::Error::new(domain::Errors::Missmatch))
        }
    }
}
