use serde::Serialize;
use serde_json::Value;
use service_utils_rs::services::db::get_db;
use surrealdb::Uuid;

use crate::{
    error::{Error, Result},
    models::{Edge, Graph, Node, graph::GraphSummary},
};

pub async fn create_graph_table() -> Result<()> {
    let query = r#"
        DEFINE TABLE IF NOT EXISTS graph SCHEMALESS PERMISSIONS FULL;

        DEFINE FIELD IF NOT EXISTS uuid ON TABLE graph TYPE string READONLY;
        DEFINE FIELD IF NOT EXISTS name ON TABLE graph TYPE string;
        DEFINE FIELD IF NOT EXISTS description ON TABLE graph TYPE string;
        DEFINE FIELD IF NOT EXISTS owner ON TABLE graph TYPE string READONLY;
        DEFINE FIELD IF NOT EXISTS nodes ON TABLE graph TYPE array<object>;
        DEFINE FIELD IF NOT EXISTS edges ON TABLE graph TYPE array<object>;
        DEFINE FIELD IF NOT EXISTS start_node ON TABLE graph TYPE string;
        DEFINE FIELD IF NOT EXISTS end_node ON TABLE graph TYPE string;
        DEFINE FIELD IF NOT EXISTS created_at ON TABLE graph TYPE datetime DEFAULT time::now() READONLY;
        DEFINE FIELD IF NOT EXISTS updated_at ON TABLE graph TYPE datetime VALUE time::now();

        DEFINE INDEX IF NOT EXISTS unique_graph_uuid ON TABLE graph FIELDS uuid UNIQUE;
        DEFINE INDEX IF NOT EXISTS idx_graph_owner ON TABLE graph FIELDS owner;
    "#;

    let db = get_db();
    db.query(query).await?;

    Ok(())
}

#[derive(Serialize, Debug)]
pub(crate) struct GraphInput {
    pub uuid: String,
    pub owner: String,
    pub name: String,
    pub description: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub start_node: String,
    pub end_node: String,
}

impl GraphInput {
    pub fn default_with_owner(owner: &str, name: &str, description: &str) -> Self {
        let start_node = Node::new_start_node();
        let end_node = Node::new_end_node();
        println!("end_node: {:?}", end_node);
        let nodes = vec![start_node, end_node];
        GraphInput {
            uuid: Uuid::new_v4().to_string(),
            owner: owner.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            nodes,
            edges: Vec::new(),
            start_node: "start".to_string(),
            end_node: "end".to_string(),
        }
    }

    pub fn demo(owner: &str, name: &str, description: &str) -> Self {
        let start_node = Node::new_start_node();
        let end_node = Node::new_end_node();

        let nodes = vec![start_node, end_node];

        GraphInput {
            uuid: Uuid::new_v4().to_string(),
            owner: "demo".to_string(),
            name: "Demo Graph".to_string(),
            description: "This is a demo graph".to_string(),
            nodes,
            edges: Vec::new(),
            start_node: "start".to_string(),
            end_node: "end".to_string(),
        }
    }
}

// pub async fn create_graph(input: GraphInput) -> Result<()> {
//     let db = get_db();
//     let _r: Option<Graph> = db.create(("graph", &input.uuid)).content(input).await?;
//     Ok(())
// }

pub async fn default_with_owner(owner: &str, name: &str, description: &str) -> Result<String> {
    let db = get_db();
    let input = GraphInput::default_with_owner(owner, name, description);
    let r: Option<Graph> = db.create(("graph", &input.uuid)).content(input).await?;
    if let Some(graph) = r {
        Ok(graph.uuid)
    } else {
        Err(Error::GraphCreationError)
    }
}

pub async fn update_graph_by_id(id: &str, nodes: Vec<Node>, edges: Vec<Edge>) -> Result<()> {
    let query = r#"
        UPDATE graph SET nodes = $nodes, edges = $edges WHERE uuid = $id;
    "#;

    let nodes_value: Value = serde_json::to_value(nodes)?.into();
    let edges_value: Value = serde_json::to_value(edges)?.into();

    let db = get_db();
    db.query(query)
        .bind(("id", id.to_owned()))
        .bind(("nodes", nodes_value))
        .bind(("edges", edges_value))
        .await?;

    Ok(())
}

pub async fn get_graph_by_id(id: &str) -> Result<Option<Graph>> {
    let db = get_db();
    let r: Option<Graph> = db.select(("graph", id.to_owned())).await?;
    Ok(r)
}

pub async fn get_graphs_by_owner(owner: &str) -> Result<Vec<GraphSummary>> {
    let query = r#"
        SELECT uuid, name, description FROM graph WHERE owner = $owner;
    "#;

    let db = get_db();
    let mut response = db.query(query).bind(("owner", owner.to_owned())).await?;
    let r = response.take(0)?;
    Ok(r)
}

pub async fn get_owner_graph_by_id(id: &str, owner: &str) -> Result<Option<Graph>> {
    let query = r#"
        SELECT * FROM graph WHERE uuid = $id AND owner = $owner;
    "#;

    let db = get_db();
    let mut response = db
        .query(query)
        .bind(("id", id.to_owned()))
        .bind(("owner", owner.to_owned()))
        .await?;
    let r = response.take(0)?;
    Ok(r)
}

pub async fn delete_graph_by_id(id: &str) -> Result<()> {
    let db = get_db();
    let _: Option<Graph> = db.delete(("graph", id.to_owned())).await?;
    Ok(())
}
