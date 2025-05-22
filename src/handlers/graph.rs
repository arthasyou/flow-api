use std::collections::HashMap;

use axum::{Extension, Json, extract::Query, http::StatusCode};
use service_utils_rs::services::http::{
    CommonError, CommonResponse,
    response::{CommonOk, Empty, Result},
};

#[utoipa::path(
    get,
    path = "/create",
    responses(
        (status = 200, description = "Succeed", body = CommonResponse<Empty>),
        (status = 500, description = "Error", body = CommonError)
    ),
    description = "创建图表",
    tag = "创建图表"
)]
pub async fn create_graph(Query(_params): Query<HashMap<String, String>>) -> Result<Empty> {
    let res = CommonOk::default().to_json();
    Ok(res)
}
