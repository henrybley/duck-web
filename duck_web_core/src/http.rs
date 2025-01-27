use core::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "PATCH" => Method::PATCH,
            "HEAD" => Method::HEAD,
            "OPTIONS" => Method::OPTIONS,
            _ => Method::GET,
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub path_params: PathParams,
    pub query_params: QueryParams,
}

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn to_http(&self) -> String {
        format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status,
            self.body.len(),
            self.body
        )
    }
}

#[derive(Debug)]
pub struct PathParams {
    pub params: HashMap<String, String>,
}

#[derive(Debug)]
pub struct QueryParams {
    pub params: HashMap<String, Vec<String>>,
}

impl QueryParams {
    pub fn from(query_str: &str) -> Self {
        let mut params = HashMap::new();
        if query_str.len() > 0 {
            for segment in query_str.split('&') {
                let mut param = segment.split('=');
                let key = param.next().unwrap().to_string();
                let value = param.next().unwrap().to_string();
                params.entry(key).or_insert(Vec::new()).push(value);
            }
        };
        QueryParams { params }
    }
}
