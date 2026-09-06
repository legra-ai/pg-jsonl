//! Public-API integration test: both accepted line dialects, streaming
//! parsing, and line-numbered errors, exercised from the shipped crate.

use pg_jsonl::{
    PgJsonlError,
    PgJsonlRecord,
    PgJsonlStreamParser,
    parse_pg_jsonl,
};

const PLAIN: &str = r#"{"type":"node","id":"alice","labels":["Person"],"properties":{"name":"Alice","age":30}}
{"type":"edge","source":"alice","target":"bob","label":"KNOWS","properties":{"since":2020}}
"#;

const APOC: &str = r#"{"type":"node","id":"0","labels":["Person"],"properties":{"name":"Alice"}}
{"type":"relationship","id":"7","label":"KNOWS","start":{"id":"0","labels":["Person"]},"end":{"id":1},"properties":{}}
"#;

#[test]
fn plain_dialect_parses_with_json_type_hints() {
    let (nodes, edges) = parse_pg_jsonl(PLAIN).expect("valid PG-JSONL");
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].id, "alice");
    assert_eq!(nodes[0].labels, ["Person"]);
    let age = nodes[0]
        .properties
        .iter()
        .find(|p| p.name == "age")
        .expect("age");
    assert_eq!(age.value, "30");
    assert_eq!(age.type_hint.as_deref(), Some("int"));
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].label, "KNOWS");
    assert_eq!(edges[0].properties[0].value, "2020");
}

#[test]
fn neo4j_apoc_dialect_parses_on_the_same_parser() {
    let (nodes, edges) = parse_pg_jsonl(APOC).expect("valid APOC JSON");
    assert_eq!(nodes[0].id, "0");
    assert_eq!(edges[0].id.as_deref(), Some("7"));
    assert_eq!(edges[0].source, "0");
    assert_eq!(edges[0].target, "1", "numeric ids are kept as text");
}

#[test]
fn streaming_parser_reports_the_offending_line_number() {
    let mut parser = PgJsonlStreamParser::new();
    let first = parser
        .parse_line(PLAIN.lines().next().unwrap())
        .expect("line 1");
    assert!(matches!(first, Some(PgJsonlRecord::Node(_))));
    assert!(parser.parse_line("").expect("blank line").is_none());
    let error = parser
        .parse_line(r#"{"type":"vertex","id":"x"}"#)
        .expect_err("unknown record type");
    assert_eq!(
        error,
        PgJsonlError::UnknownType {
            line: 3,
            kind: "vertex".to_owned()
        }
    );
}
