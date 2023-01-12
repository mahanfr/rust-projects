use http_body::Full;
use hyper::body::Bytes;
use hyper::{Body, Request, Response};

mod test;

type View = fn(Request<Body>) -> Response<Full<Bytes>>;

#[derive(Debug,PartialEq)]
pub enum PathType{
    Static(String),
    Number(String),
    Str(String),
}

pub struct UrlPattern {
    pub view: View,
    path_expr: Vec<PathType>,
}

impl UrlPattern {
    pub fn new(path:String,view: View) -> UrlPattern {
        UrlPattern {
            view,
            path_expr : Self::get_url_pattern(path.as_str().to_string())
        }
    }

    pub fn is_valid_path(&self,path:String) -> bool {
        let split :Vec<&str> = path.split("/").filter(|&x| !x.is_empty()).collect();
        if self.path_expr.len() == split.len(){
            if path == "/" {
                return true;
            }
        }else{
            return false;
        }
        for i in 0..self.path_expr.len() {
            match &self.path_expr[i] {
                PathType::Static(lit) => {
                    if split[i] != lit {
                        return false;
                    }
                },
                PathType::Number(_) => {
                    if !(split[i].parse::<u64>().is_ok()) {
                        return false;
                    }
                },
                PathType::Str(_) => {
                    if split[i] == "" {
                        return false;
                    }
                },
            }
        }
        return true;
    }
    // /products/<id:int>/<name:str>/hello
    // vec => [ products - [0-9]* - u8 - hello ]
    fn get_url_pattern(path:String) -> Vec<PathType> {
        let mut pattern_vec = Vec::new();
        let path_split = path.split("/");
        // pattern_vec.push(PathType::Static("".to_string()));
        for s in path_split {
            if s != ""{
                if s.contains('<') && s.contains('>') {
                    let mut buffer_type : u8 = 0;
                    let mut name_buffer = String::new();
                    let mut type_buffer = String::new();
                    for c in s.chars(){
                        if c == '<' {
                            buffer_type = 1;
                        }else if c == ':'{
                            buffer_type = 2;
                        }else if c == '>' {
                            break;
                        }else{
                            if buffer_type == 1{
                                name_buffer.push(c);
                            }else if buffer_type == 2{
                                type_buffer.push(c);
                            }
                        }
                    }
                    if !name_buffer.is_empty() && !type_buffer.is_empty() {
                        if type_buffer == "num" {
                            pattern_vec.push(PathType::Number(name_buffer))
                        }else if type_buffer == "str" {
                            pattern_vec.push(PathType::Str(name_buffer))
                        }else {
                            panic!("invalid type for req param");
                        }
                    }else{
                        panic!("invalid dynamic path variable")
                    }
                }else{
                    pattern_vec.push(PathType::Static(s.to_string()));
                }
            }
        }
        return pattern_vec;
    }
}

