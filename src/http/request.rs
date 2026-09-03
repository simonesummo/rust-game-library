pub struct Request<'a> {
    pub method: &'a str,
    pub path: &'a str,
}

pub fn parse_request(request: &str) -> Result<Request<'_>, String> {
    let request_line = request
        .lines()
        .next()
        .ok_or("Request is empty")?;

    let mut parts = request_line.split_whitespace();

    let method = parts
        .next()
        .ok_or("Missing method")?;

    let path = parts
        .next()
        .ok_or("Missing path")?;

    Ok(Request { method, path })
}