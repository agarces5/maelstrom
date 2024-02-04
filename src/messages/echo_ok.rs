use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EchoOk {
    pub echo: String,
}

impl super::Type for EchoOk {}
