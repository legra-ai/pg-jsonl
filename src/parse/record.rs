//! The parsed record of one line.

use crate::types::{Edge, Node};

/// One parsed PG-JSONL line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PgJsonlRecord {
    /// A `"type":"node"` line.
    Node(Node),
    /// A `"type":"edge"` (or Neo4j APOC `"type":"relationship"`) line.
    Edge(Edge),
}
