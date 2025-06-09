use axum::{Router, routing::post};
use utoipa::OpenApi;

use crate::handlers::workflow::run_workflow;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::workflow::run_workflow
    ),
    tags(
        (name = "Workflow", description = "Workflow Management APIs")
    ),
)]

pub struct WorkflowApi;

pub fn workflow_routes() -> Router {
    Router::new().route("/run", post(run_workflow))
}
