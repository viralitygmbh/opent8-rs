// SPDX-License-Identifier: Apache-2.0
// Generated from OpenPotato / STÜBER SYSTEMS OpenT8 schemas; see schema/sources.json.
// Adapted with cargo-typify 0.8.0: const enums, date unions, fixed-offset timestamps.
// Do not edit; regenerate with scripts/generate-opent8.sh.
#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = "An absence as gap reason (e.g. absence of a teacher)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Absence {
    #[doc = "Reference by id to an absence type"]
    #[serde(
        rename = "absenceType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub absence_type: ::std::option::Option<ReferenceById>,
    #[serde(
        rename = "appliesTo",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub applies_to: ::std::option::Option<AbsenceAppliesTo>,
    #[doc = "Additional notes on the absence"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "The type keyword of the gap reason"]
    #[serde(rename = "type")]
    pub type_: AbsenceKind,
}
impl Absence {
    pub fn builder() -> builder::Absence {
        Default::default()
    }
}
#[doc = "A person, a group or a room for which the absence applies"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AbsenceAppliesTo {
    #[doc = "Reference by id"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
    #[doc = "Reference type"]
    #[serde(rename = "type")]
    pub type_: AbsenceAppliesToType,
}
impl AbsenceAppliesTo {
    pub fn builder() -> builder::AbsenceAppliesTo {
        Default::default()
    }
}
#[doc = "Reference type"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AbsenceAppliesToType {
    #[serde(rename = "person")]
    Person,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "room")]
    Room,
}
impl ::std::fmt::Display for AbsenceAppliesToType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Person => f.write_str("person"),
            Self::Group => f.write_str("group"),
            Self::Room => f.write_str("room"),
        }
    }
}
impl ::std::str::FromStr for AbsenceAppliesToType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "person" => Ok(Self::Person),
            "group" => Ok(Self::Group),
            "room" => Ok(Self::Room),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AbsenceAppliesToType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AbsenceAppliesToType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type keyword of the gap reason"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AbsenceKind {
    #[serde(rename = "absence")]
    Absence,
}
impl ::std::fmt::Display for AbsenceKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Absence => f.write_str("absence"),
        }
    }
}
impl ::std::str::FromStr for AbsenceKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "absence" => Ok(Self::Absence),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AbsenceKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AbsenceKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An absence type (e.g. illness or school trip)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AbsenceType {
    #[doc = "A standardised value from an external code list that specifies the absence type"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<ExternalCode>,
    #[doc = "A description of the absence type"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identification of the absence type"]
    pub id: ::std::string::String,
    #[doc = "The detailed name of the absence type"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the absence type"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl AbsenceType {
    pub fn builder() -> builder::AbsenceType {
        Default::default()
    }
}
#[doc = "An activity of a person as schedule element (e.g. after-school supervision or standby duty)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Activity {
    #[doc = "Reference by id to an activity type"]
    #[serde(
        rename = "activityType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub activity_type: ::std::option::Option<ReferenceById>,
    #[doc = "A list of attendees"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attendees: ::std::option::Option<::std::vec::Vec<ActivityAttendeesItem>>,
    #[doc = "The color of the announcement as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<ActivityColor>,
    #[doc = "A description of the activity"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "A list of group references"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub groups: ::std::option::Option<::std::vec::Vec<ReferenceById>>,
    #[doc = "The unique identification of the activity"]
    pub id: ::std::string::String,
    #[doc = "The detailed name of the activity"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "A list of room references"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rooms: ::std::option::Option<::std::vec::Vec<ReferenceById>>,
    #[doc = "The short name of the activity"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
    #[serde(
        rename = "temporalExpressions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temporal_expressions: ::std::option::Option<TemporalExpressions>,
    #[doc = "The type keyword of the schedule element"]
    #[serde(rename = "type")]
    pub type_: ActivityKind,
}
impl Activity {
    pub fn builder() -> builder::Activity {
        Default::default()
    }
}
#[doc = "An attendee"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActivityAttendeesItem {
    #[doc = "A reference by id to a person"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
    pub role: ActivityAttendeesItemRole,
}
impl ActivityAttendeesItem {
    pub fn builder() -> builder::ActivityAttendeesItem {
        Default::default()
    }
}
#[doc = "A person role"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ActivityAttendeesItemRole {
    #[doc = "A reference by id to a person role"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
}
impl ActivityAttendeesItemRole {
    pub fn builder() -> builder::ActivityAttendeesItemRole {
        Default::default()
    }
}
#[doc = "The color of the announcement as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ActivityColor(::std::string::String);
impl ::std::ops::Deref for ActivityColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ActivityColor> for ::std::string::String {
    fn from(value: ActivityColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ActivityColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ActivityColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ActivityColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ActivityColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The type keyword of the schedule element"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ActivityKind {
    #[serde(rename = "activity")]
    Activity,
}
impl ::std::fmt::Display for ActivityKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Activity => f.write_str("activity"),
        }
    }
}
impl ::std::str::FromStr for ActivityKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "activity" => Ok(Self::Activity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ActivityKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ActivityKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An activity type (e.g. after-school supervision or standby duty)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActivityType {
    #[doc = "A standardised value from an external code list that specifies the activity type"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<ExternalCode>,
    #[doc = "A description of the activity type"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identification of the activity type"]
    pub id: ::std::string::String,
    #[doc = "The long name of the activity type"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the activity type"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl ActivityType {
    pub fn builder() -> builder::ActivityType {
        Default::default()
    }
}
#[doc = "An announcement as schedule element"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Announcement {
    #[doc = "An optional list of groups and/or persons for which the announcement applies"]
    #[serde(
        rename = "appliesTo",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub applies_to: ::std::option::Option<::std::vec::Vec<AnnouncementAppliesToItem>>,
    #[doc = "The color of the announcement as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<AnnouncementColor>,
    #[doc = "The unique identification of the announcement"]
    pub id: ::std::string::String,
    #[doc = "A more detailed description for the announcement"]
    #[serde(
        rename = "longDescription",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_description: ::std::option::Option<::std::string::String>,
    #[doc = "Additional notes on the announcement"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "The priority of the announcement"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<AnnouncementPriority>,
    #[doc = "A short description for the announcement"]
    #[serde(rename = "shortDescription")]
    pub short_description: ::std::string::String,
    #[serde(
        rename = "temporalExpressions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temporal_expressions: ::std::option::Option<TemporalExpressions>,
    #[doc = "The type keyword of the schedule element"]
    #[serde(rename = "type")]
    pub type_: AnnouncementKind,
}
impl Announcement {
    pub fn builder() -> builder::Announcement {
        Default::default()
    }
}
#[doc = "A person or a group"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AnnouncementAppliesToItem {
    #[doc = "A reference by id to a person or a group"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
    #[doc = "Reference type"]
    #[serde(rename = "type")]
    pub type_: AnnouncementAppliesToItemType,
}
impl AnnouncementAppliesToItem {
    pub fn builder() -> builder::AnnouncementAppliesToItem {
        Default::default()
    }
}
#[doc = "Reference type"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AnnouncementAppliesToItemType {
    #[serde(rename = "person")]
    Person,
    #[serde(rename = "group")]
    Group,
}
impl ::std::fmt::Display for AnnouncementAppliesToItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Person => f.write_str("person"),
            Self::Group => f.write_str("group"),
        }
    }
}
impl ::std::str::FromStr for AnnouncementAppliesToItemType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "person" => Ok(Self::Person),
            "group" => Ok(Self::Group),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AnnouncementAppliesToItemType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AnnouncementAppliesToItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The color of the announcement as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AnnouncementColor(::std::string::String);
impl ::std::ops::Deref for AnnouncementColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AnnouncementColor> for ::std::string::String {
    fn from(value: AnnouncementColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for AnnouncementColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AnnouncementColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AnnouncementColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AnnouncementColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The type keyword of the schedule element"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AnnouncementKind {
    #[serde(rename = "announcement")]
    Announcement,
}
impl ::std::fmt::Display for AnnouncementKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Announcement => f.write_str("announcement"),
        }
    }
}
impl ::std::str::FromStr for AnnouncementKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "announcement" => Ok(Self::Announcement),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AnnouncementKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AnnouncementKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The priority of the announcement"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AnnouncementPriority {
    #[serde(rename = "important")]
    Important,
    #[serde(rename = "alarm")]
    Alarm,
}
impl ::std::fmt::Display for AnnouncementPriority {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Important => f.write_str("important"),
            Self::Alarm => f.write_str("alarm"),
        }
    }
}
impl ::std::str::FromStr for AnnouncementPriority {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "important" => Ok(Self::Important),
            "alarm" => Ok(Self::Alarm),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AnnouncementPriority {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AnnouncementPriority {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A building"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Building {
    #[doc = "Reference by id to a campus"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub campus: ::std::option::Option<ReferenceById>,
    #[doc = "The color of the building as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<BuildingColor>,
    #[doc = "A description of the building"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "A list of external identifiers"]
    #[serde(
        rename = "externalIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_ids: ::std::vec::Vec<ExternalId>,
    #[doc = "The unique identification of the building"]
    pub id: ::std::string::String,
    #[doc = "The long name of the building"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "A short name of the building"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl Building {
    pub fn builder() -> builder::Building {
        Default::default()
    }
}
#[doc = "The color of the building as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BuildingColor(::std::string::String);
impl ::std::ops::Deref for BuildingColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BuildingColor> for ::std::string::String {
    fn from(value: BuildingColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for BuildingColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for BuildingColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BuildingColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for BuildingColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A campus"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Campus {
    #[doc = "The color of the campus as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<CampusColor>,
    #[doc = "A description of the campus"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "A list of external identifiers"]
    #[serde(
        rename = "externalIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_ids: ::std::vec::Vec<ExternalId>,
    #[doc = "The unique identification of the campus"]
    pub id: ::std::string::String,
    #[doc = "The long name of the campus"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "A short name of the campus"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl Campus {
    pub fn builder() -> builder::Campus {
        Default::default()
    }
}
#[doc = "The color of the campus as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CampusColor(::std::string::String);
impl ::std::ops::Deref for CampusColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CampusColor> for ::std::string::String {
    fn from(value: CampusColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CampusColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CampusColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CampusColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CampusColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A cancellation"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Cancellation {
    #[doc = "Cancellation behaviour"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub behaviour: ::std::option::Option<CancellationBehaviour>,
    #[doc = "Additional message for display"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    #[doc = "Additional notes on the cancellation"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "The type keyword of the gap resolution"]
    #[serde(rename = "type")]
    pub type_: CancellationKind,
}
impl Cancellation {
    pub fn builder() -> builder::Cancellation {
        Default::default()
    }
}
#[doc = "Cancellation behaviour"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CancellationBehaviour {
    #[serde(rename = "leaveRoom")]
    LeaveRoom,
    #[serde(rename = "stayInRoom")]
    StayInRoom,
}
impl ::std::fmt::Display for CancellationBehaviour {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LeaveRoom => f.write_str("leaveRoom"),
            Self::StayInRoom => f.write_str("stayInRoom"),
        }
    }
}
impl ::std::str::FromStr for CancellationBehaviour {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "leaveRoom" => Ok(Self::LeaveRoom),
            "stayInRoom" => Ok(Self::StayInRoom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CancellationBehaviour {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CancellationBehaviour {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type keyword of the gap resolution"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CancellationKind {
    #[serde(rename = "cancellation")]
    Cancellation,
}
impl ::std::fmt::Display for CancellationKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cancellation => f.write_str("cancellation"),
        }
    }
}
impl ::std::str::FromStr for CancellationKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "cancellation" => Ok(Self::Cancellation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CancellationKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CancellationKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A course (e.g. Advanced maths course in grade 11)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Course {
    #[doc = "A list of attendees"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attendees: ::std::option::Option<::std::vec::Vec<CourseAttendeesItem>>,
    #[doc = "The color of the course as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<CourseColor>,
    #[doc = "An additional course number"]
    #[serde(
        rename = "courseNo",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub course_no: ::std::option::Option<::std::string::String>,
    #[doc = "Reference by id to a course type"]
    #[serde(
        rename = "courseType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub course_type: ::std::option::Option<ReferenceById>,
    #[doc = "External url of website with additional information"]
    #[serde(
        rename = "courseUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub course_url: ::std::option::Option<::std::string::String>,
    #[doc = "A description of the course"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "A list of external identifiers"]
    #[serde(
        rename = "externalIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_ids: ::std::vec::Vec<ExternalId>,
    #[doc = "A list of group references"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub groups: ::std::option::Option<::std::vec::Vec<ReferenceById>>,
    #[doc = "The unique identification of the course"]
    pub id: ::std::string::String,
    #[doc = "The long name of the course"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the course"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
    #[doc = "Reference by id to a subject"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subject: ::std::option::Option<ReferenceById>,
}
impl Course {
    pub fn builder() -> builder::Course {
        Default::default()
    }
}
#[doc = "An attendee"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CourseAttendeesItem {
    #[doc = "A reference by id to a person"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
    pub role: CourseAttendeesItemRole,
}
impl CourseAttendeesItem {
    pub fn builder() -> builder::CourseAttendeesItem {
        Default::default()
    }
}
#[doc = "A person role"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CourseAttendeesItemRole {
    #[doc = "A reference by id to a person role"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
}
impl CourseAttendeesItemRole {
    pub fn builder() -> builder::CourseAttendeesItemRole {
        Default::default()
    }
}
#[doc = "The color of the course as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CourseColor(::std::string::String);
impl ::std::ops::Deref for CourseColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CourseColor> for ::std::string::String {
    fn from(value: CourseColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CourseColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CourseColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CourseColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CourseColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A course type (e.g. intensive course, basic course etc.)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CourseType {
    #[doc = "A standardised value from an external code list that specifies the course type"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<ExternalCode>,
    #[doc = "A description of the course type"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identification of the course type"]
    pub id: ::std::string::String,
    #[doc = "The detailed name of the course type"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the course type"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl CourseType {
    pub fn builder() -> builder::CourseType {
        Default::default()
    }
}
#[doc = "`CustomExtension`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CustomExtension(pub ::serde_json::Value);
impl ::std::ops::Deref for CustomExtension {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<CustomExtension> for ::serde_json::Value {
    fn from(value: CustomExtension) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for CustomExtension {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
#[doc = "An event as schedule element (e.g. an exam or a meeting)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Event {
    #[doc = "A list of attendees"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attendees: ::std::option::Option<::std::vec::Vec<EventAttendeesItem>>,
    #[doc = "The color of the event as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<EventColor>,
    #[doc = "A description of the event"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Reference by id to an event type"]
    #[serde(
        rename = "eventType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub event_type: ::std::option::Option<ReferenceById>,
    #[doc = "A list of group references"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub groups: ::std::option::Option<::std::vec::Vec<ReferenceById>>,
    #[doc = "The unique identification of the event"]
    pub id: ::std::string::String,
    #[doc = "The detailed name of the event"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "A list of room references"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rooms: ::std::option::Option<::std::vec::Vec<ReferenceById>>,
    #[doc = "The short name of the event"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
    #[serde(
        rename = "temporalExpressions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temporal_expressions: ::std::option::Option<TemporalExpressions>,
    #[doc = "The type keyword of the schedule element"]
    #[serde(rename = "type")]
    pub type_: EventKind,
}
impl Event {
    pub fn builder() -> builder::Event {
        Default::default()
    }
}
#[doc = "An attendee"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventAttendeesItem {
    #[doc = "A reference by id to a person"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
    pub role: EventAttendeesItemRole,
}
impl EventAttendeesItem {
    pub fn builder() -> builder::EventAttendeesItem {
        Default::default()
    }
}
#[doc = "A person role"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct EventAttendeesItemRole {
    #[doc = "A reference by id to a person role"]
    #[serde(
        rename = "refId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ref_id: ::std::option::Option<::std::string::String>,
}
impl EventAttendeesItemRole {
    pub fn builder() -> builder::EventAttendeesItemRole {
        Default::default()
    }
}
#[doc = "The color of the event as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct EventColor(::std::string::String);
impl ::std::ops::Deref for EventColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventColor> for ::std::string::String {
    fn from(value: EventColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EventColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The type keyword of the schedule element"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EventKind {
    #[serde(rename = "event")]
    Event,
}
impl ::std::fmt::Display for EventKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Event => f.write_str("event"),
        }
    }
}
impl ::std::str::FromStr for EventKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "event" => Ok(Self::Event),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An event type (e.g. an exam or a meeting)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventType {
    #[doc = "A standardised value from an external code list that specifies the event type"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<ExternalCode>,
    #[doc = "A description of the event type"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identification of the event type"]
    pub id: ::std::string::String,
    #[doc = "The long name of the event type"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the event type"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl EventType {
    pub fn builder() -> builder::EventType {
        Default::default()
    }
}
#[doc = "An exemption as gap reason (e.g. Teacher exemption)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Exemption {
    #[serde(rename = "appliesTo")]
    pub applies_to: ExemptionAppliesTo,
    #[doc = "Reference by id to an exemption type"]
    #[serde(
        rename = "exemptionType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub exemption_type: ::std::option::Option<ReferenceById>,
    #[doc = "Additional notes on the exemption"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "The type keyword of the gap reason"]
    #[serde(rename = "type")]
    pub type_: ExemptionKind,
}
impl Exemption {
    pub fn builder() -> builder::Exemption {
        Default::default()
    }
}
#[doc = "A reference to a person"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExemptionAppliesTo {
    #[doc = "Reference by id"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
    #[doc = "Reference type"]
    #[serde(rename = "type")]
    pub type_: ExemptionAppliesToType,
}
impl ExemptionAppliesTo {
    pub fn builder() -> builder::ExemptionAppliesTo {
        Default::default()
    }
}
#[doc = "Reference type"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ExemptionAppliesToType {
    #[serde(rename = "person")]
    Person,
}
impl ::std::fmt::Display for ExemptionAppliesToType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Person => f.write_str("person"),
        }
    }
}
impl ::std::str::FromStr for ExemptionAppliesToType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "person" => Ok(Self::Person),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExemptionAppliesToType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExemptionAppliesToType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type keyword of the gap reason"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ExemptionKind {
    #[serde(rename = "exemption")]
    Exemption,
}
impl ::std::fmt::Display for ExemptionKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Exemption => f.write_str("exemption"),
        }
    }
}
impl ::std::str::FromStr for ExemptionKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "exemption" => Ok(Self::Exemption),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExemptionKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExemptionKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An exemption type (e.g. Teacher exemption)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExemptionType {
    #[doc = "A standardised value from an external code list that specifies the exemption type"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<ExternalCode>,
    #[doc = "A description of the exemption type"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identification of the exemption type"]
    pub id: ::std::string::String,
    #[doc = "The detailed name of the exemption type"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the exemption type"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl ExemptionType {
    pub fn builder() -> builder::ExemptionType {
        Default::default()
    }
}
#[doc = "Code from an external code list, in OpenCodeList format"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExternalCode {
    #[serde(rename = "codeListRef")]
    pub code_list_ref: ExternalCodeCodeListRef,
    #[doc = "The key ID in the external code list"]
    #[serde(rename = "keyId")]
    pub key_id: ::std::string::String,
    #[doc = "A code from the external code list"]
    pub value: ::std::string::String,
}
impl ExternalCode {
    pub fn builder() -> builder::ExternalCode {
        Default::default()
    }
}
#[doc = "A reference to the external code list"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExternalCodeCodeListRef {
    #[doc = "Canonical URI which uniquely identifies all versions (collectively) of the external code list"]
    #[serde(rename = "canonicalUri")]
    pub canonical_uri: ::std::string::String,
    #[doc = "Canonical URI which uniquely identifies  a specific version of the external code list"]
    #[serde(
        rename = "canonicalVersionUri",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub canonical_version_uri: ::std::option::Option<::std::string::String>,
    #[doc = "Suggested retrieval locations for the external code list version, in OpenCodeList format"]
    #[serde(
        rename = "locationUrls",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub location_urls: ::std::vec::Vec<::std::string::String>,
}
impl ExternalCodeCodeListRef {
    pub fn builder() -> builder::ExternalCodeCodeListRef {
        Default::default()
    }
}
#[doc = "An external identifier"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExternalId {
    #[doc = "Canonical URI which uniquely identifies the source of the external identifier"]
    #[serde(rename = "canonicalUri")]
    pub canonical_uri: ::std::string::String,
    #[doc = "Indicates whether the external identifier is designed to be globally unique."]
    #[serde(rename = "globallyUnique")]
    pub globally_unique: bool,
    #[doc = "The external identifier value"]
    pub value: ::std::string::String,
}
impl ExternalId {
    pub fn builder() -> builder::ExternalId {
        Default::default()
    }
}
#[doc = "A gap for a lesson, supervision or activity as schedule element"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Gap {
    #[serde(rename = "appliesTo")]
    pub applies_to: GapAppliesTo,
    #[doc = "The unique identification of the gap"]
    pub id: ::std::string::String,
    #[doc = "Additional notes on the gap"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "The reason(s) of the gap"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasons: ::std::option::Option<::std::vec::Vec<GapReasonsItem>>,
    #[doc = "The resolution(s) of the gap"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resolutions: ::std::option::Option<::std::vec::Vec<GapResolutionsItem>>,
    #[serde(
        rename = "temporalExpressions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temporal_expressions: ::std::option::Option<TemporalExpressions>,
    #[doc = "The type keyword of the schedule element"]
    #[serde(rename = "type")]
    pub type_: GapKind,
}
impl Gap {
    pub fn builder() -> builder::Gap {
        Default::default()
    }
}
#[doc = "A reference to an activity, a lesson or a supervision"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GapAppliesTo {
    #[doc = "Reference by id"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
    #[doc = "Reference type"]
    #[serde(rename = "type")]
    pub type_: GapAppliesToType,
}
impl GapAppliesTo {
    pub fn builder() -> builder::GapAppliesTo {
        Default::default()
    }
}
#[doc = "Reference type"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GapAppliesToType {
    #[serde(rename = "activity")]
    Activity,
    #[serde(rename = "lesson")]
    Lesson,
    #[serde(rename = "supervision")]
    Supervision,
}
impl ::std::fmt::Display for GapAppliesToType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Activity => f.write_str("activity"),
            Self::Lesson => f.write_str("lesson"),
            Self::Supervision => f.write_str("supervision"),
        }
    }
}
impl ::std::str::FromStr for GapAppliesToType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "activity" => Ok(Self::Activity),
            "lesson" => Ok(Self::Lesson),
            "supervision" => Ok(Self::Supervision),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GapAppliesToType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GapAppliesToType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type keyword of the schedule element"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GapKind {
    #[serde(rename = "gap")]
    Gap,
}
impl ::std::fmt::Display for GapKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gap => f.write_str("gap"),
        }
    }
}
impl ::std::str::FromStr for GapKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gap" => Ok(Self::Gap),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GapKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GapKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`GapReasonsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum GapReasonsItem {
    Absence(Absence),
    Exemption(Exemption),
}
impl ::std::convert::From<Absence> for GapReasonsItem {
    fn from(value: Absence) -> Self {
        Self::Absence(value)
    }
}
impl ::std::convert::From<Exemption> for GapReasonsItem {
    fn from(value: Exemption) -> Self {
        Self::Exemption(value)
    }
}
#[doc = "`GapResolutionsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum GapResolutionsItem {
    Substitution(Substitution),
    Cancellation(Cancellation),
}
impl ::std::convert::From<Substitution> for GapResolutionsItem {
    fn from(value: Substitution) -> Self {
        Self::Substitution(value)
    }
}
impl ::std::convert::From<Cancellation> for GapResolutionsItem {
    fn from(value: Cancellation) -> Self {
        Self::Cancellation(value)
    }
}
#[doc = "A gender value"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Gender {
    #[doc = "A standardised value from an external code list that specifies the gender"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<ExternalCode>,
    #[doc = "A description of the gender"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identification of the gender"]
    pub id: ::std::string::String,
    #[doc = "The detailed name of the gender"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the gender"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl Gender {
    pub fn builder() -> builder::Gender {
        Default::default()
    }
}
#[doc = "A group (e.g. a class or a study group)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Group {
    #[doc = "The color of the group as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<GroupColor>,
    #[doc = "A description of the group"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "A list of external identifiers"]
    #[serde(
        rename = "externalIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_ids: ::std::vec::Vec<ExternalId>,
    #[doc = "Reference by id to a group type"]
    #[serde(
        rename = "groupType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub group_type: ::std::option::Option<ReferenceById>,
    #[doc = "The unique identification of the group"]
    pub id: ::std::string::String,
    #[doc = "The name of the group"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "A list of members"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub members: ::std::option::Option<::std::vec::Vec<GroupMembersItem>>,
    #[doc = "The code of the group"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
    #[doc = "Reference by id to a time frame"]
    #[serde(
        rename = "timeFrame",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub time_frame: ::std::option::Option<ReferenceById>,
}
impl Group {
    pub fn builder() -> builder::Group {
        Default::default()
    }
}
#[doc = "The color of the group as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct GroupColor(::std::string::String);
impl ::std::ops::Deref for GroupColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<GroupColor> for ::std::string::String {
    fn from(value: GroupColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for GroupColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for GroupColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GroupColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for GroupColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A member"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GroupMembersItem {
    #[doc = "A reference by id to a person"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
    pub role: GroupMembersItemRole,
}
impl GroupMembersItem {
    pub fn builder() -> builder::GroupMembersItem {
        Default::default()
    }
}
#[doc = "A person role"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GroupMembersItemRole {
    #[doc = "A reference by id to a person role"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
}
impl GroupMembersItemRole {
    pub fn builder() -> builder::GroupMembersItemRole {
        Default::default()
    }
}
#[doc = "A group type (e.g. a class or a study group)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GroupType {
    #[doc = "A standardised value from an external code list that specifies the group type"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<ExternalCode>,
    #[doc = "A description of the group type"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identification of the group type"]
    pub id: ::std::string::String,
    #[doc = "The detailed name of the group type"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the group type"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl GroupType {
    pub fn builder() -> builder::GroupType {
        Default::default()
    }
}
#[doc = "A holiday (e.g. summer holidays)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Holiday {
    #[doc = "The color of the holiday as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<HolidayColor>,
    #[doc = "A description of the holiday"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Holiday type"]
    #[serde(rename = "holidayType")]
    pub holiday_type: HolidayHolidayType,
    #[doc = "The unique identification of the holiday"]
    pub id: ::std::string::String,
    #[doc = "The name of the holiday"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The code of the holiday"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
    #[serde(
        rename = "temporalExpressions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temporal_expressions: ::std::option::Option<TemporalExpressions>,
    #[doc = "The type keyword of the schedule element"]
    #[serde(rename = "type")]
    pub type_: HolidayKind,
}
impl Holiday {
    pub fn builder() -> builder::Holiday {
        Default::default()
    }
}
#[doc = "The color of the holiday as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct HolidayColor(::std::string::String);
impl ::std::ops::Deref for HolidayColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<HolidayColor> for ::std::string::String {
    fn from(value: HolidayColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for HolidayColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for HolidayColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HolidayColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for HolidayColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Holiday type"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum HolidayHolidayType {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "school")]
    School,
    #[serde(rename = "custom")]
    Custom,
}
impl ::std::fmt::Display for HolidayHolidayType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Public => f.write_str("public"),
            Self::School => f.write_str("school"),
            Self::Custom => f.write_str("custom"),
        }
    }
}
impl ::std::str::FromStr for HolidayHolidayType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "public" => Ok(Self::Public),
            "school" => Ok(Self::School),
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HolidayHolidayType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HolidayHolidayType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The type keyword of the schedule element"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum HolidayKind {
    #[serde(rename = "holiday")]
    Holiday,
}
impl ::std::fmt::Display for HolidayKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Holiday => f.write_str("holiday"),
        }
    }
}
impl ::std::str::FromStr for HolidayKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "holiday" => Ok(Self::Holiday),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HolidayKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HolidayKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Meta information about the timetable data"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Info {
    #[doc = "A description for the timetable"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The ISO 639-1 language code for the content"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<::std::string::String>,
    #[doc = "The timepoint of the publication"]
    #[serde(
        rename = "publishedAt",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub published_at: ::std::option::Option<::chrono::DateTime<chrono::FixedOffset>>,
    #[doc = "The person responsible for the publication"]
    #[serde(
        rename = "publishedFrom",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub published_from: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<SourceInfo>,
    #[doc = "A summary for the timetable"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[doc = "The title of the timetable"]
    pub title: ::std::string::String,
}
impl Info {
    pub fn builder() -> builder::Info {
        Default::default()
    }
}
#[doc = "A lesson as schedule element"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Lesson {
    #[doc = "A list of attendees"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attendees: ::std::option::Option<::std::vec::Vec<LessonAttendeesItem>>,
    #[doc = "The color of the lesson as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<LessonColor>,
    #[doc = "Reference by id to a course"]
    pub course: ReferenceById,
    #[doc = "A list of group references"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub groups: ::std::option::Option<::std::vec::Vec<ReferenceById>>,
    #[doc = "The unique identification of the lesson"]
    pub id: ::std::string::String,
    #[doc = "Additional notes on the lesson"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "Relevance of the lesson"]
    #[serde(default = "defaults::lesson_relevance")]
    pub relevance: LessonRelevance,
    #[doc = "A list of room references"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rooms: ::std::option::Option<::std::vec::Vec<ReferenceById>>,
    #[doc = "Teaching format (e.g. in-person, remote, hybrid)"]
    #[serde(
        rename = "teachingFormat",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub teaching_format: ::std::option::Option<TeachingFormat>,
    #[serde(
        rename = "temporalExpressions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temporal_expressions: ::std::option::Option<TemporalExpressions>,
    #[doc = "The type keyword of the schedule element"]
    #[serde(rename = "type")]
    pub type_: LessonKind,
}
impl Lesson {
    pub fn builder() -> builder::Lesson {
        Default::default()
    }
}
#[doc = "An attendee"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LessonAttendeesItem {
    #[doc = "A reference by id to a person"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
    pub role: LessonAttendeesItemRole,
}
impl LessonAttendeesItem {
    pub fn builder() -> builder::LessonAttendeesItem {
        Default::default()
    }
}
#[doc = "A person role"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LessonAttendeesItemRole {
    #[doc = "A reference by id to a person role"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
}
impl LessonAttendeesItemRole {
    pub fn builder() -> builder::LessonAttendeesItemRole {
        Default::default()
    }
}
#[doc = "The color of the lesson as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LessonColor(::std::string::String);
impl ::std::ops::Deref for LessonColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LessonColor> for ::std::string::String {
    fn from(value: LessonColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LessonColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LessonColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LessonColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LessonColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The type keyword of the schedule element"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LessonKind {
    #[serde(rename = "lesson")]
    Lesson,
}
impl ::std::fmt::Display for LessonKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Lesson => f.write_str("lesson"),
        }
    }
}
impl ::std::str::FromStr for LessonKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "lesson" => Ok(Self::Lesson),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LessonKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LessonKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Relevance of the lesson"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LessonRelevance {
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "additional")]
    Additional,
    #[serde(rename = "substitution")]
    Substitution,
}
impl ::std::fmt::Display for LessonRelevance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Scheduled => f.write_str("scheduled"),
            Self::Additional => f.write_str("additional"),
            Self::Substitution => f.write_str("substitution"),
        }
    }
}
impl ::std::str::FromStr for LessonRelevance {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "scheduled" => Ok(Self::Scheduled),
            "additional" => Ok(Self::Additional),
            "substitution" => Ok(Self::Substitution),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LessonRelevance {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LessonRelevance {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for LessonRelevance {
    fn default() -> Self {
        LessonRelevance::Scheduled
    }
}
#[doc = "Name of a natural person"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NameOfPerson {
    #[doc = "Declared name (e.g. married name or name for civil partners)"]
    #[serde(
        rename = "declaredName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub declared_name: ::std::option::Option<::std::string::String>,
    #[doc = "Type of the name declaration"]
    #[serde(
        rename = "declaredNameType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub declared_name_type: ::std::option::Option<NameOfPersonDeclaredNameType>,
    #[doc = "Family name or last name"]
    #[serde(
        rename = "familyName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub family_name: ::std::option::Option<::std::string::String>,
    #[doc = "The full name as it should be written"]
    #[serde(
        rename = "fullName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub full_name: ::std::option::Option<::std::string::String>,
    #[doc = "Given name or first name"]
    #[serde(
        rename = "givenName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub given_name: ::std::option::Option<::std::string::String>,
    #[doc = "Middle name(s)"]
    #[serde(
        rename = "middleNames",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub middle_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[doc = "Name suffix(es)"]
    #[serde(
        rename = "nameSuffixes",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub name_suffixes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[doc = "Nickname"]
    #[serde(
        rename = "nickName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub nick_name: ::std::option::Option<::std::string::String>,
    #[doc = "Salutaion(s)"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub salutations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[doc = "A (possible unique) short name or code of the person"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
    #[doc = "Name respresenation for sorting"]
    #[serde(
        rename = "sortingName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sorting_name: ::std::option::Option<::std::string::String>,
    #[doc = "Title (e.g. academic degree)"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
}
impl NameOfPerson {
    pub fn builder() -> builder::NameOfPerson {
        Default::default()
    }
}
#[doc = "Type of the name declaration"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NameOfPersonDeclaredNameType {
    #[serde(rename = "marriedName")]
    MarriedName,
    #[serde(rename = "civilPartnershipName")]
    CivilPartnershipName,
}
impl ::std::fmt::Display for NameOfPersonDeclaredNameType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MarriedName => f.write_str("marriedName"),
            Self::CivilPartnershipName => f.write_str("civilPartnershipName"),
        }
    }
}
impl ::std::str::FromStr for NameOfPersonDeclaredNameType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "marriedName" => Ok(Self::MarriedName),
            "civilPartnershipName" => Ok(Self::CivilPartnershipName),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NameOfPersonDeclaredNameType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NameOfPersonDeclaredNameType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An one-time temporal expression"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OneTimeExpression {
    #[doc = "End timepoint of the temporal expression"]
    #[serde(rename = "endTimepoint")]
    pub end_timepoint: ::chrono::DateTime<chrono::FixedOffset>,
    #[doc = "Should this temporal expression be included or excluded?"]
    #[serde(default = "defaults::one_time_expression_operation")]
    pub operation: OneTimeExpressionOperation,
    #[doc = "Start timepoint of the temporal expression"]
    #[serde(rename = "startTimepoint")]
    pub start_timepoint: ::chrono::DateTime<chrono::FixedOffset>,
    #[doc = "The type keyword of the temporal expression"]
    #[serde(rename = "type")]
    pub type_: OnetimeKind,
}
impl OneTimeExpression {
    pub fn builder() -> builder::OneTimeExpression {
        Default::default()
    }
}
#[doc = "Should this temporal expression be included or excluded?"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum OneTimeExpressionOperation {
    #[serde(rename = "include")]
    Include,
    #[serde(rename = "exclude")]
    Exclude,
}
impl ::std::fmt::Display for OneTimeExpressionOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Include => f.write_str("include"),
            Self::Exclude => f.write_str("exclude"),
        }
    }
}
impl ::std::str::FromStr for OneTimeExpressionOperation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "include" => Ok(Self::Include),
            "exclude" => Ok(Self::Exclude),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OneTimeExpressionOperation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OneTimeExpressionOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for OneTimeExpressionOperation {
    fn default() -> Self {
        OneTimeExpressionOperation::Include
    }
}
#[doc = "The type keyword of the temporal expression"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum OnetimeKind {
    #[serde(rename = "onetime")]
    Onetime,
}
impl ::std::fmt::Display for OnetimeKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Onetime => f.write_str("onetime"),
        }
    }
}
impl ::std::str::FromStr for OnetimeKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "onetime" => Ok(Self::Onetime),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OnetimeKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OnetimeKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The specification of OpenT8 v0.5.1 documents"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OpenT8DocumentSchemaSpecification {
    #[doc = "A list of absence types"]
    #[serde(
        rename = "absenceTypes",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub absence_types: ::std::option::Option<::std::vec::Vec<AbsenceType>>,
    #[doc = "A list of activity types"]
    #[serde(
        rename = "activityTypes",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub activity_types: ::std::option::Option<::std::vec::Vec<ActivityType>>,
    #[doc = "A list of buildings"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub buildings: ::std::option::Option<::std::vec::Vec<Building>>,
    #[doc = "A list of campuses"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub campuses: ::std::option::Option<::std::vec::Vec<Campus>>,
    #[doc = "A list of course types"]
    #[serde(
        rename = "courseTypes",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub course_types: ::std::option::Option<::std::vec::Vec<CourseType>>,
    #[doc = "A list of courses"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub courses: ::std::option::Option<::std::vec::Vec<Course>>,
    #[doc = "A list of event types"]
    #[serde(
        rename = "eventTypes",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub event_types: ::std::option::Option<::std::vec::Vec<EventType>>,
    #[doc = "A list of exemption types"]
    #[serde(
        rename = "exemptionTypes",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub exemption_types: ::std::option::Option<::std::vec::Vec<ExemptionType>>,
    #[doc = "A list of genders"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub genders: ::std::option::Option<::std::vec::Vec<Gender>>,
    #[doc = "A list of group types"]
    #[serde(
        rename = "groupTypes",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub group_types: ::std::option::Option<::std::vec::Vec<GroupType>>,
    #[doc = "A list of groups"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub groups: ::std::option::Option<::std::vec::Vec<Group>>,
    pub info: Info,
    #[doc = "The version of the OpenT8 Document Schema"]
    pub opent8: OpenT8DocumentSchemaSpecificationOpent8,
    #[doc = "A list of person roles"]
    #[serde(
        rename = "personRoles",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub person_roles: ::std::option::Option<::std::vec::Vec<PersonRole>>,
    #[doc = "A list of persons"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub persons: ::std::option::Option<::std::vec::Vec<Person>>,
    #[doc = "A list of rooms"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rooms: ::std::option::Option<::std::vec::Vec<Room>>,
    pub schedule: OpenT8DocumentSchemaSpecificationSchedule,
    #[doc = "A list of subjects"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subjects: ::std::option::Option<::std::vec::Vec<Subject>>,
    #[doc = "A list of supervision areas"]
    #[serde(
        rename = "supervisionAreas",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub supervision_areas: ::std::option::Option<::std::vec::Vec<SupervisionArea>>,
    #[doc = "A list of supervision types"]
    #[serde(
        rename = "supervisionTypes",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub supervision_types: ::std::option::Option<::std::vec::Vec<SupervisionType>>,
    #[doc = "A list of teaching formats"]
    #[serde(
        rename = "teachingFormats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub teaching_formats: ::std::option::Option<::std::vec::Vec<TeachingFormat>>,
    #[doc = "A list of time frames"]
    #[serde(
        rename = "timeFrames",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub time_frames: ::std::option::Option<::std::vec::Vec<TimeFrame>>,
    #[doc = "A list of weeks patterns"]
    #[serde(
        rename = "weeksPatterns",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub weeks_patterns: ::std::option::Option<::std::vec::Vec<WeeksPattern>>,
}
impl OpenT8DocumentSchemaSpecification {
    pub fn builder() -> builder::OpenT8DocumentSchemaSpecification {
        Default::default()
    }
}
#[doc = "The version of the OpenT8 Document Schema"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct OpenT8DocumentSchemaSpecificationOpent8(::std::string::String);
impl ::std::ops::Deref for OpenT8DocumentSchemaSpecificationOpent8 {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<OpenT8DocumentSchemaSpecificationOpent8> for ::std::string::String {
    fn from(value: OpenT8DocumentSchemaSpecificationOpent8) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for OpenT8DocumentSchemaSpecificationOpent8 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^0\\.5\\.\\d+$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^0\\.5\\.\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for OpenT8DocumentSchemaSpecificationOpent8 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OpenT8DocumentSchemaSpecificationOpent8 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for OpenT8DocumentSchemaSpecificationOpent8 {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The schedule definition"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OpenT8DocumentSchemaSpecificationSchedule {
    #[doc = "Reference by id to a default time frame"]
    #[serde(
        rename = "defaultTimeFrame",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_time_frame: ::std::option::Option<ReferenceById>,
    #[doc = "A list of schedule elements"]
    #[serde(
        rename = "scheduleElements",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub schedule_elements:
        ::std::vec::Vec<OpenT8DocumentSchemaSpecificationScheduleScheduleElementsItem>,
    #[doc = "Valid from"]
    #[serde(rename = "validFrom")]
    pub valid_from: ::chrono::DateTime<chrono::FixedOffset>,
    #[doc = "Valid to"]
    #[serde(rename = "validTo")]
    pub valid_to: ::chrono::DateTime<chrono::FixedOffset>,
}
impl OpenT8DocumentSchemaSpecificationSchedule {
    pub fn builder() -> builder::OpenT8DocumentSchemaSpecificationSchedule {
        Default::default()
    }
}
#[doc = "`OpenT8DocumentSchemaSpecificationScheduleScheduleElementsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum OpenT8DocumentSchemaSpecificationScheduleScheduleElementsItem {
    Activity(Activity),
    Announcement(Announcement),
    Event(Event),
    Gap(Gap),
    Holiday(Holiday),
    Lesson(Lesson),
    Supervision(Supervision),
}
impl ::std::convert::From<Activity>
    for OpenT8DocumentSchemaSpecificationScheduleScheduleElementsItem
{
    fn from(value: Activity) -> Self {
        Self::Activity(value)
    }
}
impl ::std::convert::From<Announcement>
    for OpenT8DocumentSchemaSpecificationScheduleScheduleElementsItem
{
    fn from(value: Announcement) -> Self {
        Self::Announcement(value)
    }
}
impl ::std::convert::From<Event> for OpenT8DocumentSchemaSpecificationScheduleScheduleElementsItem {
    fn from(value: Event) -> Self {
        Self::Event(value)
    }
}
impl ::std::convert::From<Gap> for OpenT8DocumentSchemaSpecificationScheduleScheduleElementsItem {
    fn from(value: Gap) -> Self {
        Self::Gap(value)
    }
}
impl ::std::convert::From<Holiday>
    for OpenT8DocumentSchemaSpecificationScheduleScheduleElementsItem
{
    fn from(value: Holiday) -> Self {
        Self::Holiday(value)
    }
}
impl ::std::convert::From<Lesson>
    for OpenT8DocumentSchemaSpecificationScheduleScheduleElementsItem
{
    fn from(value: Lesson) -> Self {
        Self::Lesson(value)
    }
}
impl ::std::convert::From<Supervision>
    for OpenT8DocumentSchemaSpecificationScheduleScheduleElementsItem
{
    fn from(value: Supervision) -> Self {
        Self::Supervision(value)
    }
}
#[doc = "A person (e.g. a teacher or a student)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Person {
    #[doc = "Birth date of the person"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub birthdate: ::std::option::Option<::chrono::naive::NaiveDate>,
    #[doc = "The color of the person as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<PersonColor>,
    #[doc = "A list of external identifiers"]
    #[serde(
        rename = "externalIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_ids: ::std::vec::Vec<ExternalId>,
    #[doc = "Gender of the person"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub gender: ::std::option::Option<Gender>,
    #[doc = "A unique identification of the person"]
    pub id: ::std::string::String,
    #[doc = "Name of the person"]
    pub name: NameOfPerson,
    #[doc = "Reference by id to a time frame"]
    #[serde(
        rename = "timeFrame",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub time_frame: ::std::option::Option<ReferenceById>,
}
impl Person {
    pub fn builder() -> builder::Person {
        Default::default()
    }
}
#[doc = "The color of the person as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PersonColor(::std::string::String);
impl ::std::ops::Deref for PersonColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PersonColor> for ::std::string::String {
    fn from(value: PersonColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PersonColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PersonColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PersonColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PersonColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A role of a person (e.g. a teacher or a student)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PersonRole {
    #[doc = "A standardised value from an external code list that specifies the role"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<ExternalCode>,
    #[doc = "A description of the role"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identification of the role"]
    pub id: ::std::string::String,
    #[doc = "The detailed name of the role"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the grole"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl PersonRole {
    pub fn builder() -> builder::PersonRole {
        Default::default()
    }
}
#[doc = "A reference by id"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ReferenceById {
    #[doc = "The id"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
}
impl ReferenceById {
    pub fn builder() -> builder::ReferenceById {
        Default::default()
    }
}
#[doc = "A room"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Room {
    #[doc = "Reference by id to a building"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub building: ::std::option::Option<ReferenceById>,
    #[doc = "The color of the room as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<RoomColor>,
    #[doc = "A description of the room"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "A list of external identifiers"]
    #[serde(
        rename = "externalIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_ids: ::std::vec::Vec<ExternalId>,
    #[doc = "The unique identification of the room"]
    pub id: ::std::string::String,
    #[doc = "The long name of the room"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the room"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl Room {
    pub fn builder() -> builder::Room {
        Default::default()
    }
}
#[doc = "The color of the room as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RoomColor(::std::string::String);
impl ::std::ops::Deref for RoomColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RoomColor> for ::std::string::String {
    fn from(value: RoomColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RoomColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RoomColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RoomColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RoomColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Information about the source of the timetable data"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SourceInfo {
    #[doc = "Name of the service, application or tool that generated the data"]
    pub name: ::std::string::String,
    #[doc = "More information about the service, application or tool that generated the data"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[doc = "Version of the service, application or tool that generated the data"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
impl SourceInfo {
    pub fn builder() -> builder::SourceInfo {
        Default::default()
    }
}
#[doc = "A subject (e.g. Mathematics)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Subject {
    #[doc = "A standardised value from an external code list that specifies the subject"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<ExternalCode>,
    #[doc = "The color of the subject as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<SubjectColor>,
    #[doc = "A description of the subject"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "A list of external identifiers"]
    #[serde(
        rename = "externalIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_ids: ::std::vec::Vec<ExternalId>,
    #[doc = "The unique identification of the subject"]
    pub id: ::std::string::String,
    #[doc = "The long name of the subject"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the subject"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl Subject {
    pub fn builder() -> builder::Subject {
        Default::default()
    }
}
#[doc = "The color of the subject as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SubjectColor(::std::string::String);
impl ::std::ops::Deref for SubjectColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SubjectColor> for ::std::string::String {
    fn from(value: SubjectColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SubjectColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SubjectColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SubjectColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SubjectColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A substitution as schedule element"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Substitution {
    #[doc = "Additional message for display"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    #[doc = "Additional notes on the substitution"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "realisedBy")]
    pub realised_by: SubstitutionRealisedBy,
    #[doc = "The type keyword of the gap resolution"]
    #[serde(rename = "type")]
    pub type_: SubstitutionKind,
}
impl Substitution {
    pub fn builder() -> builder::Substitution {
        Default::default()
    }
}
#[doc = "The type keyword of the gap resolution"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SubstitutionKind {
    #[serde(rename = "substitution")]
    Substitution,
}
impl ::std::fmt::Display for SubstitutionKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Substitution => f.write_str("substitution"),
        }
    }
}
impl ::std::str::FromStr for SubstitutionKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "substitution" => Ok(Self::Substitution),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SubstitutionKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SubstitutionKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A reference to a lesson, a supervision or an activity"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SubstitutionRealisedBy {
    #[doc = "Reference by id"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
    #[doc = "Reference type"]
    #[serde(rename = "type")]
    pub type_: SubstitutionRealisedByType,
}
impl SubstitutionRealisedBy {
    pub fn builder() -> builder::SubstitutionRealisedBy {
        Default::default()
    }
}
#[doc = "Reference type"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SubstitutionRealisedByType {
    #[serde(rename = "activity")]
    Activity,
    #[serde(rename = "lesson")]
    Lesson,
    #[serde(rename = "supervision")]
    Supervision,
}
impl ::std::fmt::Display for SubstitutionRealisedByType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Activity => f.write_str("activity"),
            Self::Lesson => f.write_str("lesson"),
            Self::Supervision => f.write_str("supervision"),
        }
    }
}
impl ::std::str::FromStr for SubstitutionRealisedByType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "activity" => Ok(Self::Activity),
            "lesson" => Ok(Self::Lesson),
            "supervision" => Ok(Self::Supervision),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SubstitutionRealisedByType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SubstitutionRealisedByType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A supervision as schedule element"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Supervision {
    #[doc = "A list of supervision area references"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub areas: ::std::option::Option<::std::vec::Vec<ReferenceById>>,
    #[doc = "A list of attendees"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub attendees: ::std::option::Option<::std::vec::Vec<SupervisionAttendeesItem>>,
    #[doc = "The color of the supervision as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<SupervisionColor>,
    #[doc = "The unique identification of the supervision"]
    pub id: ::std::string::String,
    #[doc = "Additional notes on the supervision"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "Reference by id to a supervision type"]
    #[serde(
        rename = "supervisionType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub supervision_type: ::std::option::Option<ReferenceById>,
    #[serde(
        rename = "temporalExpressions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temporal_expressions: ::std::option::Option<TemporalExpressions>,
    #[doc = "The type keyword of the schedule element"]
    #[serde(rename = "type")]
    pub type_: SupervisionKind,
}
impl Supervision {
    pub fn builder() -> builder::Supervision {
        Default::default()
    }
}
#[doc = "A supervision area (e.g. schoolyard)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SupervisionArea {
    #[doc = "Reference by id to a campus"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub campus: ::std::option::Option<ReferenceById>,
    #[doc = "The color of the area as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<SupervisionAreaColor>,
    #[doc = "A description of the area"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "A list of external identifiers"]
    #[serde(
        rename = "externalIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_ids: ::std::vec::Vec<ExternalId>,
    #[doc = "The unique identification of the area"]
    pub id: ::std::string::String,
    #[doc = "The long name of the area"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the area"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl SupervisionArea {
    pub fn builder() -> builder::SupervisionArea {
        Default::default()
    }
}
#[doc = "The color of the area as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SupervisionAreaColor(::std::string::String);
impl ::std::ops::Deref for SupervisionAreaColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SupervisionAreaColor> for ::std::string::String {
    fn from(value: SupervisionAreaColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SupervisionAreaColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SupervisionAreaColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SupervisionAreaColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SupervisionAreaColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "An attendee"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SupervisionAttendeesItem {
    #[doc = "A reference by id to a person"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
    pub role: SupervisionAttendeesItemRole,
}
impl SupervisionAttendeesItem {
    pub fn builder() -> builder::SupervisionAttendeesItem {
        Default::default()
    }
}
#[doc = "A person role"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SupervisionAttendeesItemRole {
    #[doc = "A reference by id to a person role"]
    #[serde(rename = "refId")]
    pub ref_id: ::std::string::String,
}
impl SupervisionAttendeesItemRole {
    pub fn builder() -> builder::SupervisionAttendeesItemRole {
        Default::default()
    }
}
#[doc = "The color of the supervision as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SupervisionColor(::std::string::String);
impl ::std::ops::Deref for SupervisionColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SupervisionColor> for ::std::string::String {
    fn from(value: SupervisionColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SupervisionColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SupervisionColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SupervisionColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SupervisionColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The type keyword of the schedule element"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SupervisionKind {
    #[serde(rename = "supervision")]
    Supervision,
}
impl ::std::fmt::Display for SupervisionKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Supervision => f.write_str("supervision"),
        }
    }
}
impl ::std::str::FromStr for SupervisionKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "supervision" => Ok(Self::Supervision),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SupervisionKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SupervisionKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A supervision type (e.g. Break supervision)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SupervisionType {
    #[doc = "A standardised value from an external code list that specifies the supervision type"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<ExternalCode>,
    #[doc = "A description of the supervision type"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identification of the supervision type"]
    pub id: ::std::string::String,
    #[doc = "The long name of the supervision type"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the supervision type"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl SupervisionType {
    pub fn builder() -> builder::SupervisionType {
        Default::default()
    }
}
#[doc = "A teaching format (e.g. in-person, remote, hybrid)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TeachingFormat {
    #[doc = "A standardised value from an external code list that specifies the teaching format"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<ExternalCode>,
    #[doc = "A description of the teaching format"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identification of the teaching format"]
    pub id: ::std::string::String,
    #[doc = "The detailed name of the teaching format"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the teaching format"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
}
impl TeachingFormat {
    pub fn builder() -> builder::TeachingFormat {
        Default::default()
    }
}
#[doc = "A list of temporal expressions"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TemporalExpressions(pub ::std::vec::Vec<TemporalExpressionsItem>);
impl ::std::ops::Deref for TemporalExpressions {
    type Target = ::std::vec::Vec<TemporalExpressionsItem>;
    fn deref(&self) -> &::std::vec::Vec<TemporalExpressionsItem> {
        &self.0
    }
}
impl ::std::convert::From<TemporalExpressions> for ::std::vec::Vec<TemporalExpressionsItem> {
    fn from(value: TemporalExpressions) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<TemporalExpressionsItem>> for TemporalExpressions {
    fn from(value: ::std::vec::Vec<TemporalExpressionsItem>) -> Self {
        Self(value)
    }
}
#[doc = "`TemporalExpressionsItem`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TemporalExpressionsItem {
    OneTimeExpression(OneTimeExpression),
    WeeklyExpression(WeeklyExpression),
}
impl ::std::convert::From<OneTimeExpression> for TemporalExpressionsItem {
    fn from(value: OneTimeExpression) -> Self {
        Self::OneTimeExpression(value)
    }
}
impl ::std::convert::From<WeeklyExpression> for TemporalExpressionsItem {
    fn from(value: WeeklyExpression) -> Self {
        Self::WeeklyExpression(value)
    }
}
#[doc = "A time frame"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TimeFrame {
    #[doc = "A description of the time frame"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identification of the time frame"]
    pub id: ::std::string::String,
    #[doc = "The long name of the time frame"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "Set of valid week days"]
    #[serde(rename = "scopeOfWeek")]
    pub scope_of_week: ::std::vec::Vec<TimeFrameScopeOfWeekItem>,
    #[doc = "The short name of the time frame"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
    #[doc = "Start of the week"]
    #[serde(rename = "startOfWeek", default = "defaults::time_frame_start_of_week")]
    pub start_of_week: TimeFrameStartOfWeek,
    #[doc = "A list of time slots"]
    #[serde(rename = "timeSlots")]
    pub time_slots: ::std::vec::Vec<TimeFrameTimeSlotsItem>,
}
impl TimeFrame {
    pub fn builder() -> builder::TimeFrame {
        Default::default()
    }
}
#[doc = "`TimeFrameScopeOfWeekItem`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TimeFrameScopeOfWeekItem {
    #[serde(rename = "mon")]
    Mon,
    #[serde(rename = "tue")]
    Tue,
    #[serde(rename = "wed")]
    Wed,
    #[serde(rename = "thu")]
    Thu,
    #[serde(rename = "fri")]
    Fri,
    #[serde(rename = "sat")]
    Sat,
    #[serde(rename = "sun")]
    Sun,
}
impl ::std::fmt::Display for TimeFrameScopeOfWeekItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mon => f.write_str("mon"),
            Self::Tue => f.write_str("tue"),
            Self::Wed => f.write_str("wed"),
            Self::Thu => f.write_str("thu"),
            Self::Fri => f.write_str("fri"),
            Self::Sat => f.write_str("sat"),
            Self::Sun => f.write_str("sun"),
        }
    }
}
impl ::std::str::FromStr for TimeFrameScopeOfWeekItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mon" => Ok(Self::Mon),
            "tue" => Ok(Self::Tue),
            "wed" => Ok(Self::Wed),
            "thu" => Ok(Self::Thu),
            "fri" => Ok(Self::Fri),
            "sat" => Ok(Self::Sat),
            "sun" => Ok(Self::Sun),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TimeFrameScopeOfWeekItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TimeFrameScopeOfWeekItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Start of the week"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TimeFrameStartOfWeek {
    #[serde(rename = "mon")]
    Mon,
    #[serde(rename = "tue")]
    Tue,
    #[serde(rename = "wed")]
    Wed,
    #[serde(rename = "thu")]
    Thu,
    #[serde(rename = "fri")]
    Fri,
    #[serde(rename = "sat")]
    Sat,
    #[serde(rename = "sun")]
    Sun,
}
impl ::std::fmt::Display for TimeFrameStartOfWeek {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mon => f.write_str("mon"),
            Self::Tue => f.write_str("tue"),
            Self::Wed => f.write_str("wed"),
            Self::Thu => f.write_str("thu"),
            Self::Fri => f.write_str("fri"),
            Self::Sat => f.write_str("sat"),
            Self::Sun => f.write_str("sun"),
        }
    }
}
impl ::std::str::FromStr for TimeFrameStartOfWeek {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mon" => Ok(Self::Mon),
            "tue" => Ok(Self::Tue),
            "wed" => Ok(Self::Wed),
            "thu" => Ok(Self::Thu),
            "fri" => Ok(Self::Fri),
            "sat" => Ok(Self::Sat),
            "sun" => Ok(Self::Sun),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TimeFrameStartOfWeek {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TimeFrameStartOfWeek {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for TimeFrameStartOfWeek {
    fn default() -> Self {
        TimeFrameStartOfWeek::Mon
    }
}
#[doc = "A time slot for a time frame"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TimeFrameTimeSlotsItem {
    #[doc = "The color of the time slot as hex code"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<TimeFrameTimeSlotsItemColor>,
    #[doc = "End time"]
    #[serde(rename = "endTime")]
    pub end_time: ::std::string::String,
    #[doc = "The long label of the time slot"]
    #[serde(
        rename = "longLabel",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_label: ::std::option::Option<::std::string::String>,
    #[doc = "The short label of the time slot"]
    #[serde(rename = "shortLabel")]
    pub short_label: ::std::string::String,
    #[doc = "Start time"]
    #[serde(rename = "startTime")]
    pub start_time: ::std::string::String,
}
impl TimeFrameTimeSlotsItem {
    pub fn builder() -> builder::TimeFrameTimeSlotsItem {
        Default::default()
    }
}
#[doc = "The color of the time slot as hex code"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TimeFrameTimeSlotsItemColor(::std::string::String);
impl ::std::ops::Deref for TimeFrameTimeSlotsItemColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TimeFrameTimeSlotsItemColor> for ::std::string::String {
    fn from(value: TimeFrameTimeSlotsItemColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TimeFrameTimeSlotsItemColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TimeFrameTimeSlotsItemColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TimeFrameTimeSlotsItemColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TimeFrameTimeSlotsItemColor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A weekly temporal expression"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WeeklyExpression {
    #[doc = "End timepoint of the temporal expression"]
    #[serde(rename = "endTimepoint")]
    pub end_timepoint: ::chrono::DateTime<chrono::FixedOffset>,
    #[doc = "Should this temporal expression be included or excluded?"]
    #[serde(default = "defaults::weekly_expression_operation")]
    pub operation: WeeklyExpressionOperation,
    #[doc = "Start timepoint of the temporal expression"]
    #[serde(rename = "startTimepoint")]
    pub start_timepoint: ::chrono::DateTime<chrono::FixedOffset>,
    #[doc = "The type keyword of the temporal expression"]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<WeeklyKind>,
    #[doc = "Valid from"]
    #[serde(
        rename = "validFrom",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_from: ::std::option::Option<::chrono::naive::NaiveDate>,
    #[doc = "Valid to"]
    #[serde(
        rename = "validTo",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_to: ::std::option::Option<::chrono::naive::NaiveDate>,
    #[doc = "Set of valid date weeks per year (embedded or by reference)"]
    #[serde(
        rename = "validWeeks",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_weeks: ::std::option::Option<WeeklyExpressionValidWeeks>,
}
impl WeeklyExpression {
    pub fn builder() -> builder::WeeklyExpression {
        Default::default()
    }
}
#[doc = "Should this temporal expression be included or excluded?"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WeeklyExpressionOperation {
    #[serde(rename = "include")]
    Include,
    #[serde(rename = "exclude")]
    Exclude,
}
impl ::std::fmt::Display for WeeklyExpressionOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Include => f.write_str("include"),
            Self::Exclude => f.write_str("exclude"),
        }
    }
}
impl ::std::str::FromStr for WeeklyExpressionOperation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "include" => Ok(Self::Include),
            "exclude" => Ok(Self::Exclude),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WeeklyExpressionOperation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WeeklyExpressionOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for WeeklyExpressionOperation {
    fn default() -> Self {
        WeeklyExpressionOperation::Include
    }
}
#[doc = "Set of valid date weeks per year (embedded or by reference)"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum WeeklyExpressionValidWeeks {
    Weeks(Weeks),
    ReferenceById(ReferenceById),
}
impl ::std::convert::From<Weeks> for WeeklyExpressionValidWeeks {
    fn from(value: Weeks) -> Self {
        Self::Weeks(value)
    }
}
impl ::std::convert::From<ReferenceById> for WeeklyExpressionValidWeeks {
    fn from(value: ReferenceById) -> Self {
        Self::ReferenceById(value)
    }
}
#[doc = "The type keyword of the temporal expression"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WeeklyKind {
    #[serde(rename = "weekly")]
    Weekly,
}
impl ::std::fmt::Display for WeeklyKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Weekly => f.write_str("weekly"),
        }
    }
}
impl ::std::str::FromStr for WeeklyKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "weekly" => Ok(Self::Weekly),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WeeklyKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WeeklyKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Set of valid date weeks per year"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Weeks(pub ::std::vec::Vec<WeeksItem>);
impl ::std::ops::Deref for Weeks {
    type Target = ::std::vec::Vec<WeeksItem>;
    fn deref(&self) -> &::std::vec::Vec<WeeksItem> {
        &self.0
    }
}
impl ::std::convert::From<Weeks> for ::std::vec::Vec<WeeksItem> {
    fn from(value: Weeks) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<WeeksItem>> for Weeks {
    fn from(value: ::std::vec::Vec<WeeksItem>) -> Self {
        Self(value)
    }
}
#[doc = "`WeeksItem`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct WeeksItem(::std::string::String);
impl ::std::ops::Deref for WeeksItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<WeeksItem> for ::std::string::String {
    fn from(value: WeeksItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for WeeksItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(
            || {
                :: regress :: Regex :: new ("^\\d+:(([1-4][0-9]?|5[0-3]?|[6-9])|([1-4][0-9]?|5[0-3]?|[6-9])-([1-4][0-9]?|5[0-3]?|[6-9]))(,(([1-4][0-9]?|5[0-3]?|[6-9])|([1-4][0-9]?|5[0-3]?|[6-9])-([1-4][0-9]?|5[0-3]?|[6-9])))*$") . unwrap ()
            },
        );
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^\\d+:(([1-4][0-9]?|5[0-3]?|[6-9])|([1-4][0-9]?|5[0-3]?|[6-9])-([1-4][0-9]?|5[0-3]?|[6-9]))(,(([1-4][0-9]?|5[0-3]?|[6-9])|([1-4][0-9]?|5[0-3]?|[6-9])-([1-4][0-9]?|5[0-3]?|[6-9])))*$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for WeeksItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WeeksItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for WeeksItem {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A week pattern"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WeeksPattern {
    #[doc = "A description of the week pattern"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The unique identification of the week pattern"]
    pub id: ::std::string::String,
    #[doc = "The detailed name of the week pattern"]
    #[serde(
        rename = "longName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[doc = "The short name of the week pattern"]
    #[serde(rename = "shortName")]
    pub short_name: ::std::string::String,
    #[doc = "Set of valid date weeks per year"]
    pub weeks: Weeks,
}
impl WeeksPattern {
    pub fn builder() -> builder::WeeksPattern {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Absence {
        absence_type: ::std::result::Result<
            ::std::option::Option<super::ReferenceById>,
            ::std::string::String,
        >,
        applies_to: ::std::result::Result<
            ::std::option::Option<super::AbsenceAppliesTo>,
            ::std::string::String,
        >,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::AbsenceKind, ::std::string::String>,
    }
    impl ::std::default::Default for Absence {
        fn default() -> Self {
            Self {
                absence_type: Ok(Default::default()),
                applies_to: Ok(Default::default()),
                notes: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Absence {
        pub fn absence_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReferenceById>>,
            T::Error: ::std::fmt::Display,
        {
            self.absence_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for absence_type: {e}"));
            self
        }
        pub fn applies_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AbsenceAppliesTo>>,
            T::Error: ::std::fmt::Display,
        {
            self.applies_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for applies_to: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AbsenceKind>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Absence> for super::Absence {
        type Error = super::error::ConversionError;
        fn try_from(value: Absence) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                absence_type: value.absence_type?,
                applies_to: value.applies_to?,
                notes: value.notes?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Absence> for Absence {
        fn from(value: super::Absence) -> Self {
            Self {
                absence_type: Ok(value.absence_type),
                applies_to: Ok(value.applies_to),
                notes: Ok(value.notes),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AbsenceAppliesTo {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::AbsenceAppliesToType, ::std::string::String>,
    }
    impl ::std::default::Default for AbsenceAppliesTo {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AbsenceAppliesTo {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AbsenceAppliesToType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AbsenceAppliesTo> for super::AbsenceAppliesTo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AbsenceAppliesTo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AbsenceAppliesTo> for AbsenceAppliesTo {
        fn from(value: super::AbsenceAppliesTo) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AbsenceType {
        code: ::std::result::Result<
            ::std::option::Option<super::ExternalCode>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AbsenceType {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl AbsenceType {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExternalCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AbsenceType> for super::AbsenceType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AbsenceType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                description: value.description?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::AbsenceType> for AbsenceType {
        fn from(value: super::AbsenceType) -> Self {
            Self {
                code: Ok(value.code),
                description: Ok(value.description),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Activity {
        activity_type: ::std::result::Result<
            ::std::option::Option<super::ReferenceById>,
            ::std::string::String,
        >,
        attendees: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ActivityAttendeesItem>>,
            ::std::string::String,
        >,
        color: ::std::result::Result<
            ::std::option::Option<super::ActivityColor>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        groups: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        rooms: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        temporal_expressions: ::std::result::Result<
            ::std::option::Option<super::TemporalExpressions>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::ActivityKind, ::std::string::String>,
    }
    impl ::std::default::Default for Activity {
        fn default() -> Self {
            Self {
                activity_type: Ok(Default::default()),
                attendees: Ok(Default::default()),
                color: Ok(Default::default()),
                description: Ok(Default::default()),
                groups: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                rooms: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
                temporal_expressions: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Activity {
        pub fn activity_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReferenceById>>,
            T::Error: ::std::fmt::Display,
        {
            self.activity_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for activity_type: {e}"));
            self
        }
        pub fn attendees<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::ActivityAttendeesItem>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.attendees = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attendees: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ActivityColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn groups<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.groups = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for groups: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn rooms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.rooms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rooms: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
        pub fn temporal_expressions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TemporalExpressions>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporal_expressions = value.try_into().map_err(|e| {
                format!("error converting supplied value for temporal_expressions: {e}")
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActivityKind>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Activity> for super::Activity {
        type Error = super::error::ConversionError;
        fn try_from(value: Activity) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                activity_type: value.activity_type?,
                attendees: value.attendees?,
                color: value.color?,
                description: value.description?,
                groups: value.groups?,
                id: value.id?,
                long_name: value.long_name?,
                rooms: value.rooms?,
                short_name: value.short_name?,
                temporal_expressions: value.temporal_expressions?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Activity> for Activity {
        fn from(value: super::Activity) -> Self {
            Self {
                activity_type: Ok(value.activity_type),
                attendees: Ok(value.attendees),
                color: Ok(value.color),
                description: Ok(value.description),
                groups: Ok(value.groups),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                rooms: Ok(value.rooms),
                short_name: Ok(value.short_name),
                temporal_expressions: Ok(value.temporal_expressions),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActivityAttendeesItem {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<super::ActivityAttendeesItemRole, ::std::string::String>,
    }
    impl ::std::default::Default for ActivityAttendeesItem {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl ActivityAttendeesItem {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActivityAttendeesItemRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActivityAttendeesItem> for super::ActivityAttendeesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActivityAttendeesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::ActivityAttendeesItem> for ActivityAttendeesItem {
        fn from(value: super::ActivityAttendeesItem) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActivityAttendeesItemRole {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ActivityAttendeesItemRole {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
            }
        }
    }
    impl ActivityAttendeesItemRole {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActivityAttendeesItemRole> for super::ActivityAttendeesItemRole {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActivityAttendeesItemRole,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
            })
        }
    }
    impl ::std::convert::From<super::ActivityAttendeesItemRole> for ActivityAttendeesItemRole {
        fn from(value: super::ActivityAttendeesItemRole) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActivityType {
        code: ::std::result::Result<
            ::std::option::Option<super::ExternalCode>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ActivityType {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl ActivityType {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExternalCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActivityType> for super::ActivityType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActivityType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                description: value.description?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::ActivityType> for ActivityType {
        fn from(value: super::ActivityType) -> Self {
            Self {
                code: Ok(value.code),
                description: Ok(value.description),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Announcement {
        applies_to: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::AnnouncementAppliesToItem>>,
            ::std::string::String,
        >,
        color: ::std::result::Result<
            ::std::option::Option<super::AnnouncementColor>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        priority: ::std::result::Result<
            ::std::option::Option<super::AnnouncementPriority>,
            ::std::string::String,
        >,
        short_description: ::std::result::Result<::std::string::String, ::std::string::String>,
        temporal_expressions: ::std::result::Result<
            ::std::option::Option<super::TemporalExpressions>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::AnnouncementKind, ::std::string::String>,
    }
    impl ::std::default::Default for Announcement {
        fn default() -> Self {
            Self {
                applies_to: Ok(Default::default()),
                color: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_description: Ok(Default::default()),
                notes: Ok(Default::default()),
                priority: Ok(Default::default()),
                short_description: Err("no value supplied for short_description".to_string()),
                temporal_expressions: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Announcement {
        pub fn applies_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::AnnouncementAppliesToItem>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.applies_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for applies_to: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AnnouncementColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_description: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn priority<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AnnouncementPriority>>,
            T::Error: ::std::fmt::Display,
        {
            self.priority = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for priority: {e}"));
            self
        }
        pub fn short_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_description: {e}"));
            self
        }
        pub fn temporal_expressions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TemporalExpressions>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporal_expressions = value.try_into().map_err(|e| {
                format!("error converting supplied value for temporal_expressions: {e}")
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AnnouncementKind>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Announcement> for super::Announcement {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Announcement,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                applies_to: value.applies_to?,
                color: value.color?,
                id: value.id?,
                long_description: value.long_description?,
                notes: value.notes?,
                priority: value.priority?,
                short_description: value.short_description?,
                temporal_expressions: value.temporal_expressions?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Announcement> for Announcement {
        fn from(value: super::Announcement) -> Self {
            Self {
                applies_to: Ok(value.applies_to),
                color: Ok(value.color),
                id: Ok(value.id),
                long_description: Ok(value.long_description),
                notes: Ok(value.notes),
                priority: Ok(value.priority),
                short_description: Ok(value.short_description),
                temporal_expressions: Ok(value.temporal_expressions),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AnnouncementAppliesToItem {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::AnnouncementAppliesToItemType, ::std::string::String>,
    }
    impl ::std::default::Default for AnnouncementAppliesToItem {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AnnouncementAppliesToItem {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AnnouncementAppliesToItemType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AnnouncementAppliesToItem> for super::AnnouncementAppliesToItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AnnouncementAppliesToItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::AnnouncementAppliesToItem> for AnnouncementAppliesToItem {
        fn from(value: super::AnnouncementAppliesToItem) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Building {
        campus: ::std::result::Result<
            ::std::option::Option<super::ReferenceById>,
            ::std::string::String,
        >,
        color: ::std::result::Result<
            ::std::option::Option<super::BuildingColor>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        external_ids:
            ::std::result::Result<::std::vec::Vec<super::ExternalId>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Building {
        fn default() -> Self {
            Self {
                campus: Ok(Default::default()),
                color: Ok(Default::default()),
                description: Ok(Default::default()),
                external_ids: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl Building {
        pub fn campus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReferenceById>>,
            T::Error: ::std::fmt::Display,
        {
            self.campus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for campus: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BuildingColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn external_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ExternalId>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for external_ids: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Building> for super::Building {
        type Error = super::error::ConversionError;
        fn try_from(value: Building) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                campus: value.campus?,
                color: value.color?,
                description: value.description?,
                external_ids: value.external_ids?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::Building> for Building {
        fn from(value: super::Building) -> Self {
            Self {
                campus: Ok(value.campus),
                color: Ok(value.color),
                description: Ok(value.description),
                external_ids: Ok(value.external_ids),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Campus {
        color:
            ::std::result::Result<::std::option::Option<super::CampusColor>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        external_ids:
            ::std::result::Result<::std::vec::Vec<super::ExternalId>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Campus {
        fn default() -> Self {
            Self {
                color: Ok(Default::default()),
                description: Ok(Default::default()),
                external_ids: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl Campus {
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CampusColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn external_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ExternalId>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for external_ids: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Campus> for super::Campus {
        type Error = super::error::ConversionError;
        fn try_from(value: Campus) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                color: value.color?,
                description: value.description?,
                external_ids: value.external_ids?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::Campus> for Campus {
        fn from(value: super::Campus) -> Self {
            Self {
                color: Ok(value.color),
                description: Ok(value.description),
                external_ids: Ok(value.external_ids),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Cancellation {
        behaviour: ::std::result::Result<
            ::std::option::Option<super::CancellationBehaviour>,
            ::std::string::String,
        >,
        message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::CancellationKind, ::std::string::String>,
    }
    impl ::std::default::Default for Cancellation {
        fn default() -> Self {
            Self {
                behaviour: Ok(Default::default()),
                message: Ok(Default::default()),
                notes: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Cancellation {
        pub fn behaviour<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CancellationBehaviour>>,
            T::Error: ::std::fmt::Display,
        {
            self.behaviour = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for behaviour: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CancellationKind>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Cancellation> for super::Cancellation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Cancellation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                behaviour: value.behaviour?,
                message: value.message?,
                notes: value.notes?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Cancellation> for Cancellation {
        fn from(value: super::Cancellation) -> Self {
            Self {
                behaviour: Ok(value.behaviour),
                message: Ok(value.message),
                notes: Ok(value.notes),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Course {
        attendees: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::CourseAttendeesItem>>,
            ::std::string::String,
        >,
        color:
            ::std::result::Result<::std::option::Option<super::CourseColor>, ::std::string::String>,
        course_no: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        course_type: ::std::result::Result<
            ::std::option::Option<super::ReferenceById>,
            ::std::string::String,
        >,
        course_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        external_ids:
            ::std::result::Result<::std::vec::Vec<super::ExternalId>, ::std::string::String>,
        groups: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        subject: ::std::result::Result<
            ::std::option::Option<super::ReferenceById>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Course {
        fn default() -> Self {
            Self {
                attendees: Ok(Default::default()),
                color: Ok(Default::default()),
                course_no: Ok(Default::default()),
                course_type: Ok(Default::default()),
                course_url: Ok(Default::default()),
                description: Ok(Default::default()),
                external_ids: Ok(Default::default()),
                groups: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
                subject: Ok(Default::default()),
            }
        }
    }
    impl Course {
        pub fn attendees<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::CourseAttendeesItem>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.attendees = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attendees: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CourseColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn course_no<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.course_no = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for course_no: {e}"));
            self
        }
        pub fn course_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReferenceById>>,
            T::Error: ::std::fmt::Display,
        {
            self.course_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for course_type: {e}"));
            self
        }
        pub fn course_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.course_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for course_url: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn external_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ExternalId>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for external_ids: {e}"));
            self
        }
        pub fn groups<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.groups = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for groups: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
        pub fn subject<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReferenceById>>,
            T::Error: ::std::fmt::Display,
        {
            self.subject = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subject: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Course> for super::Course {
        type Error = super::error::ConversionError;
        fn try_from(value: Course) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attendees: value.attendees?,
                color: value.color?,
                course_no: value.course_no?,
                course_type: value.course_type?,
                course_url: value.course_url?,
                description: value.description?,
                external_ids: value.external_ids?,
                groups: value.groups?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
                subject: value.subject?,
            })
        }
    }
    impl ::std::convert::From<super::Course> for Course {
        fn from(value: super::Course) -> Self {
            Self {
                attendees: Ok(value.attendees),
                color: Ok(value.color),
                course_no: Ok(value.course_no),
                course_type: Ok(value.course_type),
                course_url: Ok(value.course_url),
                description: Ok(value.description),
                external_ids: Ok(value.external_ids),
                groups: Ok(value.groups),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
                subject: Ok(value.subject),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CourseAttendeesItem {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<super::CourseAttendeesItemRole, ::std::string::String>,
    }
    impl ::std::default::Default for CourseAttendeesItem {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl CourseAttendeesItem {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CourseAttendeesItemRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CourseAttendeesItem> for super::CourseAttendeesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CourseAttendeesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::CourseAttendeesItem> for CourseAttendeesItem {
        fn from(value: super::CourseAttendeesItem) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CourseAttendeesItemRole {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CourseAttendeesItemRole {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
            }
        }
    }
    impl CourseAttendeesItemRole {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CourseAttendeesItemRole> for super::CourseAttendeesItemRole {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CourseAttendeesItemRole,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
            })
        }
    }
    impl ::std::convert::From<super::CourseAttendeesItemRole> for CourseAttendeesItemRole {
        fn from(value: super::CourseAttendeesItemRole) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CourseType {
        code: ::std::result::Result<
            ::std::option::Option<super::ExternalCode>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CourseType {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl CourseType {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExternalCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CourseType> for super::CourseType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CourseType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                description: value.description?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::CourseType> for CourseType {
        fn from(value: super::CourseType) -> Self {
            Self {
                code: Ok(value.code),
                description: Ok(value.description),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Event {
        attendees: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::EventAttendeesItem>>,
            ::std::string::String,
        >,
        color:
            ::std::result::Result<::std::option::Option<super::EventColor>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        event_type: ::std::result::Result<
            ::std::option::Option<super::ReferenceById>,
            ::std::string::String,
        >,
        groups: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        rooms: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        temporal_expressions: ::std::result::Result<
            ::std::option::Option<super::TemporalExpressions>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::EventKind, ::std::string::String>,
    }
    impl ::std::default::Default for Event {
        fn default() -> Self {
            Self {
                attendees: Ok(Default::default()),
                color: Ok(Default::default()),
                description: Ok(Default::default()),
                event_type: Ok(Default::default()),
                groups: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                rooms: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
                temporal_expressions: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Event {
        pub fn attendees<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::EventAttendeesItem>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.attendees = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attendees: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn event_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReferenceById>>,
            T::Error: ::std::fmt::Display,
        {
            self.event_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_type: {e}"));
            self
        }
        pub fn groups<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.groups = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for groups: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn rooms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.rooms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rooms: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
        pub fn temporal_expressions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TemporalExpressions>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporal_expressions = value.try_into().map_err(|e| {
                format!("error converting supplied value for temporal_expressions: {e}")
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventKind>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Event> for super::Event {
        type Error = super::error::ConversionError;
        fn try_from(value: Event) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attendees: value.attendees?,
                color: value.color?,
                description: value.description?,
                event_type: value.event_type?,
                groups: value.groups?,
                id: value.id?,
                long_name: value.long_name?,
                rooms: value.rooms?,
                short_name: value.short_name?,
                temporal_expressions: value.temporal_expressions?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Event> for Event {
        fn from(value: super::Event) -> Self {
            Self {
                attendees: Ok(value.attendees),
                color: Ok(value.color),
                description: Ok(value.description),
                event_type: Ok(value.event_type),
                groups: Ok(value.groups),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                rooms: Ok(value.rooms),
                short_name: Ok(value.short_name),
                temporal_expressions: Ok(value.temporal_expressions),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventAttendeesItem {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<super::EventAttendeesItemRole, ::std::string::String>,
    }
    impl ::std::default::Default for EventAttendeesItem {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl EventAttendeesItem {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventAttendeesItemRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EventAttendeesItem> for super::EventAttendeesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventAttendeesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::EventAttendeesItem> for EventAttendeesItem {
        fn from(value: super::EventAttendeesItem) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventAttendeesItemRole {
        ref_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EventAttendeesItemRole {
        fn default() -> Self {
            Self {
                ref_id: Ok(Default::default()),
            }
        }
    }
    impl EventAttendeesItemRole {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EventAttendeesItemRole> for super::EventAttendeesItemRole {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventAttendeesItemRole,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
            })
        }
    }
    impl ::std::convert::From<super::EventAttendeesItemRole> for EventAttendeesItemRole {
        fn from(value: super::EventAttendeesItemRole) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventType {
        code: ::std::result::Result<
            ::std::option::Option<super::ExternalCode>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for EventType {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl EventType {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExternalCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EventType> for super::EventType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                description: value.description?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::EventType> for EventType {
        fn from(value: super::EventType) -> Self {
            Self {
                code: Ok(value.code),
                description: Ok(value.description),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Exemption {
        applies_to: ::std::result::Result<super::ExemptionAppliesTo, ::std::string::String>,
        exemption_type: ::std::result::Result<
            ::std::option::Option<super::ReferenceById>,
            ::std::string::String,
        >,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::ExemptionKind, ::std::string::String>,
    }
    impl ::std::default::Default for Exemption {
        fn default() -> Self {
            Self {
                applies_to: Err("no value supplied for applies_to".to_string()),
                exemption_type: Ok(Default::default()),
                notes: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Exemption {
        pub fn applies_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExemptionAppliesTo>,
            T::Error: ::std::fmt::Display,
        {
            self.applies_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for applies_to: {e}"));
            self
        }
        pub fn exemption_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReferenceById>>,
            T::Error: ::std::fmt::Display,
        {
            self.exemption_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exemption_type: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExemptionKind>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Exemption> for super::Exemption {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Exemption,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                applies_to: value.applies_to?,
                exemption_type: value.exemption_type?,
                notes: value.notes?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Exemption> for Exemption {
        fn from(value: super::Exemption) -> Self {
            Self {
                applies_to: Ok(value.applies_to),
                exemption_type: Ok(value.exemption_type),
                notes: Ok(value.notes),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExemptionAppliesTo {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::ExemptionAppliesToType, ::std::string::String>,
    }
    impl ::std::default::Default for ExemptionAppliesTo {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ExemptionAppliesTo {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExemptionAppliesToType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ExemptionAppliesTo> for super::ExemptionAppliesTo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExemptionAppliesTo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ExemptionAppliesTo> for ExemptionAppliesTo {
        fn from(value: super::ExemptionAppliesTo) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExemptionType {
        code: ::std::result::Result<
            ::std::option::Option<super::ExternalCode>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ExemptionType {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl ExemptionType {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExternalCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ExemptionType> for super::ExemptionType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExemptionType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                description: value.description?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::ExemptionType> for ExemptionType {
        fn from(value: super::ExemptionType) -> Self {
            Self {
                code: Ok(value.code),
                description: Ok(value.description),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExternalCode {
        code_list_ref: ::std::result::Result<super::ExternalCodeCodeListRef, ::std::string::String>,
        key_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ExternalCode {
        fn default() -> Self {
            Self {
                code_list_ref: Err("no value supplied for code_list_ref".to_string()),
                key_id: Err("no value supplied for key_id".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ExternalCode {
        pub fn code_list_ref<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ExternalCodeCodeListRef>,
            T::Error: ::std::fmt::Display,
        {
            self.code_list_ref = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_list_ref: {e}"));
            self
        }
        pub fn key_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.key_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key_id: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ExternalCode> for super::ExternalCode {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExternalCode,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_list_ref: value.code_list_ref?,
                key_id: value.key_id?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ExternalCode> for ExternalCode {
        fn from(value: super::ExternalCode) -> Self {
            Self {
                code_list_ref: Ok(value.code_list_ref),
                key_id: Ok(value.key_id),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExternalCodeCodeListRef {
        canonical_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        canonical_version_uri: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        location_urls:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for ExternalCodeCodeListRef {
        fn default() -> Self {
            Self {
                canonical_uri: Err("no value supplied for canonical_uri".to_string()),
                canonical_version_uri: Ok(Default::default()),
                location_urls: Ok(Default::default()),
            }
        }
    }
    impl ExternalCodeCodeListRef {
        pub fn canonical_uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.canonical_uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for canonical_uri: {e}"));
            self
        }
        pub fn canonical_version_uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.canonical_version_uri = value.try_into().map_err(|e| {
                format!("error converting supplied value for canonical_version_uri: {e}")
            });
            self
        }
        pub fn location_urls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.location_urls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location_urls: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ExternalCodeCodeListRef> for super::ExternalCodeCodeListRef {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExternalCodeCodeListRef,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                canonical_uri: value.canonical_uri?,
                canonical_version_uri: value.canonical_version_uri?,
                location_urls: value.location_urls?,
            })
        }
    }
    impl ::std::convert::From<super::ExternalCodeCodeListRef> for ExternalCodeCodeListRef {
        fn from(value: super::ExternalCodeCodeListRef) -> Self {
            Self {
                canonical_uri: Ok(value.canonical_uri),
                canonical_version_uri: Ok(value.canonical_version_uri),
                location_urls: Ok(value.location_urls),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExternalId {
        canonical_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        globally_unique: ::std::result::Result<bool, ::std::string::String>,
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ExternalId {
        fn default() -> Self {
            Self {
                canonical_uri: Err("no value supplied for canonical_uri".to_string()),
                globally_unique: Err("no value supplied for globally_unique".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ExternalId {
        pub fn canonical_uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.canonical_uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for canonical_uri: {e}"));
            self
        }
        pub fn globally_unique<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.globally_unique = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for globally_unique: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ExternalId> for super::ExternalId {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExternalId,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                canonical_uri: value.canonical_uri?,
                globally_unique: value.globally_unique?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ExternalId> for ExternalId {
        fn from(value: super::ExternalId) -> Self {
            Self {
                canonical_uri: Ok(value.canonical_uri),
                globally_unique: Ok(value.globally_unique),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Gap {
        applies_to: ::std::result::Result<super::GapAppliesTo, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        reasons: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::GapReasonsItem>>,
            ::std::string::String,
        >,
        resolutions: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::GapResolutionsItem>>,
            ::std::string::String,
        >,
        temporal_expressions: ::std::result::Result<
            ::std::option::Option<super::TemporalExpressions>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::GapKind, ::std::string::String>,
    }
    impl ::std::default::Default for Gap {
        fn default() -> Self {
            Self {
                applies_to: Err("no value supplied for applies_to".to_string()),
                id: Err("no value supplied for id".to_string()),
                notes: Ok(Default::default()),
                reasons: Ok(Default::default()),
                resolutions: Ok(Default::default()),
                temporal_expressions: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Gap {
        pub fn applies_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GapAppliesTo>,
            T::Error: ::std::fmt::Display,
        {
            self.applies_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for applies_to: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn reasons<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::GapReasonsItem>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.reasons = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reasons: {e}"));
            self
        }
        pub fn resolutions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::GapResolutionsItem>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.resolutions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resolutions: {e}"));
            self
        }
        pub fn temporal_expressions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TemporalExpressions>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporal_expressions = value.try_into().map_err(|e| {
                format!("error converting supplied value for temporal_expressions: {e}")
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GapKind>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Gap> for super::Gap {
        type Error = super::error::ConversionError;
        fn try_from(value: Gap) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                applies_to: value.applies_to?,
                id: value.id?,
                notes: value.notes?,
                reasons: value.reasons?,
                resolutions: value.resolutions?,
                temporal_expressions: value.temporal_expressions?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Gap> for Gap {
        fn from(value: super::Gap) -> Self {
            Self {
                applies_to: Ok(value.applies_to),
                id: Ok(value.id),
                notes: Ok(value.notes),
                reasons: Ok(value.reasons),
                resolutions: Ok(value.resolutions),
                temporal_expressions: Ok(value.temporal_expressions),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GapAppliesTo {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::GapAppliesToType, ::std::string::String>,
    }
    impl ::std::default::Default for GapAppliesTo {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl GapAppliesTo {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GapAppliesToType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GapAppliesTo> for super::GapAppliesTo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GapAppliesTo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::GapAppliesTo> for GapAppliesTo {
        fn from(value: super::GapAppliesTo) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Gender {
        code: ::std::result::Result<
            ::std::option::Option<super::ExternalCode>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Gender {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl Gender {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExternalCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Gender> for super::Gender {
        type Error = super::error::ConversionError;
        fn try_from(value: Gender) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                description: value.description?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::Gender> for Gender {
        fn from(value: super::Gender) -> Self {
            Self {
                code: Ok(value.code),
                description: Ok(value.description),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Group {
        color:
            ::std::result::Result<::std::option::Option<super::GroupColor>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        external_ids:
            ::std::result::Result<::std::vec::Vec<super::ExternalId>, ::std::string::String>,
        group_type: ::std::result::Result<
            ::std::option::Option<super::ReferenceById>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        members: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::GroupMembersItem>>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        time_frame: ::std::result::Result<
            ::std::option::Option<super::ReferenceById>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Group {
        fn default() -> Self {
            Self {
                color: Ok(Default::default()),
                description: Ok(Default::default()),
                external_ids: Ok(Default::default()),
                group_type: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                members: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
                time_frame: Ok(Default::default()),
            }
        }
    }
    impl Group {
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::GroupColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn external_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ExternalId>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for external_ids: {e}"));
            self
        }
        pub fn group_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReferenceById>>,
            T::Error: ::std::fmt::Display,
        {
            self.group_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_type: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn members<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::GroupMembersItem>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.members = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for members: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
        pub fn time_frame<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReferenceById>>,
            T::Error: ::std::fmt::Display,
        {
            self.time_frame = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_frame: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Group> for super::Group {
        type Error = super::error::ConversionError;
        fn try_from(value: Group) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                color: value.color?,
                description: value.description?,
                external_ids: value.external_ids?,
                group_type: value.group_type?,
                id: value.id?,
                long_name: value.long_name?,
                members: value.members?,
                short_name: value.short_name?,
                time_frame: value.time_frame?,
            })
        }
    }
    impl ::std::convert::From<super::Group> for Group {
        fn from(value: super::Group) -> Self {
            Self {
                color: Ok(value.color),
                description: Ok(value.description),
                external_ids: Ok(value.external_ids),
                group_type: Ok(value.group_type),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                members: Ok(value.members),
                short_name: Ok(value.short_name),
                time_frame: Ok(value.time_frame),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupMembersItem {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<super::GroupMembersItemRole, ::std::string::String>,
    }
    impl ::std::default::Default for GroupMembersItem {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl GroupMembersItem {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GroupMembersItemRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GroupMembersItem> for super::GroupMembersItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupMembersItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::GroupMembersItem> for GroupMembersItem {
        fn from(value: super::GroupMembersItem) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupMembersItemRole {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GroupMembersItemRole {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
            }
        }
    }
    impl GroupMembersItemRole {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GroupMembersItemRole> for super::GroupMembersItemRole {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupMembersItemRole,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
            })
        }
    }
    impl ::std::convert::From<super::GroupMembersItemRole> for GroupMembersItemRole {
        fn from(value: super::GroupMembersItemRole) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GroupType {
        code: ::std::result::Result<
            ::std::option::Option<super::ExternalCode>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GroupType {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl GroupType {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExternalCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GroupType> for super::GroupType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GroupType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                description: value.description?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::GroupType> for GroupType {
        fn from(value: super::GroupType) -> Self {
            Self {
                code: Ok(value.code),
                description: Ok(value.description),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Holiday {
        color: ::std::result::Result<
            ::std::option::Option<super::HolidayColor>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        holiday_type: ::std::result::Result<super::HolidayHolidayType, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        temporal_expressions: ::std::result::Result<
            ::std::option::Option<super::TemporalExpressions>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::HolidayKind, ::std::string::String>,
    }
    impl ::std::default::Default for Holiday {
        fn default() -> Self {
            Self {
                color: Ok(Default::default()),
                description: Ok(Default::default()),
                holiday_type: Err("no value supplied for holiday_type".to_string()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
                temporal_expressions: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Holiday {
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::HolidayColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn holiday_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::HolidayHolidayType>,
            T::Error: ::std::fmt::Display,
        {
            self.holiday_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for holiday_type: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
        pub fn temporal_expressions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TemporalExpressions>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporal_expressions = value.try_into().map_err(|e| {
                format!("error converting supplied value for temporal_expressions: {e}")
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::HolidayKind>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Holiday> for super::Holiday {
        type Error = super::error::ConversionError;
        fn try_from(value: Holiday) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                color: value.color?,
                description: value.description?,
                holiday_type: value.holiday_type?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
                temporal_expressions: value.temporal_expressions?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Holiday> for Holiday {
        fn from(value: super::Holiday) -> Self {
            Self {
                color: Ok(value.color),
                description: Ok(value.description),
                holiday_type: Ok(value.holiday_type),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
                temporal_expressions: Ok(value.temporal_expressions),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Info {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        language: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        published_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<chrono::FixedOffset>>,
            ::std::string::String,
        >,
        published_from: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        source:
            ::std::result::Result<::std::option::Option<super::SourceInfo>, ::std::string::String>,
        summary: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Info {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                language: Ok(Default::default()),
                published_at: Ok(Default::default()),
                published_from: Ok(Default::default()),
                source: Ok(Default::default()),
                summary: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl Info {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for language: {e}"));
            self
        }
        pub fn published_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<chrono::FixedOffset>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.published_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for published_at: {e}"));
            self
        }
        pub fn published_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.published_from = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for published_from: {e}"));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SourceInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {e}"));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Info> for super::Info {
        type Error = super::error::ConversionError;
        fn try_from(value: Info) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                language: value.language?,
                published_at: value.published_at?,
                published_from: value.published_from?,
                source: value.source?,
                summary: value.summary?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::Info> for Info {
        fn from(value: super::Info) -> Self {
            Self {
                description: Ok(value.description),
                language: Ok(value.language),
                published_at: Ok(value.published_at),
                published_from: Ok(value.published_from),
                source: Ok(value.source),
                summary: Ok(value.summary),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Lesson {
        attendees: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::LessonAttendeesItem>>,
            ::std::string::String,
        >,
        color:
            ::std::result::Result<::std::option::Option<super::LessonColor>, ::std::string::String>,
        course: ::std::result::Result<super::ReferenceById, ::std::string::String>,
        groups: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        relevance: ::std::result::Result<super::LessonRelevance, ::std::string::String>,
        rooms: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
            ::std::string::String,
        >,
        teaching_format: ::std::result::Result<
            ::std::option::Option<super::TeachingFormat>,
            ::std::string::String,
        >,
        temporal_expressions: ::std::result::Result<
            ::std::option::Option<super::TemporalExpressions>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::LessonKind, ::std::string::String>,
    }
    impl ::std::default::Default for Lesson {
        fn default() -> Self {
            Self {
                attendees: Ok(Default::default()),
                color: Ok(Default::default()),
                course: Err("no value supplied for course".to_string()),
                groups: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                notes: Ok(Default::default()),
                relevance: Ok(super::defaults::lesson_relevance()),
                rooms: Ok(Default::default()),
                teaching_format: Ok(Default::default()),
                temporal_expressions: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Lesson {
        pub fn attendees<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::LessonAttendeesItem>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.attendees = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attendees: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LessonColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn course<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReferenceById>,
            T::Error: ::std::fmt::Display,
        {
            self.course = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for course: {e}"));
            self
        }
        pub fn groups<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.groups = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for groups: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn relevance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LessonRelevance>,
            T::Error: ::std::fmt::Display,
        {
            self.relevance = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for relevance: {e}"));
            self
        }
        pub fn rooms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.rooms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rooms: {e}"));
            self
        }
        pub fn teaching_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TeachingFormat>>,
            T::Error: ::std::fmt::Display,
        {
            self.teaching_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for teaching_format: {e}"));
            self
        }
        pub fn temporal_expressions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TemporalExpressions>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporal_expressions = value.try_into().map_err(|e| {
                format!("error converting supplied value for temporal_expressions: {e}")
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LessonKind>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Lesson> for super::Lesson {
        type Error = super::error::ConversionError;
        fn try_from(value: Lesson) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attendees: value.attendees?,
                color: value.color?,
                course: value.course?,
                groups: value.groups?,
                id: value.id?,
                notes: value.notes?,
                relevance: value.relevance?,
                rooms: value.rooms?,
                teaching_format: value.teaching_format?,
                temporal_expressions: value.temporal_expressions?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Lesson> for Lesson {
        fn from(value: super::Lesson) -> Self {
            Self {
                attendees: Ok(value.attendees),
                color: Ok(value.color),
                course: Ok(value.course),
                groups: Ok(value.groups),
                id: Ok(value.id),
                notes: Ok(value.notes),
                relevance: Ok(value.relevance),
                rooms: Ok(value.rooms),
                teaching_format: Ok(value.teaching_format),
                temporal_expressions: Ok(value.temporal_expressions),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LessonAttendeesItem {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<super::LessonAttendeesItemRole, ::std::string::String>,
    }
    impl ::std::default::Default for LessonAttendeesItem {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl LessonAttendeesItem {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LessonAttendeesItemRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LessonAttendeesItem> for super::LessonAttendeesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LessonAttendeesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::LessonAttendeesItem> for LessonAttendeesItem {
        fn from(value: super::LessonAttendeesItem) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LessonAttendeesItemRole {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for LessonAttendeesItemRole {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
            }
        }
    }
    impl LessonAttendeesItemRole {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LessonAttendeesItemRole> for super::LessonAttendeesItemRole {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LessonAttendeesItemRole,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
            })
        }
    }
    impl ::std::convert::From<super::LessonAttendeesItemRole> for LessonAttendeesItemRole {
        fn from(value: super::LessonAttendeesItemRole) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NameOfPerson {
        declared_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        declared_name_type: ::std::result::Result<
            ::std::option::Option<super::NameOfPersonDeclaredNameType>,
            ::std::string::String,
        >,
        family_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        full_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        given_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        middle_names: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
        name_suffixes: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
        nick_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        salutations: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        sorting_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NameOfPerson {
        fn default() -> Self {
            Self {
                declared_name: Ok(Default::default()),
                declared_name_type: Ok(Default::default()),
                family_name: Ok(Default::default()),
                full_name: Ok(Default::default()),
                given_name: Ok(Default::default()),
                middle_names: Ok(Default::default()),
                name_suffixes: Ok(Default::default()),
                nick_name: Ok(Default::default()),
                salutations: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
                sorting_name: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl NameOfPerson {
        pub fn declared_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.declared_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for declared_name: {e}"));
            self
        }
        pub fn declared_name_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NameOfPersonDeclaredNameType>>,
            T::Error: ::std::fmt::Display,
        {
            self.declared_name_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for declared_name_type: {e}")
            });
            self
        }
        pub fn family_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.family_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for family_name: {e}"));
            self
        }
        pub fn full_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.full_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for full_name: {e}"));
            self
        }
        pub fn given_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.given_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for given_name: {e}"));
            self
        }
        pub fn middle_names<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.middle_names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for middle_names: {e}"));
            self
        }
        pub fn name_suffixes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.name_suffixes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name_suffixes: {e}"));
            self
        }
        pub fn nick_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.nick_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nick_name: {e}"));
            self
        }
        pub fn salutations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.salutations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for salutations: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
        pub fn sorting_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.sorting_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sorting_name: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<NameOfPerson> for super::NameOfPerson {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NameOfPerson,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                declared_name: value.declared_name?,
                declared_name_type: value.declared_name_type?,
                family_name: value.family_name?,
                full_name: value.full_name?,
                given_name: value.given_name?,
                middle_names: value.middle_names?,
                name_suffixes: value.name_suffixes?,
                nick_name: value.nick_name?,
                salutations: value.salutations?,
                short_name: value.short_name?,
                sorting_name: value.sorting_name?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::NameOfPerson> for NameOfPerson {
        fn from(value: super::NameOfPerson) -> Self {
            Self {
                declared_name: Ok(value.declared_name),
                declared_name_type: Ok(value.declared_name_type),
                family_name: Ok(value.family_name),
                full_name: Ok(value.full_name),
                given_name: Ok(value.given_name),
                middle_names: Ok(value.middle_names),
                name_suffixes: Ok(value.name_suffixes),
                nick_name: Ok(value.nick_name),
                salutations: Ok(value.salutations),
                short_name: Ok(value.short_name),
                sorting_name: Ok(value.sorting_name),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OneTimeExpression {
        end_timepoint:
            ::std::result::Result<::chrono::DateTime<chrono::FixedOffset>, ::std::string::String>,
        operation: ::std::result::Result<super::OneTimeExpressionOperation, ::std::string::String>,
        start_timepoint:
            ::std::result::Result<::chrono::DateTime<chrono::FixedOffset>, ::std::string::String>,
        type_: ::std::result::Result<super::OnetimeKind, ::std::string::String>,
    }
    impl ::std::default::Default for OneTimeExpression {
        fn default() -> Self {
            Self {
                end_timepoint: Err("no value supplied for end_timepoint".to_string()),
                operation: Ok(super::defaults::one_time_expression_operation()),
                start_timepoint: Err("no value supplied for start_timepoint".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl OneTimeExpression {
        pub fn end_timepoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<chrono::FixedOffset>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_timepoint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_timepoint: {e}"));
            self
        }
        pub fn operation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OneTimeExpressionOperation>,
            T::Error: ::std::fmt::Display,
        {
            self.operation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operation: {e}"));
            self
        }
        pub fn start_timepoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<chrono::FixedOffset>>,
            T::Error: ::std::fmt::Display,
        {
            self.start_timepoint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_timepoint: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OnetimeKind>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OneTimeExpression> for super::OneTimeExpression {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OneTimeExpression,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_timepoint: value.end_timepoint?,
                operation: value.operation?,
                start_timepoint: value.start_timepoint?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::OneTimeExpression> for OneTimeExpression {
        fn from(value: super::OneTimeExpression) -> Self {
            Self {
                end_timepoint: Ok(value.end_timepoint),
                operation: Ok(value.operation),
                start_timepoint: Ok(value.start_timepoint),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpenT8DocumentSchemaSpecification {
        absence_types: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::AbsenceType>>,
            ::std::string::String,
        >,
        activity_types: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ActivityType>>,
            ::std::string::String,
        >,
        buildings: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Building>>,
            ::std::string::String,
        >,
        campuses: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Campus>>,
            ::std::string::String,
        >,
        course_types: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::CourseType>>,
            ::std::string::String,
        >,
        courses: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Course>>,
            ::std::string::String,
        >,
        event_types: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::EventType>>,
            ::std::string::String,
        >,
        exemption_types: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ExemptionType>>,
            ::std::string::String,
        >,
        genders: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Gender>>,
            ::std::string::String,
        >,
        group_types: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::GroupType>>,
            ::std::string::String,
        >,
        groups: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Group>>,
            ::std::string::String,
        >,
        info: ::std::result::Result<super::Info, ::std::string::String>,
        opent8: ::std::result::Result<
            super::OpenT8DocumentSchemaSpecificationOpent8,
            ::std::string::String,
        >,
        person_roles: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::PersonRole>>,
            ::std::string::String,
        >,
        persons: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Person>>,
            ::std::string::String,
        >,
        rooms: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Room>>,
            ::std::string::String,
        >,
        schedule: ::std::result::Result<
            super::OpenT8DocumentSchemaSpecificationSchedule,
            ::std::string::String,
        >,
        subjects: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::Subject>>,
            ::std::string::String,
        >,
        supervision_areas: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::SupervisionArea>>,
            ::std::string::String,
        >,
        supervision_types: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::SupervisionType>>,
            ::std::string::String,
        >,
        teaching_formats: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::TeachingFormat>>,
            ::std::string::String,
        >,
        time_frames: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::TimeFrame>>,
            ::std::string::String,
        >,
        weeks_patterns: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::WeeksPattern>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OpenT8DocumentSchemaSpecification {
        fn default() -> Self {
            Self {
                absence_types: Ok(Default::default()),
                activity_types: Ok(Default::default()),
                buildings: Ok(Default::default()),
                campuses: Ok(Default::default()),
                course_types: Ok(Default::default()),
                courses: Ok(Default::default()),
                event_types: Ok(Default::default()),
                exemption_types: Ok(Default::default()),
                genders: Ok(Default::default()),
                group_types: Ok(Default::default()),
                groups: Ok(Default::default()),
                info: Err("no value supplied for info".to_string()),
                opent8: Err("no value supplied for opent8".to_string()),
                person_roles: Ok(Default::default()),
                persons: Ok(Default::default()),
                rooms: Ok(Default::default()),
                schedule: Err("no value supplied for schedule".to_string()),
                subjects: Ok(Default::default()),
                supervision_areas: Ok(Default::default()),
                supervision_types: Ok(Default::default()),
                teaching_formats: Ok(Default::default()),
                time_frames: Ok(Default::default()),
                weeks_patterns: Ok(Default::default()),
            }
        }
    }
    impl OpenT8DocumentSchemaSpecification {
        pub fn absence_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::AbsenceType>>>,
            T::Error: ::std::fmt::Display,
        {
            self.absence_types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for absence_types: {e}"));
            self
        }
        pub fn activity_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::ActivityType>>>,
            T::Error: ::std::fmt::Display,
        {
            self.activity_types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for activity_types: {e}"));
            self
        }
        pub fn buildings<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Building>>>,
            T::Error: ::std::fmt::Display,
        {
            self.buildings = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for buildings: {e}"));
            self
        }
        pub fn campuses<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Campus>>>,
            T::Error: ::std::fmt::Display,
        {
            self.campuses = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for campuses: {e}"));
            self
        }
        pub fn course_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::CourseType>>>,
            T::Error: ::std::fmt::Display,
        {
            self.course_types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for course_types: {e}"));
            self
        }
        pub fn courses<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Course>>>,
            T::Error: ::std::fmt::Display,
        {
            self.courses = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for courses: {e}"));
            self
        }
        pub fn event_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::EventType>>>,
            T::Error: ::std::fmt::Display,
        {
            self.event_types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_types: {e}"));
            self
        }
        pub fn exemption_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::ExemptionType>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.exemption_types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exemption_types: {e}"));
            self
        }
        pub fn genders<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Gender>>>,
            T::Error: ::std::fmt::Display,
        {
            self.genders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for genders: {e}"));
            self
        }
        pub fn group_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::GroupType>>>,
            T::Error: ::std::fmt::Display,
        {
            self.group_types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_types: {e}"));
            self
        }
        pub fn groups<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Group>>>,
            T::Error: ::std::fmt::Display,
        {
            self.groups = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for groups: {e}"));
            self
        }
        pub fn info<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Info>,
            T::Error: ::std::fmt::Display,
        {
            self.info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for info: {e}"));
            self
        }
        pub fn opent8<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OpenT8DocumentSchemaSpecificationOpent8>,
            T::Error: ::std::fmt::Display,
        {
            self.opent8 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for opent8: {e}"));
            self
        }
        pub fn person_roles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::PersonRole>>>,
            T::Error: ::std::fmt::Display,
        {
            self.person_roles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for person_roles: {e}"));
            self
        }
        pub fn persons<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Person>>>,
            T::Error: ::std::fmt::Display,
        {
            self.persons = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for persons: {e}"));
            self
        }
        pub fn rooms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Room>>>,
            T::Error: ::std::fmt::Display,
        {
            self.rooms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rooms: {e}"));
            self
        }
        pub fn schedule<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OpenT8DocumentSchemaSpecificationSchedule>,
            T::Error: ::std::fmt::Display,
        {
            self.schedule = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schedule: {e}"));
            self
        }
        pub fn subjects<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::Subject>>>,
            T::Error: ::std::fmt::Display,
        {
            self.subjects = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subjects: {e}"));
            self
        }
        pub fn supervision_areas<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::SupervisionArea>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.supervision_areas = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for supervision_areas: {e}"));
            self
        }
        pub fn supervision_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::SupervisionType>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.supervision_types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for supervision_types: {e}"));
            self
        }
        pub fn teaching_formats<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::TeachingFormat>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.teaching_formats = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for teaching_formats: {e}"));
            self
        }
        pub fn time_frames<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::TimeFrame>>>,
            T::Error: ::std::fmt::Display,
        {
            self.time_frames = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_frames: {e}"));
            self
        }
        pub fn weeks_patterns<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::WeeksPattern>>>,
            T::Error: ::std::fmt::Display,
        {
            self.weeks_patterns = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weeks_patterns: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OpenT8DocumentSchemaSpecification>
        for super::OpenT8DocumentSchemaSpecification
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OpenT8DocumentSchemaSpecification,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                absence_types: value.absence_types?,
                activity_types: value.activity_types?,
                buildings: value.buildings?,
                campuses: value.campuses?,
                course_types: value.course_types?,
                courses: value.courses?,
                event_types: value.event_types?,
                exemption_types: value.exemption_types?,
                genders: value.genders?,
                group_types: value.group_types?,
                groups: value.groups?,
                info: value.info?,
                opent8: value.opent8?,
                person_roles: value.person_roles?,
                persons: value.persons?,
                rooms: value.rooms?,
                schedule: value.schedule?,
                subjects: value.subjects?,
                supervision_areas: value.supervision_areas?,
                supervision_types: value.supervision_types?,
                teaching_formats: value.teaching_formats?,
                time_frames: value.time_frames?,
                weeks_patterns: value.weeks_patterns?,
            })
        }
    }
    impl ::std::convert::From<super::OpenT8DocumentSchemaSpecification>
        for OpenT8DocumentSchemaSpecification
    {
        fn from(value: super::OpenT8DocumentSchemaSpecification) -> Self {
            Self {
                absence_types: Ok(value.absence_types),
                activity_types: Ok(value.activity_types),
                buildings: Ok(value.buildings),
                campuses: Ok(value.campuses),
                course_types: Ok(value.course_types),
                courses: Ok(value.courses),
                event_types: Ok(value.event_types),
                exemption_types: Ok(value.exemption_types),
                genders: Ok(value.genders),
                group_types: Ok(value.group_types),
                groups: Ok(value.groups),
                info: Ok(value.info),
                opent8: Ok(value.opent8),
                person_roles: Ok(value.person_roles),
                persons: Ok(value.persons),
                rooms: Ok(value.rooms),
                schedule: Ok(value.schedule),
                subjects: Ok(value.subjects),
                supervision_areas: Ok(value.supervision_areas),
                supervision_types: Ok(value.supervision_types),
                teaching_formats: Ok(value.teaching_formats),
                time_frames: Ok(value.time_frames),
                weeks_patterns: Ok(value.weeks_patterns),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpenT8DocumentSchemaSpecificationSchedule {
        default_time_frame: ::std::result::Result<
            ::std::option::Option<super::ReferenceById>,
            ::std::string::String,
        >,
        schedule_elements: ::std::result::Result<
            ::std::vec::Vec<super::OpenT8DocumentSchemaSpecificationScheduleScheduleElementsItem>,
            ::std::string::String,
        >,
        valid_from:
            ::std::result::Result<::chrono::DateTime<chrono::FixedOffset>, ::std::string::String>,
        valid_to:
            ::std::result::Result<::chrono::DateTime<chrono::FixedOffset>, ::std::string::String>,
    }
    impl ::std::default::Default for OpenT8DocumentSchemaSpecificationSchedule {
        fn default() -> Self {
            Self {
                default_time_frame: Ok(Default::default()),
                schedule_elements: Ok(Default::default()),
                valid_from: Err("no value supplied for valid_from".to_string()),
                valid_to: Err("no value supplied for valid_to".to_string()),
            }
        }
    }
    impl OpenT8DocumentSchemaSpecificationSchedule {
        pub fn default_time_frame<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReferenceById>>,
            T::Error: ::std::fmt::Display,
        {
            self.default_time_frame = value.try_into().map_err(|e| {
                format!("error converting supplied value for default_time_frame: {e}")
            });
            self
        }
        pub fn schedule_elements<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::vec::Vec<
                        super::OpenT8DocumentSchemaSpecificationScheduleScheduleElementsItem,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.schedule_elements = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schedule_elements: {e}"));
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<chrono::FixedOffset>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for valid_from: {e}"));
            self
        }
        pub fn valid_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<chrono::FixedOffset>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for valid_to: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<OpenT8DocumentSchemaSpecificationSchedule>
        for super::OpenT8DocumentSchemaSpecificationSchedule
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OpenT8DocumentSchemaSpecificationSchedule,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default_time_frame: value.default_time_frame?,
                schedule_elements: value.schedule_elements?,
                valid_from: value.valid_from?,
                valid_to: value.valid_to?,
            })
        }
    }
    impl ::std::convert::From<super::OpenT8DocumentSchemaSpecificationSchedule>
        for OpenT8DocumentSchemaSpecificationSchedule
    {
        fn from(value: super::OpenT8DocumentSchemaSpecificationSchedule) -> Self {
            Self {
                default_time_frame: Ok(value.default_time_frame),
                schedule_elements: Ok(value.schedule_elements),
                valid_from: Ok(value.valid_from),
                valid_to: Ok(value.valid_to),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Person {
        birthdate: ::std::result::Result<
            ::std::option::Option<::chrono::naive::NaiveDate>,
            ::std::string::String,
        >,
        color:
            ::std::result::Result<::std::option::Option<super::PersonColor>, ::std::string::String>,
        external_ids:
            ::std::result::Result<::std::vec::Vec<super::ExternalId>, ::std::string::String>,
        gender: ::std::result::Result<::std::option::Option<super::Gender>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<super::NameOfPerson, ::std::string::String>,
        time_frame: ::std::result::Result<
            ::std::option::Option<super::ReferenceById>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Person {
        fn default() -> Self {
            Self {
                birthdate: Ok(Default::default()),
                color: Ok(Default::default()),
                external_ids: Ok(Default::default()),
                gender: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                time_frame: Ok(Default::default()),
            }
        }
    }
    impl Person {
        pub fn birthdate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::chrono::naive::NaiveDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.birthdate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for birthdate: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PersonColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn external_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ExternalId>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for external_ids: {e}"));
            self
        }
        pub fn gender<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Gender>>,
            T::Error: ::std::fmt::Display,
        {
            self.gender = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gender: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NameOfPerson>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn time_frame<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReferenceById>>,
            T::Error: ::std::fmt::Display,
        {
            self.time_frame = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_frame: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Person> for super::Person {
        type Error = super::error::ConversionError;
        fn try_from(value: Person) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                birthdate: value.birthdate?,
                color: value.color?,
                external_ids: value.external_ids?,
                gender: value.gender?,
                id: value.id?,
                name: value.name?,
                time_frame: value.time_frame?,
            })
        }
    }
    impl ::std::convert::From<super::Person> for Person {
        fn from(value: super::Person) -> Self {
            Self {
                birthdate: Ok(value.birthdate),
                color: Ok(value.color),
                external_ids: Ok(value.external_ids),
                gender: Ok(value.gender),
                id: Ok(value.id),
                name: Ok(value.name),
                time_frame: Ok(value.time_frame),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PersonRole {
        code: ::std::result::Result<
            ::std::option::Option<super::ExternalCode>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PersonRole {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl PersonRole {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExternalCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PersonRole> for super::PersonRole {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PersonRole,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                description: value.description?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::PersonRole> for PersonRole {
        fn from(value: super::PersonRole) -> Self {
            Self {
                code: Ok(value.code),
                description: Ok(value.description),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReferenceById {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ReferenceById {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
            }
        }
    }
    impl ReferenceById {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ReferenceById> for super::ReferenceById {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReferenceById,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
            })
        }
    }
    impl ::std::convert::From<super::ReferenceById> for ReferenceById {
        fn from(value: super::ReferenceById) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Room {
        building: ::std::result::Result<
            ::std::option::Option<super::ReferenceById>,
            ::std::string::String,
        >,
        color:
            ::std::result::Result<::std::option::Option<super::RoomColor>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        external_ids:
            ::std::result::Result<::std::vec::Vec<super::ExternalId>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Room {
        fn default() -> Self {
            Self {
                building: Ok(Default::default()),
                color: Ok(Default::default()),
                description: Ok(Default::default()),
                external_ids: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl Room {
        pub fn building<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReferenceById>>,
            T::Error: ::std::fmt::Display,
        {
            self.building = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for building: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RoomColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn external_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ExternalId>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for external_ids: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Room> for super::Room {
        type Error = super::error::ConversionError;
        fn try_from(value: Room) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                building: value.building?,
                color: value.color?,
                description: value.description?,
                external_ids: value.external_ids?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::Room> for Room {
        fn from(value: super::Room) -> Self {
            Self {
                building: Ok(value.building),
                color: Ok(value.color),
                description: Ok(value.description),
                external_ids: Ok(value.external_ids),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SourceInfo {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SourceInfo {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                url: Ok(Default::default()),
                version: Ok(Default::default()),
            }
        }
    }
    impl SourceInfo {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SourceInfo> for super::SourceInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SourceInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                url: value.url?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::SourceInfo> for SourceInfo {
        fn from(value: super::SourceInfo) -> Self {
            Self {
                name: Ok(value.name),
                url: Ok(value.url),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Subject {
        code: ::std::result::Result<
            ::std::option::Option<super::ExternalCode>,
            ::std::string::String,
        >,
        color: ::std::result::Result<
            ::std::option::Option<super::SubjectColor>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        external_ids:
            ::std::result::Result<::std::vec::Vec<super::ExternalId>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Subject {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                color: Ok(Default::default()),
                description: Ok(Default::default()),
                external_ids: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl Subject {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExternalCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SubjectColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn external_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ExternalId>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for external_ids: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Subject> for super::Subject {
        type Error = super::error::ConversionError;
        fn try_from(value: Subject) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                color: value.color?,
                description: value.description?,
                external_ids: value.external_ids?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::Subject> for Subject {
        fn from(value: super::Subject) -> Self {
            Self {
                code: Ok(value.code),
                color: Ok(value.color),
                description: Ok(value.description),
                external_ids: Ok(value.external_ids),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Substitution {
        message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        realised_by: ::std::result::Result<super::SubstitutionRealisedBy, ::std::string::String>,
        type_: ::std::result::Result<super::SubstitutionKind, ::std::string::String>,
    }
    impl ::std::default::Default for Substitution {
        fn default() -> Self {
            Self {
                message: Ok(Default::default()),
                notes: Ok(Default::default()),
                realised_by: Err("no value supplied for realised_by".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Substitution {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn realised_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubstitutionRealisedBy>,
            T::Error: ::std::fmt::Display,
        {
            self.realised_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for realised_by: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubstitutionKind>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Substitution> for super::Substitution {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Substitution,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
                notes: value.notes?,
                realised_by: value.realised_by?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Substitution> for Substitution {
        fn from(value: super::Substitution) -> Self {
            Self {
                message: Ok(value.message),
                notes: Ok(value.notes),
                realised_by: Ok(value.realised_by),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubstitutionRealisedBy {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::SubstitutionRealisedByType, ::std::string::String>,
    }
    impl ::std::default::Default for SubstitutionRealisedBy {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl SubstitutionRealisedBy {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubstitutionRealisedByType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SubstitutionRealisedBy> for super::SubstitutionRealisedBy {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubstitutionRealisedBy,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::SubstitutionRealisedBy> for SubstitutionRealisedBy {
        fn from(value: super::SubstitutionRealisedBy) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Supervision {
        areas: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
            ::std::string::String,
        >,
        attendees: ::std::result::Result<
            ::std::option::Option<::std::vec::Vec<super::SupervisionAttendeesItem>>,
            ::std::string::String,
        >,
        color: ::std::result::Result<
            ::std::option::Option<super::SupervisionColor>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        supervision_type: ::std::result::Result<
            ::std::option::Option<super::ReferenceById>,
            ::std::string::String,
        >,
        temporal_expressions: ::std::result::Result<
            ::std::option::Option<super::TemporalExpressions>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::SupervisionKind, ::std::string::String>,
    }
    impl ::std::default::Default for Supervision {
        fn default() -> Self {
            Self {
                areas: Ok(Default::default()),
                attendees: Ok(Default::default()),
                color: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                notes: Ok(Default::default()),
                supervision_type: Ok(Default::default()),
                temporal_expressions: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Supervision {
        pub fn areas<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::ReferenceById>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.areas = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for areas: {e}"));
            self
        }
        pub fn attendees<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::SupervisionAttendeesItem>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.attendees = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attendees: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SupervisionColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn supervision_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReferenceById>>,
            T::Error: ::std::fmt::Display,
        {
            self.supervision_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for supervision_type: {e}"));
            self
        }
        pub fn temporal_expressions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TemporalExpressions>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporal_expressions = value.try_into().map_err(|e| {
                format!("error converting supplied value for temporal_expressions: {e}")
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SupervisionKind>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Supervision> for super::Supervision {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Supervision,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                areas: value.areas?,
                attendees: value.attendees?,
                color: value.color?,
                id: value.id?,
                notes: value.notes?,
                supervision_type: value.supervision_type?,
                temporal_expressions: value.temporal_expressions?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Supervision> for Supervision {
        fn from(value: super::Supervision) -> Self {
            Self {
                areas: Ok(value.areas),
                attendees: Ok(value.attendees),
                color: Ok(value.color),
                id: Ok(value.id),
                notes: Ok(value.notes),
                supervision_type: Ok(value.supervision_type),
                temporal_expressions: Ok(value.temporal_expressions),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SupervisionArea {
        campus: ::std::result::Result<
            ::std::option::Option<super::ReferenceById>,
            ::std::string::String,
        >,
        color: ::std::result::Result<
            ::std::option::Option<super::SupervisionAreaColor>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        external_ids:
            ::std::result::Result<::std::vec::Vec<super::ExternalId>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SupervisionArea {
        fn default() -> Self {
            Self {
                campus: Ok(Default::default()),
                color: Ok(Default::default()),
                description: Ok(Default::default()),
                external_ids: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl SupervisionArea {
        pub fn campus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ReferenceById>>,
            T::Error: ::std::fmt::Display,
        {
            self.campus = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for campus: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SupervisionAreaColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn external_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ExternalId>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for external_ids: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SupervisionArea> for super::SupervisionArea {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SupervisionArea,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                campus: value.campus?,
                color: value.color?,
                description: value.description?,
                external_ids: value.external_ids?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::SupervisionArea> for SupervisionArea {
        fn from(value: super::SupervisionArea) -> Self {
            Self {
                campus: Ok(value.campus),
                color: Ok(value.color),
                description: Ok(value.description),
                external_ids: Ok(value.external_ids),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SupervisionAttendeesItem {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<super::SupervisionAttendeesItemRole, ::std::string::String>,
    }
    impl ::std::default::Default for SupervisionAttendeesItem {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
                role: Err("no value supplied for role".to_string()),
            }
        }
    }
    impl SupervisionAttendeesItem {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SupervisionAttendeesItemRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SupervisionAttendeesItem> for super::SupervisionAttendeesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SupervisionAttendeesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
                role: value.role?,
            })
        }
    }
    impl ::std::convert::From<super::SupervisionAttendeesItem> for SupervisionAttendeesItem {
        fn from(value: super::SupervisionAttendeesItem) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
                role: Ok(value.role),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SupervisionAttendeesItemRole {
        ref_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SupervisionAttendeesItemRole {
        fn default() -> Self {
            Self {
                ref_id: Err("no value supplied for ref_id".to_string()),
            }
        }
    }
    impl SupervisionAttendeesItemRole {
        pub fn ref_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SupervisionAttendeesItemRole> for super::SupervisionAttendeesItemRole {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SupervisionAttendeesItemRole,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ref_id: value.ref_id?,
            })
        }
    }
    impl ::std::convert::From<super::SupervisionAttendeesItemRole> for SupervisionAttendeesItemRole {
        fn from(value: super::SupervisionAttendeesItemRole) -> Self {
            Self {
                ref_id: Ok(value.ref_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SupervisionType {
        code: ::std::result::Result<
            ::std::option::Option<super::ExternalCode>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SupervisionType {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl SupervisionType {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExternalCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SupervisionType> for super::SupervisionType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SupervisionType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                description: value.description?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::SupervisionType> for SupervisionType {
        fn from(value: super::SupervisionType) -> Self {
            Self {
                code: Ok(value.code),
                description: Ok(value.description),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TeachingFormat {
        code: ::std::result::Result<
            ::std::option::Option<super::ExternalCode>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TeachingFormat {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
            }
        }
    }
    impl TeachingFormat {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ExternalCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TeachingFormat> for super::TeachingFormat {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TeachingFormat,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                description: value.description?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
            })
        }
    }
    impl ::std::convert::From<super::TeachingFormat> for TeachingFormat {
        fn from(value: super::TeachingFormat) -> Self {
            Self {
                code: Ok(value.code),
                description: Ok(value.description),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TimeFrame {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scope_of_week: ::std::result::Result<
            ::std::vec::Vec<super::TimeFrameScopeOfWeekItem>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        start_of_week: ::std::result::Result<super::TimeFrameStartOfWeek, ::std::string::String>,
        time_slots: ::std::result::Result<
            ::std::vec::Vec<super::TimeFrameTimeSlotsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TimeFrame {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                scope_of_week: Err("no value supplied for scope_of_week".to_string()),
                short_name: Err("no value supplied for short_name".to_string()),
                start_of_week: Ok(super::defaults::time_frame_start_of_week()),
                time_slots: Err("no value supplied for time_slots".to_string()),
            }
        }
    }
    impl TimeFrame {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn scope_of_week<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TimeFrameScopeOfWeekItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.scope_of_week = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scope_of_week: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
        pub fn start_of_week<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TimeFrameStartOfWeek>,
            T::Error: ::std::fmt::Display,
        {
            self.start_of_week = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_of_week: {e}"));
            self
        }
        pub fn time_slots<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TimeFrameTimeSlotsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.time_slots = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_slots: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TimeFrame> for super::TimeFrame {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TimeFrame,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                long_name: value.long_name?,
                scope_of_week: value.scope_of_week?,
                short_name: value.short_name?,
                start_of_week: value.start_of_week?,
                time_slots: value.time_slots?,
            })
        }
    }
    impl ::std::convert::From<super::TimeFrame> for TimeFrame {
        fn from(value: super::TimeFrame) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                scope_of_week: Ok(value.scope_of_week),
                short_name: Ok(value.short_name),
                start_of_week: Ok(value.start_of_week),
                time_slots: Ok(value.time_slots),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TimeFrameTimeSlotsItem {
        color: ::std::result::Result<
            ::std::option::Option<super::TimeFrameTimeSlotsItemColor>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_label: ::std::result::Result<::std::string::String, ::std::string::String>,
        start_time: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TimeFrameTimeSlotsItem {
        fn default() -> Self {
            Self {
                color: Ok(Default::default()),
                end_time: Err("no value supplied for end_time".to_string()),
                long_label: Ok(Default::default()),
                short_label: Err("no value supplied for short_label".to_string()),
                start_time: Err("no value supplied for start_time".to_string()),
            }
        }
    }
    impl TimeFrameTimeSlotsItem {
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TimeFrameTimeSlotsItemColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn long_label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_label: {e}"));
            self
        }
        pub fn short_label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_label: {e}"));
            self
        }
        pub fn start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_time: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TimeFrameTimeSlotsItem> for super::TimeFrameTimeSlotsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TimeFrameTimeSlotsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                color: value.color?,
                end_time: value.end_time?,
                long_label: value.long_label?,
                short_label: value.short_label?,
                start_time: value.start_time?,
            })
        }
    }
    impl ::std::convert::From<super::TimeFrameTimeSlotsItem> for TimeFrameTimeSlotsItem {
        fn from(value: super::TimeFrameTimeSlotsItem) -> Self {
            Self {
                color: Ok(value.color),
                end_time: Ok(value.end_time),
                long_label: Ok(value.long_label),
                short_label: Ok(value.short_label),
                start_time: Ok(value.start_time),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WeeklyExpression {
        end_timepoint:
            ::std::result::Result<::chrono::DateTime<chrono::FixedOffset>, ::std::string::String>,
        operation: ::std::result::Result<super::WeeklyExpressionOperation, ::std::string::String>,
        start_timepoint:
            ::std::result::Result<::chrono::DateTime<chrono::FixedOffset>, ::std::string::String>,
        type_:
            ::std::result::Result<::std::option::Option<super::WeeklyKind>, ::std::string::String>,
        valid_from: ::std::result::Result<
            ::std::option::Option<::chrono::naive::NaiveDate>,
            ::std::string::String,
        >,
        valid_to: ::std::result::Result<
            ::std::option::Option<::chrono::naive::NaiveDate>,
            ::std::string::String,
        >,
        valid_weeks: ::std::result::Result<
            ::std::option::Option<super::WeeklyExpressionValidWeeks>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WeeklyExpression {
        fn default() -> Self {
            Self {
                end_timepoint: Err("no value supplied for end_timepoint".to_string()),
                operation: Ok(super::defaults::weekly_expression_operation()),
                start_timepoint: Err("no value supplied for start_timepoint".to_string()),
                type_: Ok(Default::default()),
                valid_from: Ok(Default::default()),
                valid_to: Ok(Default::default()),
                valid_weeks: Ok(Default::default()),
            }
        }
    }
    impl WeeklyExpression {
        pub fn end_timepoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<chrono::FixedOffset>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_timepoint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_timepoint: {e}"));
            self
        }
        pub fn operation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WeeklyExpressionOperation>,
            T::Error: ::std::fmt::Display,
        {
            self.operation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operation: {e}"));
            self
        }
        pub fn start_timepoint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<chrono::FixedOffset>>,
            T::Error: ::std::fmt::Display,
        {
            self.start_timepoint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_timepoint: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WeeklyKind>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::chrono::naive::NaiveDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for valid_from: {e}"));
            self
        }
        pub fn valid_to<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::chrono::naive::NaiveDate>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_to = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for valid_to: {e}"));
            self
        }
        pub fn valid_weeks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WeeklyExpressionValidWeeks>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_weeks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for valid_weeks: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WeeklyExpression> for super::WeeklyExpression {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WeeklyExpression,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                end_timepoint: value.end_timepoint?,
                operation: value.operation?,
                start_timepoint: value.start_timepoint?,
                type_: value.type_?,
                valid_from: value.valid_from?,
                valid_to: value.valid_to?,
                valid_weeks: value.valid_weeks?,
            })
        }
    }
    impl ::std::convert::From<super::WeeklyExpression> for WeeklyExpression {
        fn from(value: super::WeeklyExpression) -> Self {
            Self {
                end_timepoint: Ok(value.end_timepoint),
                operation: Ok(value.operation),
                start_timepoint: Ok(value.start_timepoint),
                type_: Ok(value.type_),
                valid_from: Ok(value.valid_from),
                valid_to: Ok(value.valid_to),
                valid_weeks: Ok(value.valid_weeks),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WeeksPattern {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        long_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        weeks: ::std::result::Result<super::Weeks, ::std::string::String>,
    }
    impl ::std::default::Default for WeeksPattern {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                long_name: Ok(Default::default()),
                short_name: Err("no value supplied for short_name".to_string()),
                weeks: Err("no value supplied for weeks".to_string()),
            }
        }
    }
    impl WeeksPattern {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn long_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
        pub fn weeks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Weeks>,
            T::Error: ::std::fmt::Display,
        {
            self.weeks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weeks: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WeeksPattern> for super::WeeksPattern {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WeeksPattern,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                long_name: value.long_name?,
                short_name: value.short_name?,
                weeks: value.weeks?,
            })
        }
    }
    impl ::std::convert::From<super::WeeksPattern> for WeeksPattern {
        fn from(value: super::WeeksPattern) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                long_name: Ok(value.long_name),
                short_name: Ok(value.short_name),
                weeks: Ok(value.weeks),
            }
        }
    }
}
#[doc = " Generation of default values for serde."]
pub mod defaults {
    pub(super) fn lesson_relevance() -> super::LessonRelevance {
        super::LessonRelevance::Scheduled
    }
    pub(super) fn one_time_expression_operation() -> super::OneTimeExpressionOperation {
        super::OneTimeExpressionOperation::Include
    }
    pub(super) fn time_frame_start_of_week() -> super::TimeFrameStartOfWeek {
        super::TimeFrameStartOfWeek::Mon
    }
    pub(super) fn weekly_expression_operation() -> super::WeeklyExpressionOperation {
        super::WeeklyExpressionOperation::Include
    }
}
#[doc = " Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
