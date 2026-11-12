// SPDX-License-Identifier: MIT
//! Versioned OpenT8 timetable types and schema-ordered JSON serialization.
//!
//! Use [`v0_7`] or [`v0_8`] explicitly. Generated types are not a complete
//! schema validator; validate external documents against the original schema.
//! Unknown fields (including `x-` extensions) are rejected by the current types.
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
pub mod v0_7;
pub mod v0_8;
