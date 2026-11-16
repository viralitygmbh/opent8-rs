# opent8-rs

A Rust library for reading and writing [OpenT8](https://openpotato.github.io/opent8/)
timetable data. It provides Serde types for OpenT8 **0.5–0.8**, the
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

- `Document` and its field types, with Serde support.
- `to_json_value(&document)` to serialize fields in schema order.
- `SCHEMA`, the original JSON schema for validation.

## Supported versions

| OpenT8             | Rust module    |
|--------------------|----------------|
| 0.5 (schema 0.5.1) | `opent8::v0_5` |
| 0.6                | `opent8::v0_6` |
| 0.7                | `opent8::v0_7` |
| 0.8                | `opent8::v0_8` |

All versions are included. The crate version tracks the Rust API independently
of the OpenT8 specification. Requires Rust 1.88 or newer.

## Validation

Deserialization does not enforce every JSON Schema constraint, including
uniqueness. Validate against the matching `SCHEMA` when full schema validation
is needed. Reference integrity and other semantic rules may require additional
application checks.

Unknown fields, including custom `x-` properties, may be rejected or ignored
depending on the object. They are not preserved when writing the document back.
Chrono-backed timestamps retain at most nine fractional digits (nanoseconds);
additional digits are truncated, not rounded.

## Development

Schemas and Rust code are checked in. See [CONTRIBUTING.md](CONTRIBUTING.md)
for regeneration and release checks.

## License

MIT, except for the upstream schemas and code derived from them, which are
Apache-2.0. See [licenses and attribution](THIRD_PARTY_NOTICES.md) for details.
