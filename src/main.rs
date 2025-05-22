mod error;
mod handlers;
mod models;
mod routes;
mod settings;

use service_utils_rs::services::http::http_server;
use settings::Settings;

#[tokio::main]
async fn main() {
    let settings = Settings::load("config/services.toml").unwrap();
    let router = routes::create_routes();
    let http_task = http_server::start(settings.http.port, router);

    let _ = tokio::join!(http_task);
}
