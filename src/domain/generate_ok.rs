use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "generate_ok")]
pub struct GenerateOk {
    id: String,
}

impl GenerateOk {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_owned() }
    }
}
