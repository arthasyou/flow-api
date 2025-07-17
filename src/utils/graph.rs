use workflow_rs::model::node::{ControlNode, DataNode, NodeType};

pub(crate) fn get_workflow_node_type(t: &str) -> workflow_rs::model::node::NodeType {
    match t {
        "input" => NodeType::Data(DataNode::Input),
        "prompt" => NodeType::Data(DataNode::Prompt),
        "identity" => NodeType::Data(DataNode::Identity),
        "branch" => NodeType::Control(ControlNode::Branch),
        "llm" => NodeType::Data(DataNode::LLM),
        _ => NodeType::Data(DataNode::Identity),
    }
}
