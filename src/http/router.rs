use super::request::Request;
use super::response::Response;

pub fn handle_request(request: &Request) -> Response {
    match (request.method, request.path) {
        ("GET", "/") => Response {
            status: 200,
            reason: "OK",
            body: "Hello, world!".to_string(),
        },
        ("GET", "/users") => Response {
            status: 200,
            reason: "OK",
            body: "Users page".to_string(),
        },
        _ => Response {
            status: 404,
            reason: "Not Found",
            body: "Not found".to_string(),
        },
    }
}