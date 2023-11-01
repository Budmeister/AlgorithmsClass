use std::num::ParseIntError;

#[derive(Debug)]
pub enum OrdStatErr {
    IO(std::io::Error),
    ParseInt(ParseIntError),
    Args,
}
impl From<ParseIntError> for OrdStatErr {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}
impl From<std::io::Error> for OrdStatErr {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}
impl<T> From<OrdStatErr> for Result<T, OrdStatErr> {
    fn from(value: OrdStatErr) -> Self {
        Err(value)
    }
}