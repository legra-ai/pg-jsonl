//! Line-oriented parsing.

mod entry;
mod record;
mod stream;
mod wire;

#[cfg(test)]
mod tests;

pub use entry::{parse_pg_jsonl, parse_pg_jsonl_with};
pub use record::PgJsonlRecord;
pub use stream::PgJsonlStreamParser;
