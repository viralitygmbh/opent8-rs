// SPDX-License-Identifier: MIT
use serde_json::json;

macro_rules! strict_fields_test {
    ($module:ident, $version:literal) => {
        #[test]
        fn $module() {
            use opent8::$module as api;

            let mut original = json!({
                "opent8": $version,
                "info": {"title": "Test"},
                "schedule": {"validFrom": "2026-09-01", "validTo": "2027-08-01"}
            });
            if $version == "0.8.0" {
                original["schedule"]["timeZone"] = json!("Europe/Berlin");
            }
            assert!(serde_json::from_value::<api::Document>(original.clone()).is_ok());

            for (object, field) in [("", "unknownRoot"), ("/info", "titel"),
                ("/info", "x-vendor"), ("/schedule", "x-schedule")] {
                let mut value = original.clone();
                value.pointer_mut(object).unwrap().as_object_mut().unwrap()
                    .insert(field.into(), json!({"nested": [1, 2]}));
                assert!(serde_json::from_value::<api::Document>(value).is_err(),
                    "{object}/{field} should be rejected");
            }
        }
    };
}
strict_fields_test!(v0_7, "0.7.0");
strict_fields_test!(v0_8, "0.8.0");
