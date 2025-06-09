use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RunWorkflowRequest {
    pub id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkflowOutput {
    pub output: String,
}
