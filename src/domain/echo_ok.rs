use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "echo_ok")]
pub struct EchoOk {
    echo: String,
}

impl EchoOk {
    pub fn new(echo: String) -> Self {
        Self { echo }
    }
}
