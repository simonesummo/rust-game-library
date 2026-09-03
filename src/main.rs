mod http;
mod server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::run()
}