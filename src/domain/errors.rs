use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum Errors {
    Missmatch,
}

impl Error for Errors {}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
