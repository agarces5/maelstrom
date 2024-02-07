use crate::domain::{self, *};
pub struct EchoUseCase;

impl super::UseCase for EchoUseCase {
    fn execute(
        &self,
        msg: Message<MsgType>,
        sender: std::sync::mpsc::Sender<String>,
    ) -> anyhow::Result<()> {
        if let MsgType::Echo(echo) = msg.clone().body().payload() {
            let mut resp: Message<EchoOk> = msg.generate_response().into();
            resp.body_mut()
                .set_payload(EchoOk::new(echo.echo().to_string()));
            let resp = serde_json::to_string(&resp)?;
            sender.send(resp)?;
            Ok(())
        } else {
            Err(anyhow::Error::new(domain::Errors::Missmatch))
        }
    }
}
