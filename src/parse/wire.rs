//! Serde shapes of the two accepted line dialects.
//!
//! * the plain dialect: `{"type":"node","id":..,"labels":[..],"properties":{..}}`
//!   and `{"type":"edge","source":..,"target":..,"label":..,"properties":{..}}`;
//! * Neo4j APOC `apoc.export.json`: `{"type":"relationship","id":..,"label":..,
//!   "start":{"id":..,"labels":[..]},"end":{"id":..},"properties":{..}}`.
//!
//! Both are accepted on every line; a file may mix them.

use serde::Deserialize;
use serde_json::{Map, Value};

use crate::value::Id;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub(super) enum Line {
    Node(NodeLine),
    #[serde(alias = "relationship")]
    Edge(EdgeLine),
}

#[derive(Debug, Deserialize)]
pub(super) struct NodeLine {
    pub(super) id: Id,
    #[serde(default)]
    pub(super) labels: Vec<String>,
    #[serde(default)]
    pub(super) properties: Map<String, Value>,
}

#[derive(Debug, Deserialize)]
pub(super) struct EdgeLine {
    pub(super) id: Option<Id>,
    #[serde(alias = "start")]
    pub(super) source: Endpoint,
    #[serde(alias = "end")]
    pub(super) target: Endpoint,
    pub(super) label: String,
    #[serde(default)]
    pub(super) properties: Map<String, Value>,
}

/// An endpoint written as a bare id or as an APOC `{"id":..}` object.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum Endpoint {
    Bare(Id),
    Object { id: Id },
}

impl From<Endpoint> for String {
    fn from(endpoint: Endpoint) -> Self {
        match endpoint {
            Endpoint::Bare(id) | Endpoint::Object { id } => id.into(),
        }
    }
}
