use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use workflow_rs::model::graph_data::GraphData;

use super::Node;
use crate::models::Edge;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Graph {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub start_node: String,
    pub end_node: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateGraphRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 500))]
    pub description: String,
}

#[derive(Debug, Serialize, ToSchema, Validate)]
pub struct CreateGraphResponse {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GraphSummary {
    pub uuid: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GraphDetail {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl From<Graph> for GraphDetail {
    fn from(graph: Graph) -> Self {
        GraphDetail {
            uuid: graph.uuid,
            name: graph.name,
            description: graph.description,
            nodes: graph.nodes,
            edges: graph.edges,
        }
    }
}

impl Graph {
    pub fn to_graph_data(self) -> GraphData {
        GraphData {
            nodes: self
                .nodes
                .into_iter()
                .map(workflow_rs::model::Node::from)
                .collect(),
            edges: self
                .edges
                .into_iter()
                .map(workflow_rs::model::graph_data::EdgeData::from)
                .collect(),
            start_node: Some(self.start_node),
            end_node: Some(self.end_node),
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateGraphRequest {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}
