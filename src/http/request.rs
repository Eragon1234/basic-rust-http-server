use super::method::Method;
use std::convert;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct Request {
    method: Method,
    path: String,
    query_string: Option<String>,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}