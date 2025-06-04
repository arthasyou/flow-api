mod database;
mod error;
mod handlers;
mod models;
mod routes;
mod settings;

use std::sync::Arc;

use service_utils_rs::services::{db::init_db, http::http_server, jwt::Jwt};
use settings::Settings;

use crate::database::create_tables;

#[tokio::main]
async fn main() {
    let settings = Settings::load("config/services.toml").unwrap();
    init_db(settings.surrealdb).await.unwrap();
    create_tables().await.unwrap();

    let jwt = Arc::new(Jwt::new(settings.jwt));
    let router = routes::create_routes(jwt);
    let http_task = http_server::start(settings.http.port, router);

    let _ = tokio::join!(http_task);
}
