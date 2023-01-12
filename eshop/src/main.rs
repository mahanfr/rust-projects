// Main
use hyper::server::conn::Http;
use http_body::Full;
use hyper::body::Bytes;
use hyper::service::service_fn;
use std::net::SocketAddr;
use tokio::join;
use tokio::net::TcpListener;

// Server
use hyper::{Body, Request, Response, StatusCode};
use std::convert::Infallible;

// Internal - macro
use colored::Colorize;
mod logger;

// Views
mod views;
mod core;
pub use crate::core::UrlPattern;

fn is_file_location(path:&str) -> bool {
    let location : String = format!("{}{}","static",path);
    if location.contains("..") {
        return false;
    }
    if std::path::Path::new(&location).is_file() {
        return true;
    }
    return false;
}

pub fn get_file(path:&str) -> Response<Full<Bytes>>{
    let location : String = format!("{}{}","static",path);
    if let Ok(contents) = std::fs::read(&location) {
        let body = contents.into();
        return Response::builder()
        .status(StatusCode::OK)
        .body(Full::new(body))
        .unwrap()
    }
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new("file not found".into()))
        .unwrap()
}

pub async fn service(req: Request<Body>) -> Result<Response<Full<Bytes>>,Infallible> {
    let mut _url_patterns: Vec<UrlPattern> = Vec::new();
    _url_patterns.push(UrlPattern::new("/items/<sku:str>".to_string(),views::product));
    _url_patterns.push(UrlPattern::new("/product/<id:num>".to_string(),views::product));
    _url_patterns.push(UrlPattern::new("/product/".to_string(),views::product));
    _url_patterns.push(UrlPattern::new("/".to_string(),views::index));

    info!("{} : {}",req.method(),req.uri().path());

    for pattern in _url_patterns {
        if pattern.is_valid_path(req.uri().path().to_string()) {
            return Ok((pattern.view)(req));
        }
    }

    if is_file_location(req.uri().path()){
        return Ok(get_file(req.uri().path()));
    }
    return Ok(Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body("404 Not Found".into())
      .unwrap());
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from("127.0.0.1:7000".to_string().parse::<SocketAddr>().unwrap());
    let server = async move{
        let listener = TcpListener::bind(addr).await.unwrap();
        loop{
            let (stream, _) = listener.accept().await.unwrap();
            tokio::task::spawn(async move {
                if let Err(err) = Http::new()
                    .serve_connection(stream,service_fn(service))
                    .await{
                        println!("Error Server Connection: {:?}",err);
                    }
            });
        }
    };
    let _ret = join!(server,);
}

