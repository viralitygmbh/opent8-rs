// SPDX-License-Identifier: MIT
use serde_json::{Value, json};

fn input(version: &str) -> Value {
    let mut value = json!({
        "opent8": version,
        "info": {"title": "Test timetable", "publishedAt": "2026-09-01T08:30:00+02:00"},
        "schedule": {"timeZone": "Europe/Berlin", "validFrom": "2026-09-01", "validTo": "2027-08-01"}
    });
    if version != "0.8.0" {
        value["schedule"]
            .as_object_mut()
            .unwrap()
            .remove("timeZone");
    }
    if version.starts_with("0.5.") {
        value["schedule"]["validFrom"] = json!("2026-09-01T00:00:00+02:00");
        value["schedule"]["validTo"] = json!("2027-08-01T00:00:00+02:00");
    }
    value
}

macro_rules! version_tests {
    ($module:ident, $version:literal) => {
        mod $module {
            use super::*;
            use opent8::$module as api;

            #[test]
            fn round_trip_is_schema_valid_and_ordered() {
                let original = input($version);
                let schema: Value = serde_json::from_str(api::SCHEMA).unwrap();
                let validator = jsonschema::options().should_validate_formats(true).build(&schema).unwrap();
                assert!(validator.is_valid(&original));
                let document: api::Document = serde_json::from_value(original.clone()).unwrap();
                let output = api::to_json_value(&document).unwrap();
                assert_eq!(output, original);
                assert!(validator.is_valid(&output));
                assert_eq!(output.as_object().unwrap().keys().map(String::as_str).collect::<Vec<_>>(), ["opent8", "info", "schedule"]);
                assert_eq!(output["info"]["publishedAt"], "2026-09-01T08:30:00+02:00");
            }

            #[test]
            fn rejects_missing_required_metadata() {
                let mut value = input($version);
                value["info"].as_object_mut().unwrap().remove("title");
                assert!(serde_json::from_value::<api::Document>(value).is_err());
            }

            #[test]
            fn rejects_unknown_schedule_discriminator() {
                let mut value = input($version);
                value["schedule"]["scheduleElements"] = json!([{"type": "not-an-opent8-type", "id": "bad"}]);
                assert!(serde_json::from_value::<api::Document>(value).is_err());
            }
        }
    };
}
version_tests!(v0_5, "0.5.1");
version_tests!(v0_6, "0.6.0");
version_tests!(v0_7, "0.7.0");
version_tests!(v0_8, "0.8.0");

#[test]
fn v0_8_preserves_datetime_validity_offsets() {
    let mut value = input("0.8.0");
    value["schedule"]["validFrom"] = json!("2026-09-01T08:00:00+02:00");
    value["schedule"]["validTo"] = json!("2027-08-01T08:00:00+02:00");
    let document: opent8::v0_8::Document = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(opent8::v0_8::to_json_value(&document).unwrap(), value);
}

#[test]
fn nested_schedule_elements_follow_schema_order() {
    let mut value = input("0.8.0");
    value["schedule"]["scheduleElements"] = json!([{
        "id": "lesson-1", "course": {"refId": "math"}, "type": "lesson",
        "temporalExpressions": [{"type": "onetime", "startTimepoint": "2026-09-01T08:00:00+02:00", "endTimepoint": "2026-09-01T09:00:00+02:00", "operation": "include"}]
    }]);
    let document: opent8::v0_8::Document = serde_json::from_value(value).unwrap();
    let output = opent8::v0_8::to_json_value(&document).unwrap();
    let schema: Value = serde_json::from_str(opent8::v0_8::SCHEMA).unwrap();
    assert!(
        jsonschema::options()
            .should_validate_formats(true)
            .build(&schema)
            .unwrap()
            .is_valid(&output)
    );
    let lesson = &output["schedule"]["scheduleElements"][0];
    let keys: Vec<_> = lesson
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(&keys[..3], ["type", "id", "course"]);
    assert_eq!(
        lesson["temporalExpressions"][0]["startTimepoint"],
        "2026-09-01T08:00:00+02:00"
    );
}
