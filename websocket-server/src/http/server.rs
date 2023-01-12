use crate::http::{HttpVersion, Method, Request, Response};
use base64::{self, Engine};
use sha1::{Digest, Sha1};
use std::error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub struct Server {
    pub addr: String,
    pub clients: Vec<Box<tokio::net::TcpStream>>,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Server {
            addr: addr.to_string(),
            clients: vec![],
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn error::Error>> {
        let listener = TcpListener::bind(self.addr.as_str()).await?;
        //let mut active_sockets = Vec::<tokio::net::TcpStream>::new();
        loop {
            let (mut socket, socker_addr) = listener.accept().await?;
            //self.clients.push(Box::new(socket));
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                loop {
                    let n = match socket.read(&mut buf).await {
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            eprint!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };
                    let mut res: Response;
                    let request = &Request::parse(&buf[0..n]);
                    // TODO: create a middleware for connection preservation
                    if request.method == Method::GET && request.url == "/" {
                        let file = get_file("/index.html".to_string()).await;
                        res = Response::new(200, &file);
                        res.headers
                            .insert("Content-Type".to_string(), "text/html".to_string());
                    } else if request.method == Method::GET && request.is_file() {
                        if is_file_location(&request.url) {
                            let file = get_file((*request.url).to_string()).await;
                            res = Response::new(200, &file);
                            // res.headers.insert("Content-Type".to_string(), "text/html".to_string());
                        } else {
                            res = Response::not_found();
                        }
                    } else if request.method == Method::POST && request.url == "/" {
                        res = Response::new(200, b"Data Recived!");
                        res.headers
                            .insert("Content-Type".to_string(), "text/html".to_string());
                    } else if request.method == Method::GET && request.url == "/ws" {
                        match initiate_websocket_handshake(&request) {
                            Ok(response) => {
                                res = response;
                            }
                            Err(_) => {
                                res = Response::new(505, b"Unsupported Version (1.1 required)");
                            }
                        }
                    } else {
                        res = Response::not_found();
                    }
                    let bytes = res.to_u8_vec();
                    println!(
                        "[{:?}] {} {} {}",
                        request.method, request.url, res.status_code, socker_addr
                    );
                    if let Err(e) = socket.write_all(&bytes).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    }
}

pub fn initiate_websocket_handshake(req: &Request) -> Result<Response, ()> {
    if req.version != HttpVersion::V1_1 {
        return Err(());
    }
    if !req.headers.contains_key("Connection")
        || req.headers.get("Connection") != Some(&"Upgrade".to_string())
    {
        println!("No Connection header");
        return Err(());
    }
    if !req.headers.contains_key("Upgrade")
        || req.headers.get("Upgrade") != Some(&"websocket".to_string())
    {
        println!("No upgrade header");
        return Err(());
    }
    if !req.headers.contains_key("Sec-WebSocket-Key") {
        println!("No sec header");
        return Err(());
    }
    let ws_sec = req.headers.get("Sec-WebSocket-Key").unwrap();
    let ws_sec_new = generate_response_ws_sec(ws_sec);
    let mut res = Response::new(101, b"");
    res.headers
        .insert("Upgrade".to_string(), "websocket".to_string());
    res.headers
        .insert("Connection".to_string(), "Upgrade".to_string());
    res.headers
        .insert("Sec-WebSocket-Accept".to_string(), ws_sec_new.clone());
    Ok(res)
}

fn generate_response_ws_sec(sec: &String) -> String {
    let mut result: String = sec.clone();
    let magic_websocket_uuid_string = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
    result = result + magic_websocket_uuid_string;
    let mut hasher = Sha1::new();
    hasher.update(result);
    result = base64::engine::general_purpose::STANDARD.encode(hasher.finalize());
    result
}

fn is_file_location(path: &str) -> bool {
    let location: String = format!("{}{}", "public", path);
    if location.contains("..") {
        return false;
    }
    if std::path::Path::new(&location).is_file() {
        return true;
    }
    return false;
}

pub async fn get_file(url: String) -> Vec<u8> {
    let location: String = format!("{}{}", "public", url);
    if let Ok(contents) = std::fs::read(&location) {
        return contents;
    } else {
        return Vec::new();
    }
}
