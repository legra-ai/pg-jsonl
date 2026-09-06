//! PG-JSONL parser error types.

/// Errors from parsing PG-JSONL files.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PgJsonlError {
    /// A line is not a valid JSON object of a known record shape.
    #[error("PG-JSONL: line {line}: {detail}")]
    Json {
        /// 1-based line number.
        line: usize,
        /// The JSON parser's failure detail.
        detail: String,
    },

    /// A record's `type` is neither `node` nor `edge`/`relationship`.
    #[error("PG-JSONL: line {line}: unknown record type '{kind}'")]
    UnknownType {
        /// 1-based line number.
        line: usize,
        /// The offending `type` value.
        kind: String,
    },

    /// An identifier field is empty.
    #[error("PG-JSONL: line {line}: '{field}' must not be empty")]
    EmptyField {
        /// 1-based line number.
        line: usize,
        /// The field name.
        field: &'static str,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_unknown_type() {
        let err = PgJsonlError::UnknownType {
            line: 3,
            kind: "vertex".to_owned(),
        };
        assert_eq!(
            err.to_string(),
            "PG-JSONL: line 3: unknown record type 'vertex'"
        );
    }
}
