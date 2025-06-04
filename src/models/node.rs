use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Node {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub data: Option<Value>,
    pub position: Position,
}

impl Node {
    pub fn new_start_node() -> Self {
        Node {
            id: "start".to_string(),
            kind: "start".to_string(),
            data: Some(json!({
                "label": "Start Node",
            })),
            position: Position::new(0.0, 0.0),
        }
    }

    pub fn new_end_node() -> Self {
        Node {
            id: "end".to_string(),
            kind: "end".to_string(),
            data: Some(json!({
                "label": "End Node",
            })),
            position: Position::new(200.0, 0.0),
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
