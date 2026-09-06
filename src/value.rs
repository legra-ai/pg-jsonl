//! JSON values → text properties with type hints.

use serde_json::{
    Map,
    Value,
};

use crate::types::Property;

/// Type hint for a JSON string.
pub(crate) const HINT_STRING: &str = "string";
/// Type hint for a JSON integer.
pub(crate) const HINT_INT: &str = "int";
/// Type hint for a JSON non-integer number.
pub(crate) const HINT_FLOAT: &str = "float";
/// Type hint for a JSON boolean.
pub(crate) const HINT_BOOLEAN: &str = "boolean";
/// Type hint for a JSON array or object (kept as compact JSON text).
pub(crate) const HINT_JSON: &str = "json";

/// Convert a JSON `properties` object into properties in file order.
/// `null` values are skipped: a null property is an absent property.
pub(crate) fn properties_from(map: Map<String, Value>) -> Vec<Property> {
    map.into_iter()
        .filter_map(|(name, value)| property_from(name, value))
        .collect()
}

fn property_from(name: String, value: Value) -> Option<Property> {
    let (value, hint) = match value {
        Value::Null => return None,
        Value::String(text) => (text, HINT_STRING),
        Value::Number(number) => {
            let hint = if number.is_f64() {
                HINT_FLOAT
            } else {
                HINT_INT
            };
            (number.to_string(), hint)
        }
        Value::Bool(flag) => (flag.to_string(), HINT_BOOLEAN),
        composite @ (Value::Array(_) | Value::Object(_)) => (composite.to_string(), HINT_JSON),
    };
    Some(Property {
        name,
        value,
        type_hint: Some(hint.to_owned()),
    })
}

/// An identifier that a file may spell as a JSON string or number.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum Id {
    Text(String),
    Number(serde_json::Number),
}

impl From<Id> for String {
    fn from(id: Id) -> Self {
        match id {
            Id::Text(text) => text,
            Id::Number(number) => number.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn scalars_get_json_type_hints_and_nulls_vanish() {
        let Value::Object(map) = json!({
            "name": "Alice", "age": 30, "score": 1.5, "active": true,
            "tags": ["a", "b"], "gone": null
        }) else {
            unreachable!()
        };
        let props = properties_from(map);
        let hints: Vec<(&str, &str, &str)> = props
            .iter()
            .map(|p| {
                (
                    p.name.as_str(),
                    p.value.as_str(),
                    p.type_hint.as_deref().unwrap(),
                )
            })
            .collect();
        assert_eq!(
            hints,
            vec![
                ("active", "true", "boolean"),
                ("age", "30", "int"),
                ("name", "Alice", "string"),
                ("score", "1.5", "float"),
                ("tags", "[\"a\",\"b\"]", "json"),
            ]
        );
    }
}
