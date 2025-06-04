use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GraphSummary {
    pub uuid: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateGraphRequest {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}
