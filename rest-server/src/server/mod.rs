use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;

struct UrlPattern {
    path: String,
    method: Method,
    view: fn(Request<Body>) -> Response<Body>,
}

impl UrlPattern {
    fn new(path: String, method: Method, view: fn(Request<Body>) -> Response<Body>) -> UrlPattern {
        UrlPattern {
            path,
            method,
            view,
        }
    }
}

fn view1(_req: Request<Body>) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body("It Works".into())
        .unwrap()
}

fn view2(_req: Request<Body>) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body("It is Working".into())
        .unwrap()
}

pub async fn service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut _url_patterns: Vec<UrlPattern> = Vec::new();
    _url_patterns.push(UrlPattern::new("/".to_string(),Method::GET,view1));
    _url_patterns.push(UrlPattern::new("/post".to_string(),Method::GET,view2));

    for pattern in _url_patterns {
        if req.uri().path() == pattern.path.as_str() && req.method() == pattern.method {
            return Ok((pattern.view)(req));
        } else if req.uri().path() == pattern.path.as_str() && req.method() != pattern.method {
            return Ok(Response::builder()
                .status(StatusCode::NOT_ACCEPTABLE)
                .body("Method Is Not Acceptable!".into())
                .unwrap());
        }
    }
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("Not Found!".into())
        .unwrap())
}

// pub async fn service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     let mut _paths: Vec<(&str, Method, bool)> = Vec::new();

//     for path in _paths {
//         if req.uri().path() == path.0 && req.method() == path.1 {
//             return Ok(Response::new("Hello world!".into()));
//         } else if req.uri().path() == path.0 && req.method() != path.1 {
//             return Ok(Response::builder()
//                 .status(StatusCode::NOT_ACCEPTABLE)
//                 .body("Method Is Not Acceptable!".into())
//                 .unwrap());
//         }
//     }
//     Ok(Response::builder()
//         .status(StatusCode::NOT_FOUND)
//         .body("Not Found!".into())
//         .unwrap())
// }
