use crate::domain::{self, *};

pub struct InitUseCase {
    node: Node,
}

impl InitUseCase {
    pub fn new(node: Node) -> Self {
        Self { node }
    }
}

impl super::UseCase for InitUseCase {
    fn execute(
        &self,
        msg: Message<MsgType>,
        sender: std::sync::mpsc::Sender<String>,
    ) -> anyhow::Result<()> {
        if let MsgType::Init(init) = msg.body().payload() {
            let mut node = self.node.lock().unwrap();
            node.node_id = init.node_id.clone();
            node.gossip_nodes = init.node_ids.clone();
            node.gossip_nodes.retain(|node_id| node_id != &init.node_id);
            let mut resp: Message<InitOk> = msg.generate_response().into();
            let resp = serde_json::to_string(&resp)?;
            sender.send(resp)?;
            Ok(())
        } else {
            Err(anyhow::Error::new(domain::Errors::Missmatch))
        }
    }
}
