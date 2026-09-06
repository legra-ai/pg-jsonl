use crate::error::PgJsonlError;
use crate::parse::{PgJsonlRecord, PgJsonlStreamParser, parse_pg_jsonl};

#[test]
fn plain_dialect_nodes_and_edges() {
    let input = r#"{"type":"node","id":"alice","labels":["Person"],"properties":{"name":"Alice","age":30}}
{"type":"node","id":"bob","labels":["Person","Employee"],"properties":{"name":"Bob"}}

{"type":"edge","source":"alice","target":"bob","label":"KNOWS","properties":{"since":2020}}
"#;
    let (nodes, edges) = parse_pg_jsonl(input).expect("parse");
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].id, "alice");
    assert_eq!(nodes[0].labels, ["Person"]);
    assert_eq!(nodes[0].properties[0].name, "age");
    assert_eq!(nodes[0].properties[0].value, "30");
    assert_eq!(nodes[0].properties[0].type_hint.as_deref(), Some("int"));
    assert_eq!(nodes[1].labels, ["Person", "Employee"]);
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].source, "alice");
    assert_eq!(edges[0].target, "bob");
    assert_eq!(edges[0].label, "KNOWS");
    assert_eq!(edges[0].id, None);
    assert_eq!(edges[0].properties[0].value, "2020");
}

#[test]
fn neo4j_apoc_dialect_is_accepted() {
    let input = r#"{"type":"node","id":"0","labels":["Person"],"properties":{"name":"Alice"}}
{"type":"relationship","id":"7","label":"KNOWS","start":{"id":"0","labels":["Person"]},"end":{"id":"1","labels":["Person"]},"properties":{"since":2020}}
"#;
    let (nodes, edges) = parse_pg_jsonl(input).expect("parse");
    assert_eq!(nodes.len(), 1);
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].id.as_deref(), Some("7"));
    assert_eq!(edges[0].source, "0");
    assert_eq!(edges[0].target, "1");
    assert_eq!(edges[0].label, "KNOWS");
}

#[test]
fn numeric_ids_are_kept_as_text() {
    let mut parser = PgJsonlStreamParser::new();
    let record = parser
        .parse_line(r#"{"type":"node","id":42}"#)
        .expect("parse")
        .expect("record");
    let PgJsonlRecord::Node(node) = record else {
        panic!("node expected");
    };
    assert_eq!(node.id, "42");
    assert_eq!(node.labels.len(), 0);
    assert_eq!(node.properties.len(), 0);
}

#[test]
fn errors_carry_the_line_number() {
    let mut parser = PgJsonlStreamParser::new();
    parser.parse_line("").expect("blank line");
    let error = parser
        .parse_line(r#"{"type":"vertex","id":"x"}"#)
        .unwrap_err();
    assert_eq!(
        error,
        PgJsonlError::UnknownType {
            line: 2,
            kind: "vertex".to_owned()
        }
    );
    let error = parser.parse_line("not json").unwrap_err();
    assert!(
        matches!(error, PgJsonlError::Json { line: 3, .. }),
        "{error}"
    );
    let error = parser
        .parse_line(r#"{"type":"edge","source":"","target":"b","label":"L"}"#)
        .unwrap_err();
    assert_eq!(
        error,
        PgJsonlError::EmptyField {
            line: 4,
            field: "source"
        }
    );
}
