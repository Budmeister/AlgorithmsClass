
use std::{io, num, fmt};

pub const DEBUG: bool = true;

#[derive(fmt::Debug)]
pub enum BWError {
    IOErr(io::Error),
    ParseIntErr(num::ParseIntError),
    ParseInstrErr(ParseInstrErr),
}
impl From<io::Error> for BWError {
    fn from(value: io::Error) -> Self {
        Self::IOErr(value)
    }
}
impl From<num::ParseIntError> for BWError {
    fn from(value: num::ParseIntError) -> Self {
        Self::ParseIntErr(value)
    }
}
impl From<ParseInstrErr> for BWError {
    fn from(value: ParseInstrErr) -> Self {
        Self::ParseInstrErr(value)
    }
}
impl fmt::Display for BWError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOErr(err) => write!(f, "{}", err),
            Self::ParseIntErr(err) => write!(f, "{}", err),
            Self::ParseInstrErr(err) => write!(f, "{}", err),
        }
    }
}

#[derive(fmt::Debug)]
pub struct ParseInstrErr(pub String);
impl fmt::Display for ParseInstrErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<&str> for ParseInstrErr {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}
impl From<String> for ParseInstrErr {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<num::ParseIntError> for ParseInstrErr {
    fn from(value: num::ParseIntError) -> Self {
        Self(value.to_string())
    }
}
