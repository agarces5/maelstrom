use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BroadcastOk;

impl super::Type for BroadcastOk {
    fn from_msg<M>(_msg: crate::message::Message<M>) -> Self
    where
        Self: Sized + Clone,
        M: super::Type + Clone,
    {
        todo!()
    }
}
