use serde::{Deserialize, Serialize};

use super::Type;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateOk {
    pub id: String,
}

impl Type for GenerateOk {}
