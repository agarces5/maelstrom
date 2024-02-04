use serde::{Deserialize, Serialize};

pub trait Type: Default + Clone + Send + Sync {}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub struct Echo {
    pub echo: String,
}
impl Type for Echo {}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub struct EchoOk {
    pub echo: String,
}
impl Type for EchoOk {}
