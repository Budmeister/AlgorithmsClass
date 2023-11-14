use crate::input;
use std::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(fmt::Debug)]
pub enum Error {
    InError(input::Error),
}
impl From<input::Error> for Error {
    fn from(value: input::Error) -> Self {
        Self::InError(value)
    }
}

