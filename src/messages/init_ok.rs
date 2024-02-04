use serde::{Deserialize, Serialize};

use super::Type;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

pub struct InitOk;

impl Type for InitOk {}
