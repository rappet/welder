use crate::module::Module;

use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Role {
    #[serde(default)]
    pub vars: BTreeMap<String, serde_json::Value>,
    #[serde(rename = "task")]
    pub tasks: Vec<Task>,
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
