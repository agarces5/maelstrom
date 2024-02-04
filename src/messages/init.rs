use serde::{Deserialize, Serialize};

use super::Type;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

impl Type for Init {
    fn from_msg<M>(_msg: crate::message::Message<M>) -> Self
    where
        Self: Sized + Clone,
        M: Type + Clone,
    {
        todo!()
    }
}
