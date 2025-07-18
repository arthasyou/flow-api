use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MarkerEnd {
    #[serde(rename = "type")]
    pub arrow_type: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_end: Option<MarkerEnd>,
    pub animated: bool,
}

impl From<Edge> for workflow_rs::model::graph_data::EdgeData {
    fn from(edge: Edge) -> Self {
        workflow_rs::model::graph_data::EdgeData {
            id: edge.id,
            source: edge.source,
            target: edge.target,
            source_handle: edge.source_handle,
            target_handle: edge.target_handle,
        }
    }
}
