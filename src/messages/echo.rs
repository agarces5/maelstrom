use serde::{Deserialize, Serialize};

use crate::message::Message;

use super::Type;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Echo {
    pub echo: String,
}

impl Type for Echo {
    fn from_msg<M>(_msg: Message<M>) -> Self
    where
        Self: Sized + Clone,
        M: Type + Clone,
    {
        todo!()
    }
}
