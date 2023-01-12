mod http;
use std::error;
use http::server::Server;

static SERVER_ADDR : &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let mut app = Server::new(SERVER_ADDR);
    app.start().await?;
    Ok(())
}
