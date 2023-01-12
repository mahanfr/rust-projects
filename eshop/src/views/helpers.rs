use http_body::Full; 
use std::error::Error;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};

pub fn render(template:&str,_context: &str) -> Result<Response<Full<Bytes>>,Box<dyn Error>> {
    // TODO: Add Template engine here
    Ok(get_file(template).unwrap())
}

pub fn get_file(filename: &str) -> Result<Response<Full<Bytes>>,Box<dyn Error>> {
    if let Ok(contents) = std::fs::read(filename) {
        let body = contents.into();
        return Ok(Response::new(Full::new(body)));
    }

    Ok(not_found())
}

pub fn not_found() -> Response<Full<Bytes>>{
    if let Ok(contents) = std::fs::read("resources/404.html") {
        let body = contents.into();
        return Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(body))
        .unwrap()
    }
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new("not found".into()))
        .unwrap()
}
