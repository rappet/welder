use std::str::FromStr;
use std::path::Display;
use std::fmt;
use serde::export::Formatter;
use serde::export::fmt::Error;
use crate::conf::Task;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Visitor};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TaskIdentifier {
    role: Option<String>,
    module: String,
    name: String,
}

impl TaskIdentifier {
    /// Creates a new `TaskIdentifier` from defined contents.
    ///
    /// The Identifier is only valid for the local scope.
    /// A scope might be added using `scoped` or a scoped Identifier might be
    /// created with `with_scope`.
    pub fn new(module: &str, name: &str) -> TaskIdentifier {
        TaskIdentifier {
            role: None,
            module: module.to_string(),
            name: name.to_string()
        }
    }

    /// Creates a new scoped `TaskIdentifier` from defined contents.
    pub fn with_scope(role: &str, module: &str, name: &str) -> TaskIdentifier {
        TaskIdentifier {
            role: Some(role.to_string()),
            module: module.to_string(),
            name: name.to_string()
        }
    }

    /// Adds or replaces a scope to a `TaskIdentifier`.
    pub fn scoped(self, role: &str) -> TaskIdentifier {
        TaskIdentifier {
            role: Some(role.to_string()),
            module: self.module,
            name: self.name
        }
    }
}

impl FromStr for TaskIdentifier {
    type Err = ParseTaskIdentifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.split('.').collect();
        match split.len() {
            2 => Ok(TaskIdentifier::new( split[0], split[1])),
            3 => Ok(TaskIdentifier::with_scope(split[0], split[1], split[2])),
            _ => Err(ParseTaskIdentifierError { kind: TaskIdentifierErrorKind::WrongNumberDelimiters })
        }
    }
}

impl fmt::Display for TaskIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if let Some(role) = &self.role {
            write!(f, "{}.{}.{}", role, self.module, self.name)
        } else {
            write!(f, "{}.{}", self.module, self.name)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTaskIdentifierError {
    kind: TaskIdentifierErrorKind,
}

/// Enum to store the types of errors that cause TaskIdentifier parsing to fail.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TaskIdentifierErrorKind {
    /// Wrong number of delimiters/fields.
    ///
    /// A valid TaskIdentifier should have two or three field seperated by a "."
    WrongNumberDelimiters,
}

impl From<TaskIdentifierErrorKind> for ParseTaskIdentifierError {
    fn from(kind: TaskIdentifierErrorKind) -> Self {
        ParseTaskIdentifierError { kind }
    }
}

impl ParseTaskIdentifierError {
    /// Outputs the detailed cause of error.
    pub fn kind(&self) -> &TaskIdentifierErrorKind {
        &self.kind
    }

    pub fn description(&self) -> &str {
        match self.kind {
            TaskIdentifierErrorKind::WrongNumberDelimiters =>
                "The task identifier contains wrong number of delimiters. Correct format would be <module>.<name> or <scope>.<module>.<name>",
        }
    }
}

impl fmt::Display for ParseTaskIdentifierError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.description().fmt(f)
    }
}

// Implement serializer for serde
impl Serialize for TaskIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

// Implement Visitor for serde to allow deserializing
struct TaskIdentifierVisitor;

impl<'de> Visitor<'de> for TaskIdentifierVisitor {
    type Value = TaskIdentifier;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a string in the format <module>.<name> or <scope>.<module>.<name>")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error, {
        Ok(v.parse().map_err(|e: ParseTaskIdentifierError| {
            E::custom(e.description())
        })?)
    }
}

impl<'de> Deserialize<'de> for TaskIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<TaskIdentifier, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TaskIdentifierVisitor)
    }
}
