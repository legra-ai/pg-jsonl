//! Stateful line-by-line streaming parser.

use crate::error::PgJsonlError;
use crate::parse::record::PgJsonlRecord;
use crate::parse::wire::{
    EdgeLine,
    Line,
    NodeLine,
};
use crate::types::{
    Edge,
    Node,
};
use crate::value::properties_from;

/// Stateful line-by-line PG-JSONL parser. Memory is bounded by one
/// line: nothing is retained between calls except the line counter.
#[derive(Debug, Default)]
pub struct PgJsonlStreamParser {
    line: usize,
}

impl PgJsonlStreamParser {
    /// Create a new streaming parser.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse one physical line. Blank lines yield `Ok(None)`.
    ///
    /// # Errors
    ///
    /// Returns [`PgJsonlError`] when the line is not a valid node or
    /// edge record.
    pub fn parse_line(&mut self, line: &str) -> Result<Option<PgJsonlRecord>, PgJsonlError> {
        self.line += 1;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }
        let parsed: Line =
            serde_json::from_str(trimmed).map_err(|error| self.json_error(&error))?;
        let record = match parsed {
            Line::Node(node) => PgJsonlRecord::Node(self.node(node)?),
            Line::Edge(edge) => PgJsonlRecord::Edge(self.edge(edge)?),
        };
        Ok(Some(record))
    }

    fn json_error(&self, error: &serde_json::Error) -> PgJsonlError {
        let detail = error.to_string();
        if let Some(kind) = detail
            .strip_prefix("unknown variant `")
            .and_then(|rest| rest.split('`').next())
        {
            return PgJsonlError::UnknownType {
                line: self.line,
                kind: kind.to_owned(),
            };
        }
        PgJsonlError::Json {
            line: self.line,
            detail,
        }
    }

    fn non_empty(&self, value: String, field: &'static str) -> Result<String, PgJsonlError> {
        if value.is_empty() {
            return Err(PgJsonlError::EmptyField {
                line: self.line,
                field,
            });
        }
        Ok(value)
    }

    fn node(&self, node: NodeLine) -> Result<Node, PgJsonlError> {
        Ok(Node {
            id: self.non_empty(node.id.into(), "id")?,
            labels: node.labels,
            properties: properties_from(node.properties),
        })
    }

    fn edge(&self, edge: EdgeLine) -> Result<Edge, PgJsonlError> {
        Ok(Edge {
            id: edge.id.map(String::from),
            source: self.non_empty(edge.source.into(), "source")?,
            target: self.non_empty(edge.target.into(), "target")?,
            label: self.non_empty(edge.label, "label")?,
            properties: properties_from(edge.properties),
        })
    }
}
