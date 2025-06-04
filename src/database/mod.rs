pub mod graph;

use crate::{database::graph::create_graph_table, error::Result};

pub async fn create_tables() -> Result<()> {
    create_graph_table().await?;
    Ok(())
}
