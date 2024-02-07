use serde::{Deserialize, Serialize};

use crate::application::{EchoUseCase, Type, UseCase};
use crate::domain::Node;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Echo {
    echo: String,
}

impl Echo {
    pub fn new(echo: String) -> Self {
        Self { echo }
    }
    pub fn echo(&self) -> &str {
        &self.echo
    }
    pub fn set_echo(&mut self, echo: String) {
        self.echo = echo;
    }
}

impl Type for Echo {
    fn create_use_case(&self, node: Node) -> Box<dyn UseCase> {
        Box::new(EchoUseCase {})
    }
}
