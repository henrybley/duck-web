use crate::http::{Request, Response};
use crate::router::RoutePattern;

pub trait RouteHandler: Send + Sync + 'static {
    fn path_pattern(&self) -> &RoutePattern;
    fn handle(&self, req: Request) -> Response;
}
