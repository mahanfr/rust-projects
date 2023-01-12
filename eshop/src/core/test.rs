
#[cfg(test)]
mod tests {
    use crate::UrlPattern;
    use crate::core::PathType;
    use hyper::{Body, Request, Response, StatusCode};
    use http_body::Full;
    use hyper::body::Bytes;

    fn get_view() -> fn(Request<Body>) -> Response<Full<Bytes>> {
        fn index(_req: Request<Body>) -> Response<Full<Bytes>> {
            Response::builder()
                .status(StatusCode::OK)
                .body(Full::new("This Works".into()))
                .unwrap()
        }
        return index
    }

    #[test]
    fn urlpattern_root_path(){
        let url_pattern = UrlPattern::new("/".to_string(),get_view());
        let path_expr: Vec<PathType> = vec![];
        assert_eq!(url_pattern.path_expr.len(),path_expr.len());
    }

    #[test]
    fn urlpattern_static_path(){
        let url_pattern = UrlPattern::new("/products/items/categories".to_string(),get_view());
        let path_expr: Vec<PathType> = vec![
            PathType::Static("products".to_string()),
            PathType::Static("items".to_string()),
            PathType::Static("categories".to_string())
        ];
        assert_eq!(url_pattern.path_expr.len(),path_expr.len());
        assert_eq!(path_expr[0],url_pattern.path_expr[0]);
        assert_eq!(path_expr[1],url_pattern.path_expr[1]);
        assert_eq!(path_expr[2],url_pattern.path_expr[2]);
    }

    #[test]
    fn urlpattern_dynamic_path(){
        let url_pattern = UrlPattern::new("/products/<id:num>/<sku:str>".to_string(),get_view());
        let path_expr: Vec<PathType> = vec![
            PathType::Static("products".to_string()),
            PathType::Number("id".to_string()),
            PathType::Str("sku".to_string())
        ];
        assert_eq!(url_pattern.path_expr.len(),path_expr.len());
        assert_eq!(path_expr[0],url_pattern.path_expr[0]);
        assert_eq!(path_expr[1],url_pattern.path_expr[1]);
        assert_eq!(path_expr[2],url_pattern.path_expr[2]);
    }

    #[test]
    #[should_panic]
    fn urlpattern_incorrect_path1(){
        let _url_pattern = UrlPattern::new("/products/<id>".to_string(),get_view());
    }
    
    #[test]
    #[should_panic]
    fn urlpattern_incorrect_path2(){
        let _url_pattern = UrlPattern::new("/products/<:id>".to_string(),get_view());
    }
    
    #[test]
    #[should_panic]
    fn urlpattern_incorrect_path3(){
        let _url_pattern = UrlPattern::new("/products/<:>".to_string(),get_view());
    }

    #[test]
    #[should_panic]
    fn urlpattern_incorrect_path4(){
        let _url_pattern = UrlPattern::new("/products/<>".to_string(),get_view());
    }

    #[test]
    fn urlpattern_is_valid(){
        let _url_pattern = UrlPattern::new("/products/<id:num>".to_string(),get_view());
        assert!(_url_pattern.is_valid_path("/products/5".to_string()));
        assert!(_url_pattern.is_valid_path("/products/0".to_string()));
        assert!(_url_pattern.is_valid_path("/products/1000000000000000000".to_string()));
        let _url_pattern = UrlPattern::new("/products/<sku:str>".to_string(),get_view());
        assert!(_url_pattern.is_valid_path("/products/hello".to_string()));
        assert!(_url_pattern.is_valid_path("/products/abc".to_string()));
        assert!(_url_pattern.is_valid_path("/products/900a".to_string()));
    }

}
