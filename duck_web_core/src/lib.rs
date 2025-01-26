pub mod thread_pool;
pub mod http;
pub mod router;
pub mod handler;
pub mod duck_web;

pub use duck_web::DuckWeb;
pub use handler::RouteHandler;
pub use http::{Request, Response};
pub mod prelude {
    pub use crate::duck_web::DuckWeb;
    pub use crate::handler::RouteHandler;
    pub use crate::http::{Request, Response, Method};
}


