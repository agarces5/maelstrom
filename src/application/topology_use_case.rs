use anyhow::bail;

use crate::domain::{self, GenerateOk, Message, MsgType, Node, Topology};

pub struct TopologyUseCase {
    node: Node,
}

impl TopologyUseCase {
    pub fn new(node: Node) -> Self {
        Self { node }
    }
}

impl super::UseCase for TopologyUseCase {
    fn execute(
        &self,
        msg: Message<MsgType>,
        sender: std::sync::mpsc::Sender<String>,
    ) -> anyhow::Result<()> {
        if let MsgType::Topology(topology) = msg.body().payload() {
            let mut node = self.node.lock().unwrap();
            node.topology = topology.topology.clone();
            let mut resp: Message<GenerateOk> = msg.generate_response().into();
            let resp = serde_json::to_string(&resp)?;
            sender.send(resp)?;
            Ok(())
        } else {
            Err(anyhow::Error::new(domain::Errors::Missmatch))
        }
    }
}
