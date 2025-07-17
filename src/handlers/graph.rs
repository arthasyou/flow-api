use axum::{Extension, Json, http::StatusCode};
use service_utils_rs::services::http::{
    CommonError, CommonResponse, IntoCommonResponse,
    middleware::auth_mw::UserId,
    response::{CommonOk, Empty, ResponseResult},
};

use crate::{
    database::graph::{
        default_with_owner, delete_graph_by_id, get_graph_by_id, get_graphs_by_owner,
        update_graph_by_id,
    },
    error::error_code,
    models::graph::{
        CreateGraphRequest, CreateGraphResponse, GraphDetail, GraphSummary, UpdateGraphRequest,
    },
};

#[utoipa::path(
    post,
    path = "/create",
    request_body = CreateGraphRequest,
    responses(
        (status = 200, description = "Succeed", body = CommonResponse<CreateGraphResponse>),
        (status = 500, description = "Error", body = CommonError)
    ),
    description = "创建图表",
    tag = "Graph",
    security(("Bearer" = [])),
)]
pub async fn create_graph(
    Extension(UserId(user_id)): Extension<UserId>,
    Json(payload): Json<CreateGraphRequest>,
) -> ResponseResult<CreateGraphResponse> {
    // Validate the payload
    if payload.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(error_code::INVALID_PARAMS.into()),
        ));
    }

    let graph_id = default_with_owner(&user_id, &payload.name, &payload.description)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(error_code::SERVER_ERROR.into()),
            )
        })?;

    let res = CreateGraphResponse { id: graph_id }
        .into_common_response()
        .to_json();
    Ok(res)
}

#[utoipa::path(
    get,
    path = "/get",
    responses(
        (status = 200, description = "Succeed", body = CommonResponse<Vec<GraphSummary>>),
        (status = 500, description = "Error", body = CommonError)
    ),
    description = "获取图表列表",
    tag = "Graph",
    security(("Bearer" = [])),
)]
pub async fn get_graphs(
    Extension(UserId(user_id)): Extension<UserId>,
) -> ResponseResult<Vec<GraphSummary>> {
    let graphs = get_graphs_by_owner(&user_id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;
    let res = graphs.into_common_response().to_json();
    Ok(res)
}

#[utoipa::path(
    get,
    path = "/get/{id}",
    responses(
        (status = 200, description = "Succeed", body = CommonResponse<GraphDetail>),
        (status = 404, description = "Graph not found", body = CommonError),
        (status = 500, description = "Error", body = CommonError)
    ),
    description = "获取图表详情",
    tag = "Graph",
    security(("Bearer" = [])),
)]
pub async fn get_graph(
    Extension(UserId(user_id)): Extension<UserId>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> ResponseResult<GraphDetail> {
    let graph = get_graph_by_id(&id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    match graph {
        Some(g) if g.owner == user_id => {
            let g: GraphDetail = g.into();
            Ok(g.into_common_response().to_json())
        }
        _ => Err((
            StatusCode::NOT_FOUND,
            Json(error_code::GRAPH_NOT_FOUND.into()),
        )),
    }
}

#[utoipa::path(
    delete,
    path = "/delete/{id}",
    responses(
        (status = 200, description = "Succeed", body = CommonResponse<Empty>),
        (status = 404, description = "Graph not found", body = CommonError),
        (status = 500, description = "Error", body = CommonError)
    ),
    description = "删除图表",
    tag = "Graph",
    security(("Bearer" = [])),
)]
pub async fn delete_graph(
    Extension(UserId(user_id)): Extension<UserId>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> ResponseResult<Empty> {
    let graph = get_graph_by_id(&id).await.map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            Json(error_code::GRAPH_NOT_FOUND.into()),
        )
    })?;

    match graph {
        Some(g) if g.owner == user_id => {
            // Call the database function to delete the graph
            // Assuming a function `delete_graph_by_id` exists
            delete_graph_by_id(&id).await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(error_code::SERVER_ERROR.into()),
                )
            })?;
            Ok(CommonOk::default().to_json())
        }
        _ => Err((
            StatusCode::FORBIDDEN,
            Json(error_code::GRAPH_NOT_BELONG_TO_USER.into()),
        )),
    }
}

// 写一个update函数
#[utoipa::path(
    put,
    path = "/update/{id}",
    request_body = UpdateGraphRequest,
    responses(
        (status = 200, description = "Succeed", body = CommonResponse<Empty>),
        (status = 404, description = "Graph not found", body = CommonError),
        (status = 500, description = "Error", body = CommonError)
    ),
    description = "更新图表",
    tag = "Graph",
    security(("Bearer" = [])),
)]
pub async fn update_graph(
    Extension(UserId(user_id)): Extension<UserId>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<UpdateGraphRequest>,
) -> ResponseResult<Empty> {
    // First, fetch the graph and check ownership
    let graph = get_graph_by_id(&id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    match graph {
        Some(g) if g.owner == user_id => {
            update_graph_by_id(&id, payload.nodes, payload.edges)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(error_code::SERVER_ERROR.into()),
                    )
                })?;
            Ok(CommonOk::default().to_json())
        }
        Some(_) => Err((
            StatusCode::FORBIDDEN,
            Json(error_code::GRAPH_NOT_BELONG_TO_USER.into()),
        )),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(error_code::GRAPH_NOT_FOUND.into()),
        )),
    }
}
