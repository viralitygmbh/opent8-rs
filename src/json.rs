// SPDX-License-Identifier: MIT
use serde::Serialize;
use serde_json::{Map, Value};

/// Produce JSON in specification order rather than the generator's alphabetical
/// struct order. Use this value with serde_json::to_string_pretty for file exports.
pub(crate) fn to_json_value(
    document: &impl Serialize,
    schema: &Value,
) -> Result<Value, serde_json::Error> {
    let value = serde_json::to_value(document)?;
    Ok(ordered(value, schema, schema))
}

fn resolve<'a>(mut schema: &'a Value, root: &'a Value) -> &'a Value {
    while let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
        schema = root
            .pointer(reference.strip_prefix('#').expect("local schema reference"))
            .expect("schema reference must resolve");
    }
    schema
}

fn ordered(value: Value, schema: &Value, root: &Value) -> Value {
    let mut schema = resolve(schema, root);
    if let Some(variants) = schema.get("oneOf").and_then(Value::as_array)
        && let Some(variant) =
            variants
                .iter()
                .map(|variant| resolve(variant, root))
                .find(|variant| {
                    let discriminator = variant.pointer("/properties/type/const");
                    discriminator.is_some() && discriminator == value.get("type")
                })
    {
        schema = variant;
    }
    match value {
        Value::Object(mut fields) => {
            let mut result = Map::new();
            if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
                for (key, property) in properties {
                    if let Some(value) = fields.shift_remove(key) {
                        result.insert(key.clone(), ordered(value, property, root));
                    }
                }
            }
            // Keep extensions or fields not described by this schema.
            result.extend(fields);
            Value::Object(result)
        }
        Value::Array(values) => Value::Array(
            values
                .into_iter()
                .map(|value| ordered(value, schema.get("items").unwrap_or(&Value::Null), root))
                .collect(),
        ),
        other => other,
    }
}
