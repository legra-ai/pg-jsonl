//! Whole-input convenience entry points.

use crate::error::PgJsonlError;
use crate::parse::record::PgJsonlRecord;
use crate::parse::stream::PgJsonlStreamParser;
use crate::types::{
    Edge,
    Node,
};

/// Parse a complete PG-JSONL document, invoking `on_node` / `on_edge`
/// for each record as it is parsed. Memory stays bounded by one line.
///
/// # Errors
///
/// Returns the first [`PgJsonlError`] encountered.
pub fn parse_pg_jsonl_with(
    input: &str,
    mut on_node: impl FnMut(Node),
    mut on_edge: impl FnMut(Edge),
) -> Result<(), PgJsonlError> {
    let mut parser = PgJsonlStreamParser::new();
    for line in input.lines() {
        match parser.parse_line(line)? {
            Some(PgJsonlRecord::Node(node)) => on_node(node),
            Some(PgJsonlRecord::Edge(edge)) => on_edge(edge),
            None => {}
        }
    }
    Ok(())
}

/// Parse a complete PG-JSONL document into its nodes and edges.
///
/// # Errors
///
/// Returns the first [`PgJsonlError`] encountered.
pub fn parse_pg_jsonl(input: &str) -> Result<(Vec<Node>, Vec<Edge>), PgJsonlError> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    parse_pg_jsonl_with(input, |node| nodes.push(node), |edge| edges.push(edge))?;
    Ok((nodes, edges))
}
