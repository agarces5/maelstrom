use serde::{Deserialize, Serialize};

use super::body::Body;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

impl Message {
    pub fn new(src: &str, dest: &str, body: &Body) -> Self {
        Self {
            src: src.to_owned(),
            dest: dest.to_owned(),
            body: body.to_owned(),
        }
    }
}
