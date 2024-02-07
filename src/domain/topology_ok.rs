use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize)]
pub struct TopologyOk {}

impl TopologyOk {
    pub fn new() -> Self {
        Self {}
    }
}
