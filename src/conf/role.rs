use crate::module::Module;

use serde::{Serialize, Deserialize};

use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;

use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Role {
    #[serde(default)]
    pub vars: BTreeMap<String, serde_json::Value>,
    #[serde(rename = "task")]
    pub tasks: Vec<Task>,
}

impl Role {
    pub fn open(path: &Path) -> Result<Role, Box<dyn Error>> {
        info!("Open role file {}", path.display());
        let file = fs::read_to_string(path)?;
        info!("Parse role file {}", path.display());
        let role = toml::from_str(&file)?;
        Ok(role)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Task {
    pub name: String,
    pub module: Module,

    #[serde(default = "empty_toml_table", flatten)]
    pub params: toml::value::Table,

    pub requires: Option<String>,
    pub yields: Option<String>,
}
