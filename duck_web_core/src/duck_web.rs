use serde_json::value::Serializer;

use crate::http::{Method, QueryString};
use crate::router::ROUTE_REGISTRY;
use crate::thread_pool::ThreadPool;
use crate::{Request, Response};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct DuckWeb {
    addr: String,
}

impl DuckWeb {
    pub fn new() -> Self {
        DuckWeb {
            addr: "127.0.0.1:7878".to_string(),
        }
    }

    pub fn bind<A: Into<String>>(mut self, addr: A) -> Self {
        self.addr = addr.into();
        self
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        let thread_pool_size = 4;
        let pool = ThreadPool::new(thread_pool_size);

        println!(
            "Server running with a thread pool size of {} at http://{}",
            thread_pool_size, self.addr
        );

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            pool.execute(|| {
                handle_connection(stream);
            });
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request_str = String::from_utf8_lossy(&buffer);
    let request_line = request_str.lines().next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    if parts.len() >= 2 {
        let method = Method::from(parts[0].to_uppercase().as_str());
        let full_path = parts[1];
        let (path, query_str) = match full_path.split_once('?') {
            Some((p, q)) => (p, q),
            None => (full_path, ""),
        };

        //todo: proper query string handling
        //let query = QueryString::from(query_str);
        let mut query_params = HashMap::new();

        let query = QueryString(query_params);

        let registry = ROUTE_REGISTRY.read();
        let mut matched = false;

        for route in registry.iter() {
            if let Some(path_params) = route.path_pattern().matches(path, &method) {
                let request = Request {
                    path: path.to_string(),
                    method: method.clone(),
                    headers: HashMap::new(),
                    body: Vec::new(),
                    path_params,
                    query,
                };
                let response = route.handle(request);
                stream.write(response.to_http().as_bytes());
                stream.flush().unwrap();
                //todo:
                //send_response(stream, response);
                matched = true;
                break;
            }
        }

        if !matched {
            // Send 404 or Method Not Allowed based on if path matches but method doesn't
            let status = if registry
                .iter()
                .any(|r| r.path_pattern().regex.is_match(path))
            {
                405 // Method Not Allowed
            } else {
                404 // Not Found
            };

            let response = Response {
                status,
                headers: HashMap::new(),
                body: "Not Found".to_string(),
            };
            stream.write(response.to_http().as_bytes());
            stream.flush().unwrap();
        }
    }
}
