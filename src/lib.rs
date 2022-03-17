use std::{error, fmt::Display, ops::Deref};

pub mod domain;
pub mod presentation;
pub mod utils;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ErrorMsg {
    msg: String,
}

impl ErrorMsg {
    pub fn new(msg: &str) -> Self {
        let msg = String::from(msg);
        Self { msg }
    }
}

impl Deref for ErrorMsg {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.msg
    }
}

impl Display for ErrorMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for ErrorMsg {}

impl Default for ErrorMsg {
    fn default() -> Self {
        Self {
            msg: Default::default(),
        }
    }
}
