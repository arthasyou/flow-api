use axum::{Extension, Json, http::StatusCode};
use service_utils_rs::services::http::{
    CommonError, CommonResponse, IntoCommonResponse,
    middleware::auth_mw::UserId,
    response::{Empty, ResponseResult},
};
use workflow_rs::{
    Workflow,
    graph::Graph,
    model::{DataPayload, data_payload::SingleData},
};

use crate::{
    database::graph::get_owner_graph_by_id,
    error::error_code,
    models::workflow::{RunWorkflowRequest, WorkflowOutput},
};

#[utoipa::path(
    post,
    path = "/run",
    request_body = RunWorkflowRequest,
    responses(
        (status = 200, description = "Succeed", body = CommonResponse<Empty>),
        (status = 404, description = "Graph not found", body = CommonError),
        (status = 500, description = "Error", body = CommonError)
    ),
    description = "运行工作流",
    tag = "Workflow",
    security(("Bearer" = [])),
)]
pub async fn run_workflow(
    Extension(UserId(user_id)): Extension<UserId>,
    Json(payload): Json<RunWorkflowRequest>,
) -> ResponseResult<WorkflowOutput> {
    let graph_db = get_owner_graph_by_id(&payload.id, &user_id)
        .await
        .map_err(|e| {
            println!("Error getting graph by ID: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(error_code::SERVER_ERROR.into()),
            )
        })?
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(error_code::GRAPH_NOT_FOUND.into()),
        ))?;

    let graph_data = graph_db.to_graph_data();
    println!("Graph data: {:#?}", graph_data);
    let graph = Graph::from(graph_data);
    println!("Graph: {:#?}", graph);

    let r = Workflow::start(graph).await.map_err(|e| {
        println!("Error executing workflow: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;
    println!("Graph execution result: {:?}", r);

    if let DataPayload::Single(SingleData::Text(r1)) = r {
        let r1 = WorkflowOutput { output: r1 };
        let res = r1.into_common_response().to_json();
        return Ok(res);
    }

    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(error_code::SERVER_ERROR.into()),
    ))
}
