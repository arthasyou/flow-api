mod graph;
mod workflow;

use std::sync::Arc;

use axum::{Extension, Router, middleware::from_fn};
use graph::{GraphApi, graph_routes};
use service_utils_rs::services::{
    http::middleware::{auth_mw::auth, cors::create_cors},
    jwt::Jwt,
};
use utoipa::{
    OpenApi,
    openapi::security::{ApiKey, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

use crate::routes::workflow::WorkflowApi;

#[derive(OpenApi)]
#[openapi(
        nest(
            (path = "/graph", api = GraphApi),
            (path = "/workflow", api = WorkflowApi)
        ),
    )]
struct ApiDoc;

pub fn create_routes(jwt: Arc<Jwt>) -> Router {
    let cors = create_cors();
    let mut doc = ApiDoc::openapi();
    doc.components
        .get_or_insert_with(Default::default)
        .add_security_scheme(
            "Bearer",
            SecurityScheme::ApiKey(ApiKey::Header(
                utoipa::openapi::security::ApiKeyValue::with_description(
                    "Authorization",
                    "请输入格式：Bearer <token>",
                ),
            )),
        );

    Router::new()
        .nest("/graph", graph_routes())
        .nest("/workflow", workflow::workflow_routes())
        .route_layer(from_fn(auth))
        .layer(Extension(jwt))
        .layer(cors)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc))
}
