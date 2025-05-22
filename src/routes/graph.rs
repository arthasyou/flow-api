use axum::{Router, routing::get};
use utoipa::OpenApi;

use crate::handlers::graph::create_graph;

#[derive(OpenApi)]
#[openapi(paths(crate::handlers::graph::create_graph))]

pub struct GraphApi;

pub fn graph_routes() -> Router {
    Router::new().route("/create", get(create_graph))
}
