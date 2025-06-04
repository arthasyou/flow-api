use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
}
