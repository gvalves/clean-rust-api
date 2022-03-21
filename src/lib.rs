use std::{fmt::Display, ops::Deref};

pub mod data;
pub mod domain;
pub mod infra;
pub mod presentation;
pub mod utils;

pub trait SyncError: std::error::Error + Send + Sync {}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ErrorMsg {
    msg: String,
}

impl ErrorMsg {
    pub fn new(msg: &str) -> Self {
        let msg = String::from(msg);
        Self { msg }
    }

    pub fn parse<T: std::error::Error>(err: T) -> Self {
        let msg = err.to_string();
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

impl std::error::Error for ErrorMsg {}

impl SyncError for ErrorMsg {}

impl Default for ErrorMsg {
    fn default() -> Self {
        Self {
            msg: Default::default(),
        }
    }
}

impl<T> From<ErrorMsg> for Result<T, Box<dyn std::error::Error>> {
    fn from(err: ErrorMsg) -> Self {
        Err(err.into())
    }
}

impl From<ErrorMsg> for Box<dyn SyncError> {
    fn from(err: ErrorMsg) -> Self {
        Box::new(err)
    }
}

impl<T> From<ErrorMsg> for Result<T, Box<dyn SyncError>> {
    fn from(err: ErrorMsg) -> Self {
        Err(err.into())
    }
}

pub type GenericResult<T = (), E = Box<dyn std::error::Error>> = Result<T, E>;
