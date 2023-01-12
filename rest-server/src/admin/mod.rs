use http_body::Full;
use hyper::body::Bytes;
use std::error::Error;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;

pub async fn index2(req: Request<Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    if req.uri().path() == "/" && req.method() == Method::GET {
        return Ok(
            get_file("resources/index.html").await.unwrap()
        )
    }else if req.uri().path().starts_with("/static") && req.method() == Method::GET {
        return Ok(
            get_file(format!("resources{}",req.uri().path()).as_str()).await.unwrap()
        )
    }else if req.uri().path().starts_with("/fonts") && req.method() == Method::GET {
        return Ok(
            get_file(format!("resources/static{}",req.uri().path()).as_str()).await.unwrap()
        )
    }else if req.uri().path() == "/favicon.ico" && req.method() == Method::GET {
        return Ok(
            get_file("resources/favicon.ico").await.unwrap()
        )
    }
    Ok(not_found().await)
}

async fn get_file(filename: &str) -> Result<Response<Full<Bytes>>,Box<dyn Error>> {
    if let Ok(contents) = tokio::fs::read(filename).await {
        let body = contents.into();
        return Ok(Response::new(Full::new(body)));
    }

    Ok(not_found().await)
}

async fn not_found() -> Response<Full<Bytes>>{
    if let Ok(contents) = tokio::fs::read("resources/404.html").await {
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
