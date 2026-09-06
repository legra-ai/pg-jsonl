# pg-jsonl

[![Crates.io](https://img.shields.io/crates/v/pg-jsonl.svg)](https://crates.io/crates/pg-jsonl)
[![Downloads](https://img.shields.io/crates/d/pg-jsonl.svg)](https://crates.io/crates/pg-jsonl)
[![Documentation](https://docs.rs/pg-jsonl/badge.svg)](https://docs.rs/pg-jsonl)
[![CI](https://github.com/legra-ai/pg-jsonl/actions/workflows/ci.yml/badge.svg)](https://github.com/legra-ai/pg-jsonl/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/legra-ai/pg-jsonl)

An owned, streaming parser for property-graph JSON Lines (PG-JSONL): one
JSON object per line describing a node or an edge.

Two dialects are accepted on every line, so files may mix them:

- the plain dialect
  ```json
  {"type":"node","id":"alice","labels":["Person"],"properties":{"name":"Alice","age":30}}
  {"type":"edge","source":"alice","target":"bob","label":"KNOWS","properties":{"since":2020}}
  ```
- Neo4j APOC `apoc.export.json`
  ```json
  {"type":"relationship","id":"7","label":"KNOWS","start":{"id":"0","labels":["Person"]},"end":{"id":"1"},"properties":{}}
  ```

Identifiers may be JSON strings or numbers; they are kept as text. Property
values are rendered as text with the JSON type as a hint (`string`, `int`,
`float`, `boolean`, or `json` for arrays and objects); `null` properties are
skipped.

## Why this crate

The `PgJsonlStreamParser` API processes one physical line at a time and keeps
nothing between lines but a counter, so memory stays bounded for large exports
and callers decide where records go. The convenience `parse_pg_jsonl` function
is available when collecting the complete result is appropriate.

The crate depends on no database driver, graph store, RDF model, or application
framework. The optional `property-graph-model` feature adds field-for-field
`From` conversions into that crate's `PgNode` / `PgEdge`.

## Quick start

```rust
use pg_jsonl::parse_pg_jsonl;

let input = r#"{"type":"node","id":"alice","labels":["Person"],"properties":{"name":"Alice"}}
{"type":"edge","source":"alice","target":"bob","label":"KNOWS"}
"#;
let (nodes, edges) = parse_pg_jsonl(input)?;

assert_eq!(nodes[0].id, "alice");
assert_eq!(nodes[0].labels, ["Person"]);
assert_eq!(nodes[0].properties[0].name, "name");
assert_eq!(edges[0].label, "KNOWS");
# Ok::<(), pg_jsonl::PgJsonlError>(())
```

## Streaming large exports

```rust
use pg_jsonl::{PgJsonlRecord, PgJsonlStreamParser};

let mut parser = PgJsonlStreamParser::new();
let mut edges = 0;
for line in r#"{"type":"node","id":"a"}
{"type":"edge","source":"a","target":"a","label":"SELF"}
"#.lines() {
    if let Some(PgJsonlRecord::Edge(_)) = parser.parse_line(line)? {
        edges += 1;
    }
}
assert_eq!(edges, 1);
# Ok::<(), pg_jsonl::PgJsonlError>(())
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
