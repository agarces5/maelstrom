use std::future::Future;

use std::pin::Pin;
use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

struct Request {
    pub message: String,
}

#[derive(Debug, Default)]
struct ResponseMessage {
    pub message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Creates the server
    let server = Server::new().await?;
    // Run the 'Handler'
    server.run(RequestHandler).await?;
    Ok(())
}

#[derive(Default)]
struct Server {}

impl Server {
    async fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }
    async fn run<H>(self, handler: H) -> anyhow::Result<()>
    where
        H: Handler<Request, Response = ResponseMessage> + Send + 'static,
        H::Future: Send,
    {
        let stdin = tokio::io::stdin();
        let reader = tokio::io::BufReader::new(stdin);
        let mut lines = reader.lines();
        let handler = Arc::new(Mutex::new(handler));

        while let Some(line) = lines.next_line().await? {
            let mut stdout = tokio::io::stdout();
            let request = parse_request(line.clone()).await?;
            let handler = handler.clone();
            tokio::task::spawn(async move {
                let response = handler.lock().await.call(request).await.unwrap_or_default();
                stdout.write_all(response.message.as_bytes()).await?;
                stdout.write_all(&[b'\n']).await?;
                stdout.flush().await?;
                Ok::<(), anyhow::Error>(())
            });
        }

        Ok(())
    }
}

async fn parse_request(line: String) -> anyhow::Result<Request> {
    Ok(Request { message: line })
}

trait Handler<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;
    fn call(&mut self, request: Request) -> Self::Future;
}

#[derive(Clone)]
struct RequestHandler;

impl Handler<Request> for RequestHandler {
    type Response = ResponseMessage;

    type Error = anyhow::Error;

    type Future = Pin<Box<dyn Future<Output = anyhow::Result<Self::Response>> + Send>>;
    fn call(&mut self, request: Request) -> Self::Future {
        Box::pin(async move {
            Ok(ResponseMessage {
                message: format!("Hello {}", request.message),
            })
        })
    }
}
