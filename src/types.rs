//! Owned records produced by the parser.

/// A property-graph node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    /// The node's identifier, as written in the file.
    pub id: String,
    /// Zero or more labels.
    pub labels: Vec<String>,
    /// Properties in file order.
    pub properties: Vec<Property>,
}

/// A property-graph edge (relationship).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    /// The edge's own identifier, when the file carries one.
    pub id: Option<String>,
    /// The source (start) node identifier.
    pub source: String,
    /// The target (end) node identifier.
    pub target: String,
    /// The relationship type / label.
    pub label: String,
    /// Properties in file order.
    pub properties: Vec<Property>,
}

/// One property with its value rendered as text and the JSON type it
/// came from as a hint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property {
    /// Property name.
    pub name: String,
    /// The value as text: strings verbatim, numbers and booleans in
    /// their JSON spelling, arrays and objects as compact JSON.
    pub value: String,
    /// The JSON type: `string`, `int`, `float`, `boolean`, or `json`
    /// for arrays and objects.
    pub type_hint: Option<String>,
}
