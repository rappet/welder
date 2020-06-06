use serde::{Serialize, Deserialize};

use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;

use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Role {
    #[serde(default)]
    pub version: u32,
    #[serde(default)]
    pub vars: BTreeMap<String, Var>,
    #[serde(default)]
    pub locals: BTreeMap<String, serde_json::Value>,
    #[serde(default)]
    pub tasks: BTreeMap<String, BTreeMap<String, Task>>,
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
#[serde(untagged)]
pub enum StringOrVec {
    String(String),
    Vec(Vec<String>)
}

impl Default for StringOrVec {
    fn default() -> StringOrVec {
        StringOrVec::Vec(Vec::new())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Var {
    #[serde(default)]
    required: bool,
    #[serde(default)]
    description: Option<String>,
    #[serde(rename = "type")]
    var_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Task {
    pub description: Option<String>,

    #[serde(default = "empty_toml_table", flatten)]
    pub params: toml::value::Table,

    #[serde(default)]
    pub require: StringOrVec,
    #[serde(default)]
    pub yields: StringOrVec,
}
