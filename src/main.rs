use server::Server;
use website_handler::WebsiteHandler;

mod http;
mod server;                          
mod website_handler;

fn main() {
    let default_path = env!("CARGO_MANIFEST_DIR");
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);
}
