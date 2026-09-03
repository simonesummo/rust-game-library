pub struct Response {
    pub status: u16,
    pub reason: &'static str,
    pub body: String,
}

impl Response {
    pub fn to_http(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\n\
             Content-Length: {}\r\n\
             \r\n\
             {}",
            self.status,
            self.reason,
            self.body.len(),
            self.body
        )
    }
}