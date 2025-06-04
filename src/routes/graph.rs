use axum::{
    Router,
    routing::{delete, get, post, put},
};
use utoipa::OpenApi;

use crate::handlers::graph::{create_graph, delete_graph, get_graph, get_graphs, update_graph};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::graph::create_graph,
        crate::handlers::graph::get_graphs,
        crate::handlers::graph::get_graph,
        crate::handlers::graph::delete_graph,
        crate::handlers::graph::update_graph,
    ),
    tags(
        (name = "Graph", description = "Graph Management APIs")
    ),
)]

pub struct GraphApi;

pub fn graph_routes() -> Router {
    Router::new()
        .route("/create", post(create_graph))
        .route("/get", get(get_graphs))
        .route("/get/{id}", get(get_graph))
        .route("/delete/{id}", delete(delete_graph))
        .route("/update/{id}", put(update_graph))
}
