use hyper::{Body, Request, Response, StatusCode};
use http_body::Full;
use hyper::body::Bytes;
pub mod helpers;

//pub fn test_view(_req: Request<Body>) -> Response<Body> {
//    Response::builder()
//        .status(StatusCode::OK)
//        .body("It works!".into())
//        .unwrap()
//}

pub fn index(_req: Request<Body>) -> Response<Full<Bytes>> {
    helpers::render("templates/index.html","").unwrap()
}

//pub fn products(_req: Request<Body>) -> Response<Body> {
//    Response::builder()
//        .status(StatusCode::OK)
//        .body("It works!".into())
//        .unwrap()
//}

pub fn product(_req: Request<Body>) -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Full::new("This Works".into()))
        .unwrap()
}
