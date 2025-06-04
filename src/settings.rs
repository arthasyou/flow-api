use serde::Deserialize;
use service_utils_rs::{
    services::{db::SurrealdbCfg, jwt::JwtCfg},
    utils::load_settings,
};

use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub http: HttpCfg,
    pub surrealdb: SurrealdbCfg,
    pub jwt: JwtCfg,
}

#[derive(Debug, Deserialize)]
pub struct HttpCfg {
    pub port: u16,
}

impl Settings {
    pub fn load(config_path: &str) -> Result<Self> {
        let r = load_settings(config_path)?;
        Ok(r)
    }
}
