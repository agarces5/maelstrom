use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EchoOk {
    pub echo: String,
}

impl super::Type for EchoOk {
    fn from_msg<M>(_msg: Message<M>) -> Self
    where
        Self: Sized + Clone,
        M: super::Type + Clone,
    {
        todo!()
    }
}
