use hyper::server::conn::Http;
use hyper::service::service_fn;
use std::net::SocketAddr;
use tokio::join;
use tokio::net::TcpListener;

mod logger;
mod utils;
mod admin;
mod server;

#[macro_use]
extern crate lazy_static;

// TODO: check this line before publishing
const DEV: bool = true;

lazy_static! {
    #[derive(Debug)]
    static ref CONFIG : utils::AppConfig = utils::get_app_config().unwrap();
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(
        format!("{}:{}", CONFIG.server_addr, CONFIG.server_port)
            .parse::<SocketAddr>()
            .unwrap(),
    );
    let admin_addr = SocketAddr::from(
        format!("{}:{}", CONFIG.admin_addr, CONFIG.admin_port)
            .parse::<SocketAddr>()
            .unwrap(),
    );

    let api_server = async move {
        let listener = TcpListener::bind(addr).await.unwrap();
        loop {
            let (stream, _) = listener.accept().await.unwrap();

            tokio::task::spawn(async move {
                if let Err(err) = Http::new()
                    .serve_connection(stream, service_fn(server::service))
                    .await
                {
                    println!("Error Serving connection: {:?}", err);
                }
            });
        }
    };

    let admin_server = async move {
        let listener = TcpListener::bind(admin_addr).await.unwrap();
        loop {
            let (stream, _) = listener.accept().await.unwrap();

            tokio::task::spawn(async move {
                if let Err(err) = Http::new()
                    .serve_connection(stream, service_fn(admin::index2))
                    .await
                {
                    println!("Error serving connection: {:?}", err);
                }
            });
        }
    };

    logger::debug("Application is in development stage");
    logger::info(format!("Listening on http://{} and http://{}", addr, admin_addr).as_str());

    if CONFIG.debug == true {
        logger::warning("Application is in development stage");
    }

    let _ret = join!(api_server, admin_server);
}
