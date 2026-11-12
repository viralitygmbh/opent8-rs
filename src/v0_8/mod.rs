// SPDX-License-Identifier: MIT
//! Schema-generated OpenT8 0.8 types. Regenerate with scripts/generate-opent8.sh.
#[allow(clippy::all)]
mod generated;
pub use generated::OpenT8DocumentSchemaSpecification as Document;
pub use generated::OpenT8DocumentSchemaSpecificationSchedule as Schedule;
pub use generated::OpenT8DocumentSchemaSpecificationScheduleScheduleElementsItem as ScheduleElement;
pub use generated::*;

pub const SCHEMA: &str = include_str!("../../schema/v0.8/schema.json");

/// Serialize a document with fields in this version's schema order.
pub fn to_json_value(document: &Document) -> Result<serde_json::Value, serde_json::Error> {
    static SCHEMA_VALUE: std::sync::LazyLock<serde_json::Value> = std::sync::LazyLock::new(|| {
        serde_json::from_str(SCHEMA.trim_start_matches('\u{feff}'))
            .expect("vendored OpenT8 schema must be valid JSON")
    });
    crate::json::to_json_value(document, &SCHEMA_VALUE)
}
