use crate::domain::{self, Generate, GenerateOk, Message, MsgType, Node};

pub struct GenerateUseCase {
    node: Node,
}

impl GenerateUseCase {
    pub fn new(node: Node) -> Self {
        Self { node }
    }
}

impl super::UseCase for GenerateUseCase {
    fn execute(
        &self,
        msg: crate::domain::Message<crate::domain::MsgType>,
        sender: std::sync::mpsc::Sender<String>,
    ) -> anyhow::Result<()> {
        if let MsgType::Generate(generate) = msg.clone().body().payload() {
            let node = self.node.lock().unwrap();
            let mut resp: Message<GenerateOk> = msg.generate_response().into();
            resp.body_mut()
                .set_payload(GenerateOk::new(&format!("{}-{}", node.node_id, node.id)));
            let resp = serde_json::to_string(&resp)?;
            sender.send(resp)?;
            Ok(())
        } else {
            Err(anyhow::Error::new(domain::Errors::Missmatch))
        }
    }
}
