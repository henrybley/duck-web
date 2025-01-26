use parking_lot::RwLock;
use std::collections::HashMap;

use crate::{
    handler::RouteHandler,
    http::{Method, PathParams},
};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref ROUTE_REGISTRY: RwLock<Vec<Box<dyn RouteHandler>>> = RwLock::new(Vec::new());
}

#[derive(Clone)]
pub struct RoutePattern {
    pub pattern: String,
    pub regex: Regex,
    pub param_names: Vec<String>,
    pub method: Method,
}

impl RoutePattern {
    pub fn new(pattern: &str, method: &str) -> Self {
        let mut param_names = Vec::new();

        let regex_pattern = pattern
            .split('/')
            .map(|segment| {
                if segment.starts_with('{') && segment.ends_with('}') {
                    let param_name = segment[1..segment.len() - 1].to_string();
                    param_names.push(param_name.clone());
                    format!(r"(?P<{}>[\w-]+)", param_name)
                } else {
                    segment.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("/");

        RoutePattern {
            pattern: pattern.to_string(),
            regex: Regex::new(&format!("^{}$", regex_pattern)).unwrap(),
            param_names,
            method: Method::from(method),
        }
    }

    pub fn matches(&self, path: &str, method: &Method) -> Option<PathParams> {
        if self.method != *method {
            return None;
        }

        self.regex.captures(path).map(|caps| {
            let mut params = HashMap::new();
            for name in &self.param_names {
                if let Some(value) = caps.name(name) {
                    params.insert(name.clone(), value.as_str().to_string());
                }
            }
            PathParams(params)
        })
    }
}

pub fn register_route(route: Box<dyn RouteHandler>) {
    let mut registry = ROUTE_REGISTRY.write();
    registry.push(route);
}
