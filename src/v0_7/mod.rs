// SPDX-License-Identifier: MIT
//! OpenT8 0.7 timetable types and JSON schema.
//!
//! Based on the [OpenT8 0.7.0 schema][upstream]. The schema and its Rust types
//! are licensed under Apache-2.0; see [licences and attribution](crate::licenses).
//!
//! [upstream]: https://github.com/openpotato/opent8/blob/ba944b392e7bb36a72bb8d452e356c88444cc1e7/schemas/v0.7/schema.json

#[allow(clippy::all)]
mod generated;
pub use generated::OpenT8DocumentSchemaSpecification as Document;
pub use generated::OpenT8DocumentSchemaSpecificationSchedule as Schedule;
pub use generated::OpenT8DocumentSchemaSpecificationScheduleScheduleElementsItem as ScheduleElement;
pub use generated::*;

/// Original OpenT8 0.7.0 JSON Schema.
///
/// Unmodified [upstream schema][upstream], licensed under Apache-2.0.
/// See [licences and attribution](crate::licenses) for the full licence text.
///
/// [upstream]: https://github.com/openpotato/opent8/blob/ba944b392e7bb36a72bb8d452e356c88444cc1e7/schemas/v0.7/schema.json
pub const SCHEMA: &str = include_str!("../../schema/v0.7/schema.json");

/// Serialize a document with fields in this version's schema order.
pub fn to_json_value(document: &Document) -> Result<serde_json::Value, serde_json::Error> {
    static SCHEMA_VALUE: std::sync::LazyLock<serde_json::Value> = std::sync::LazyLock::new(|| {
        serde_json::from_str(SCHEMA.trim_start_matches('\u{feff}'))
            .expect("vendored OpenT8 schema must be valid JSON")
    });
    crate::json::to_json_value(document, &SCHEMA_VALUE)
}
