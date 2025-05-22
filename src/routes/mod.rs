mod graph;

use axum::Router;
use graph::{GraphApi, graph_routes};
use service_utils_rs::services::http::middleware::cors::create_cors;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
        nest(
            (path = "/graph", api = GraphApi)
        ),
    )]
struct ApiDoc;

pub fn create_routes() -> Router {
    let cors = create_cors();
    let doc = ApiDoc::openapi();

    Router::new()
        .nest("/graph", graph_routes())
        .layer(cors)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc))
}
