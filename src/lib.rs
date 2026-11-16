// SPDX-License-Identifier: MIT
//! Rust types and serialization for [OpenT8](https://openpotato.github.io/opent8/) timetable data.
//!
//! See the [OpenT8 specification](https://openpotato.github.io/opent8/specification/)
//! for document structure and scheduling semantics.
//!
//! The schemas and schema-derived types use Apache-2.0; other project code uses MIT.
//! See [licences and attribution](licenses) for the full licence texts.
//!
//! Choose [`v0_5`], [`v0_6`], [`v0_7`], or [`v0_8`] to match the document version.
//! Serde does not enforce every schema constraint; validate against the original schema.
//! Unknown fields, including `x-` extensions, may be rejected or ignored depending
//! on the object; they are not preserved. Chrono-backed timestamps have nanosecond
//! precision: additional fractional digits are truncated during deserialization.
//!
//! ```
//! let input = r#"{
//!   "opent8": "0.8.0",
//!   "info": {"title": "Example timetable"},
//!   "schedule": {
//!     "timeZone": "Europe/Berlin",
//!     "validFrom": "2026-09-01",
//!     "validTo": "2027-08-01"
//!   }
//! }"#;
//! let document: opent8::v0_8::Document = serde_json::from_str(input)?;
//! let value = opent8::v0_8::to_json_value(&document)?;
//! let output = serde_json::to_string_pretty(&value)?;
//! # Ok::<(), serde_json::Error>(())
//! ```
mod json;
pub mod v0_5;
pub mod v0_6;
pub mod v0_7;
pub mod v0_8;

pub mod licenses;
