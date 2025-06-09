use axum::{Extension, Json, http::StatusCode};
use serde_json::{Value, json};
use service_utils_rs::services::http::{
    CommonError, CommonResponse, IntoCommonResponse,
    middleware::auth_mw::UserId,
    response::{Empty, ResponseResult},
};
use workflow_rs::{
    Workflow,
    graph::Graph,
    model::{
        DataPayload, Node,
        data_payload::SingleData,
        node::{ControlNode, DataNode, DataProcessorMapping, NodeType},
    },
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
    let graph_data = get_owner_graph_by_id(&payload.id, &user_id)
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
    // let mut graph = Graph::new();
    // for node_data in graph_data.nodes {
    //     let node_type = match node_data.kind.as_str() {
    //         "identity" => NodeType::Data(DataNode::Identity),
    //         "prompt" => NodeType::Data(DataNode::Prompt),
    //         "branch" => NodeType::Control(ControlNode::Branch),
    //         _ => {
    //             return Err((
    //                 StatusCode::BAD_REQUEST,
    //                 Json(error_code::SERVER_ERROR.into()),
    //             ));
    //         }
    //     };
    //     let node = Node::new(
    //         &node_data.id,
    //         node_type,
    //         node_data.data,
    //         DataProcessorMapping::default(),
    //         None, // Assuming no parent for simplicity
    //         None,
    //     );

    //     graph.add_node(node).unwrap();
    // }

    // let r = Workflow::start(graph).await.unwrap();

    let mut graph = Graph::new();

    // 定义节点 (不包含 Start 和 End)
    let nodes = vec![
        Node::new(
            "A",
            NodeType::Data(DataNode::Prompt),
            json!({ "template": "Node A Data" }),
            DataProcessorMapping::default(),
            None,
            None,
        ),
        Node::new(
            "B",
            NodeType::Data(DataNode::Prompt),
            json!({ "template": "Node B Data" }),
            DataProcessorMapping::default(),
            Some("input1".to_string()),
            Some("output1".to_string()),
        ),
        Node::new(
            "C",
            NodeType::Data(DataNode::Prompt),
            json!({ "template": "Node C Data" }),
            DataProcessorMapping::default(),
            None,
            None,
        ),
        Node::new(
            "D",
            NodeType::Data(DataNode::Identity),
            json!({}),
            DataProcessorMapping::default(),
            None,
            None,
        ),
        Node::new(
            "Control1",
            NodeType::Control(ControlNode::Branch),
            json!({
              "branches": {
                "A": "A",
                "B": "B"
              },
              "default": "C"
            }),
            DataProcessorMapping::default(),
            None,
            None,
        ),
    ];

    // 添加节点
    for node in nodes {
        graph.add_node(node).unwrap();
    }

    // 设置 Start 节点
    graph
        .set_start_node(Node::new(
            "start",
            NodeType::Data(DataNode::Input),
            serde_json::json!({
                "input": {
                    "Single": {
                        "Text": "A"
                    }
                }
            }),
            DataProcessorMapping::default(),
            None,
            None,
        ))
        .unwrap();

    // 设置 End 节点
    graph
        .set_end_node(Node::new(
            "end",
            NodeType::Data(DataNode::Identity),
            Value::Null,
            DataProcessorMapping::default(),
            None,
            None,
        ))
        .unwrap();

    // 添加边
    graph.add_edge("start", "Control1").unwrap();

    graph.add_edge("Control1", "A").unwrap();
    graph.add_edge("Control1", "B").unwrap();
    graph.add_edge("Control1", "C").unwrap();

    graph.add_edge("A", "D").unwrap();
    graph.add_edge("B", "D").unwrap();
    graph.add_edge("C", "D").unwrap();
    graph.add_edge("D", "end").unwrap();

    let r = Workflow::start(graph).await.unwrap();
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
