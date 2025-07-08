use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub handle: String,
}

impl From<Edge> for workflow_rs::model::graph_data::EdgeData {
    fn from(edge: Edge) -> Self {
        workflow_rs::model::graph_data::EdgeData {
            id: edge.id,
            start: edge.source,
            end: edge.target,
        }
    }
}
