use std::env;

use server::Server;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("localhost:8080".to_string());

    server.run(WebsiteHandler::new(public_path));
}

