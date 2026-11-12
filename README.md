# opent8-rs

A Rust library for reading and writing [OpenT8](https://openpotato.github.io/opent8/)
timetable data. It provides Serde types for OpenT8 **0.7** and **0.8**, the
original JSON schemas, and helpers for serializing fields in specification order.

## Usage

Choose the module matching your document's OpenT8 version:

```rust
use opent8::v0_8;

fn read_and_write(input: &str) -> Result<String, serde_json::Error> {
    let document: v0_8::Document = serde_json::from_str(input)?;
    let value = v0_8::to_json_value(&document)?;
    serde_json::to_string_pretty(&value)
}
```

Each version module exposes:

- `Document` and the types used by its fields, with Serde serialization and
  deserialization support.
- `to_json_value(&document)` to serialize fields in the order defined by the
  schema. Use this before writing JSON when specification ordering matters.
- `SCHEMA`, the original JSON schema as a string, for use with a JSON Schema
  validator.

## Supported versions

| OpenT8 | Rust module |
| --- | --- |
| 0.7 | `opent8::v0_7` |
| 0.8 | `opent8::v0_8` |

Both versions are included. The crate version tracks the Rust API independently
of the OpenT8 specification. Conversion between versions is not implemented yet.

## Validation

Deserialization does not enforce every JSON Schema constraint, including
uniqueness. Validate against the matching `SCHEMA` when full schema validation
is needed. Reference integrity and other semantic rules may require additional
application checks.

Unknown fields are rejected during deserialization. This includes custom `x-`
properties: although OpenT8 allows them on certain objects, such as `info`,
the current Rust types do not support them.

## Development

The schemas and code are checked in; normal builds do not need a generator or
schema downloads. See [CONTRIBUTING.md](CONTRIBUTING.md) for schema updates,
code generation, and release checks.

## License

MIT, except for the upstream schemas and code derived from them, which are
Apache-2.0. See [licenses and attribution](THIRD_PARTY_NOTICES.md) for details.
