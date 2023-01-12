use std::{collections::HashMap};

pub mod server;
//pub use crate::http::server::{Server,Client};

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    OPTION,
    PUT,
    DELETE,
}
#[derive(Debug,PartialEq,Eq)]
pub enum HttpVersion {
    V1,
    V1_1,
    V2,
    V3,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Response {
    pub headers: HashMap<String, String>,
    pub version: HttpVersion,
    pub status_code: u32,
    pub reason_phrase: String,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status_code: u32, body: &[u8]) -> Self {
        let mut headers = HashMap::<String, String>::new();
        headers.insert("Content-Length".to_string(), body.len().to_string());
        Response {
            headers,
            version: HttpVersion::V1_1,
            status_code,
            reason_phrase: Self::default_reason_phrase(status_code).to_string(),
            body: body.to_vec(),
        }
    }

    pub fn not_found() -> Response {
        Response {
            headers: HashMap::new(),
            version: HttpVersion::V1_1,
            status_code: 404,
            reason_phrase: Self::default_reason_phrase(404).to_string(),
            body: Vec::new()
        }
    }

    pub fn to_u8_vec(self: &mut Self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        let mut ver_bytes = Self::version_to_bytes(&self.version);
        buffer.append(&mut ver_bytes);
        buffer.push(b' ');
        buffer.append(&mut self.status_code.to_string().as_bytes().to_vec());
        buffer.push(b' ');
        buffer.append(&mut Self::default_reason_phrase(self.status_code).to_string().as_bytes().to_vec());
        buffer.append(&mut [b'\r',b'\n'].to_vec());
        for (key,value) in &self.headers{
            buffer.append(&mut key.as_bytes().to_vec());
            buffer.push(b':');
            buffer.push(b' ');
            buffer.append(&mut value.as_bytes().to_vec());
            buffer.append(&mut [b'\r',b'\n'].to_vec());
        }
        buffer.append(&mut [b'\r',b'\n'].to_vec());
        buffer.append(&mut self.body);
        return buffer;
    }

    fn version_to_bytes(version: &HttpVersion) -> Vec<u8> {
        match version {
            HttpVersion::V1_1 => return b"HTTP/1.1".to_vec(),
            HttpVersion::V1 => return b"HTTP/1".to_vec(),
            HttpVersion::V2 => return b"HTTP/2".to_vec(),
            HttpVersion::V3 => return b"HTTP/3".to_vec()
        }
    }

    fn default_reason_phrase(status_code: u32) -> &'static str {
        match status_code {
            100 => return "Continue",
            101 => return "Switching Protocols",
            200 => return "OK",
            201 => return "Created",
            202 => return "Accepted",
            203 => return "Non-Authoritative Information",
            204 => return "No Content",
            205 => return "Reset Content",
            206 => return "Partial Content",
            300 => return "Multiple Choices",
            301 => return "Moved Permanently",
            302 => return "Found",
            303 => return "See Other",
            304 => return "Not Modified",
            305 => return "Use Proxy",
            307 => return "Temporary Redirect",
            308 => return "Permanent Redirect",
            400 => return "Bad Request",
            401 => return "Unauthorized",
            402 => return "Payment Required",
            403 => return "Forbidden",
            404 => return "Not Found",
            405 => return "Method Not Allowed",
            406 => return "Not Acceptable",
            407 => return "Proxy Authentication Required",
            408 => return "Request Timeout",
            409 => return "Conflict",
            410 => return "Gone",
            411 => return "Length Required",
            412 => return "Precondition Failed",
            413 => return "Content Too Large",
            414 => return "URI Too Long",
            415 => return "Unsupported Media Type",
            416 => return "Range Not Satisfiable",
            417 => return "Expectation Failed",
            421 => return "Misdirected Request",
            422 => return "Unprocessable Content",
            426 => return "Upgrade Required",
            500 => return "Internal Server Error",
            501 => return "Not Implemented",
            502 => return "Bad Gateway",
            503 => return "Service Unavailable",
            504 => return "Gateway Timeout",
            505 => return "HTTP Version Not Supported",
            _ => {
                return "Custom Protcol";
            }
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub headers: HashMap<String, String>,
    pub queries: HashMap<String, String>,
    pub method: Method,
    pub url: String,
    pub body: Vec<u8>,
    pub version: HttpVersion,
}
impl Default for Request {
    fn default() -> Self {
        Request {
            headers: HashMap::new(),
            queries: HashMap::new(),
            method: Method::GET,
            url: "/".to_owned(),
            body: Vec::new(),
            version: HttpVersion::V1_1,
        }
    }
}

impl Request {
    pub fn parse(buffer: &[u8]) -> Self {
        // GET / HTTP/1.1
        // Host: 127.0.0.1:8000
        // Connection: keep-alive
        let mut req = Request::default();
        let sections = buffer
            .windows(2)
            .enumerate()
            .filter(|(_, w)| matches!(*w, b"\r\n"))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        let mut j: usize = sections[0];
        let req_line = Self::parse_request_line(&buffer[0..j]);
        req.method = req_line.0;
        req.url = req_line
            .1
            .chars()
            .take_while(|&ch| ch != '?')
            .collect::<String>();
        if req.url.ends_with('/') && req.url.len() > 1 {
            req.url.pop();
        }
        req.queries = Self::parse_query(req.url.as_str());
        req.version = req_line.2;
        let mut headers = HashMap::<String, String>::new();
        for i in sections.clone() {
            if i == j {
                continue;
            }
            let line = &buffer[j + 2..i];
            if line.len() < 1 {
                continue;
            }
            let header_line = Self::parse_header_line(line);
            headers.insert(header_line.0, header_line.1);
            j = i;
        }
        req.headers = headers;
        req.body = (&buffer[sections.last().copied().unwrap() + 2..buffer.len()]).to_vec();
        return req;
    }

    pub fn is_file(self: &Self) -> bool {
        self.url.ends_with(".css") || self.url.ends_with(".ico") ||
        self.url.ends_with(".png") || self.url.ends_with(".svg") ||
        self.url.ends_with(".js")  || self.url.ends_with(".html")
    }

    fn parse_query(request_target: &str) -> HashMap<String, String> {
        let mut query = HashMap::new();

        // Split the request target into the path and the query string
        let mut path_and_query = request_target.split('?');
        if path_and_query.clone().count() == 1 {
            return query;
        }

        // Get the query string (if it exists)
        let query_string = path_and_query.nth(1);

        if let Some(query_string) = query_string {
            // Split the query string into individual parameters
            for param in query_string.split('&') {
                let mut key_value = param.split('=');
                let key = key_value.next().unwrap_or("");
                let value = key_value.next().unwrap_or("");
                query.insert(key.to_string(), value.to_string());
            }
        }

        query
    }

    fn parse_header_line(line: &[u8]) -> (String, String) {
        let mut iter = line.split(|c| *c == b':');
        let key = String::from_utf8_lossy(iter.next().unwrap()).to_string();
        let mut value = String::from_utf8_lossy(iter.next().unwrap()).to_string();
        if value.starts_with(' ') {
            value = value[1..value.len()].to_string();
        }
        return (key, value);
    }

    fn parse_request_line(line: &[u8]) -> (Method, String, HttpVersion) {
        let mut iter = line.split(|c| *c == b' ');
        let method = Self::parse_method(iter.next().unwrap()).unwrap();
        let url = String::from_utf8_lossy(iter.next().unwrap());
        let version = Self::parse_version(iter.next().unwrap()).unwrap();
        return (method, url.to_string(), version);
    }

    fn parse_method(chunk: &[u8]) -> Result<Method, ()> {
        match chunk {
            b"GET" => return Ok(Method::GET),
            b"POST" => return Ok(Method::POST),
            b"PUT" => return Ok(Method::PUT),
            b"DELETE" => return Ok(Method::DELETE),
            b"OPTION" => return Ok(Method::OPTION),
            _ => return Err(()),
        }
    }

    fn parse_version(chunk: &[u8]) -> Result<HttpVersion, ()> {
        match chunk {
            b"HTTP/1.1" => return Ok(HttpVersion::V1_1),
            b"HTTP/1" => return Ok(HttpVersion::V1),
            b"HTTP/2" => return Ok(HttpVersion::V2),
            b"HTTP/3" => return Ok(HttpVersion::V3),
            _ => return Err(()),
        }
    }
}
