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
}
