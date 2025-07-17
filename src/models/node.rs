use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use utoipa::ToSchema;
use workflow_rs::model::node::DataProcessorMapping;

use crate::utils::graph::get_workflow_node_type;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NodeData {
    pub label: String,
    pub description: Option<String>,
    #[serde(default)]
    pub payload: Value,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Node {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub data: NodeData,
    pub position: Position,
}

impl From<Node> for workflow_rs::model::Node {
    fn from(node: Node) -> Self {
        let node_type = get_workflow_node_type(&node.kind);
        workflow_rs::model::Node {
            id: node.id,
            node_type: node_type,
            data: node.data.payload,
            processors: DataProcessorMapping::default(),
            input_id: None,
            output_id: None,
        }
    }
}

impl Node {
    pub fn new_start_node() -> Self {
        let data = NodeData {
            label: "Start Node".to_string(),
            description: None,
            payload: json!({
                "input": {
                    "type": "Single",
                    "value": {
                        "type": "Text",
                        "value": ""
                    }
                }
            }),
        };
        Node {
            id: "start".to_string(),
            kind: "input".to_string(),
            data: data,
            position: Position::new(0.0, 0.0),
        }
    }

    pub fn new_end_node() -> Self {
        let data = NodeData {
            label: "End Node".to_string(),
            description: None,
            payload: Value::Null,
        };
        Node {
            id: "end".to_string(),
            kind: "identity".to_string(),
            data: data,
            position: Position::new(200.0, 0.0),
        }
    }

    pub fn new_prompt(id: &str, template: &str) -> Self {
        let data = NodeData {
            label: "Prompt Node".to_string(),
            description: None,
            payload: json!({
                "template": template,
            }),
        };
        Node {
            id: id.to_string(),
            kind: "prompt".to_string(),
            data: data,
            position: Position::new(100.0, 100.0),
        }
    }

    pub fn new_branch(id: &str) -> Self {
        let data = NodeData {
            label: "Branch Node".to_string(),
            description: None,
            payload: json!({
                "input": "What is your choice?",
            }),
        };
        Node {
            id: id.to_string(),
            kind: "branch".to_string(),
            data: data,
            position: Position::new(100.0, 100.0),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }
}
