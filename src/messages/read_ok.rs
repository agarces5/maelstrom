use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::Type;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReadOk {
    pub messages: HashSet<usize>,
}

impl Type for ReadOk {
    fn from_msg<M>(_msg: crate::message::Message<M>) -> Self
    where
        Self: Sized + Clone,
        M: Type + Clone,
    {
        todo!()
    }
}
