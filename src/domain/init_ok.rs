use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "init_ok")]
pub struct InitOk {}

impl InitOk {
    pub fn new() -> Self {
        Self {}
    }
}
