mod database;
mod error;
mod handlers;
mod logging;
mod models;
mod routes;
mod settings;

use std::sync::Arc;

use service_utils_rs::services::{db::init_db, http::http_server, jwt::Jwt};
use settings::Settings;
use tracing::{error, info};

use crate::{database::create_tables, logging::init_tracing_to_file};

#[tokio::main]
async fn main() {
    init_tracing_to_file();
    info!("服务已启动");
    error!("错误日志示例");
    let settings = Settings::load("config/services.toml").unwrap();
    init_db(settings.surrealdb).await.unwrap();
    create_tables().await.unwrap();

    let jwt = Arc::new(Jwt::new(settings.jwt));
    let router = routes::create_routes(jwt);
    let http_task = http_server::start(settings.http.port, router);

    let _ = tokio::join!(http_task);
}
