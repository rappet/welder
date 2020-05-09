use serde::{Serialize, Deserialize};

mod debug;
mod var;

pub use debug::{Debug};
pub use var::{Var};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all="snake_case")]
pub enum Module {
    Debug,
    Var,
}
