use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;

use crate::http::status_code::StatusCode;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<&str>) -> Self {
        Self {
            status_code,
            body: body.map(|s| s.to_string()),
        }
    }

    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason_phrase(), body)
    }
}
