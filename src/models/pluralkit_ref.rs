use std::{fmt::Display, str::FromStr};

use serde::Serialize;

use crate::models::{
    marker::{GroupMarker, MemberMarker, SystemMarker},
    pluralkit_id::PluralKitId,
    pluralkit_uuid::PluralKitUuid,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum SystemRef {
    #[serde(rename = "@me")]
    Me,
    #[serde(untagged)]
    Id(PluralKitId<SystemMarker>),
    #[serde(untagged)]
    Uuid(PluralKitUuid<SystemMarker>),

    /// discord snowflake
    #[serde(untagged)]
    Snowflake(String),
}

impl Display for SystemRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Me => f.write_str("@me"),
            Self::Id(id) => id.fmt(f),
            Self::Uuid(uuid) => uuid.fmt(f),
            Self::Snowflake(snowflake) => snowflake.fmt(f),
        }
    }
}

impl From<PluralKitId<SystemMarker>> for SystemRef {
    fn from(value: PluralKitId<SystemMarker>) -> Self {
        Self::Id(value)
    }
}

impl From<PluralKitUuid<SystemMarker>> for SystemRef {
    fn from(value: PluralKitUuid<SystemMarker>) -> Self {
        Self::Uuid(value)
    }
}

impl TryFrom<String> for SystemRef {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl TryFrom<&str> for SystemRef {
    type Error = crate::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "@me" {
            return Ok(SystemRef::Me);
        }

        if let Ok(id) = PluralKitId::try_from(value) {
            return Ok(id.into());
        }

        if let Ok(uuid) = PluralKitUuid::try_from(value) {
            return Ok(uuid.into());
        }

        if let Ok(snowflake) = value.parse::<u64>()
            && snowflake > 0
        {
            return Ok(Self::Snowflake(value.to_string()));
        }

        Err(format!("no valid system ref in string {value}").into())
    }
}

impl FromStr for SystemRef {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum GroupRef {
    Id(PluralKitId<GroupMarker>),
    Uuid(PluralKitUuid<GroupMarker>),
}

impl Display for GroupRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => id.fmt(f),
            Self::Uuid(uuid) => uuid.fmt(f),
        }
    }
}

impl From<PluralKitId<GroupMarker>> for GroupRef {
    fn from(value: PluralKitId<GroupMarker>) -> Self {
        Self::Id(value)
    }
}

impl From<PluralKitUuid<GroupMarker>> for GroupRef {
    fn from(value: PluralKitUuid<GroupMarker>) -> Self {
        Self::Uuid(value)
    }
}

impl TryFrom<String> for GroupRef {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl TryFrom<&str> for GroupRef {
    type Error = crate::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(id) = PluralKitId::try_from(value) {
            return Ok(id.into());
        }

        if let Ok(uuid) = PluralKitUuid::try_from(value) {
            return Ok(uuid.into());
        }

        Err(format!("no valid group ref in string {value}").into())
    }
}

impl FromStr for GroupRef {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum MemberRef {
    Id(PluralKitId<MemberMarker>),
    Uuid(PluralKitUuid<MemberMarker>),
}

impl Display for MemberRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => id.fmt(f),
            Self::Uuid(uuid) => uuid.fmt(f),
        }
    }
}

impl From<PluralKitId<MemberMarker>> for MemberRef {
    fn from(value: PluralKitId<MemberMarker>) -> Self {
        Self::Id(value)
    }
}

impl From<PluralKitUuid<MemberMarker>> for MemberRef {
    fn from(value: PluralKitUuid<MemberMarker>) -> Self {
        Self::Uuid(value)
    }
}

impl TryFrom<String> for MemberRef {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl TryFrom<&str> for MemberRef {
    type Error = crate::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(id) = PluralKitId::try_from(value) {
            return Ok(id.into());
        }

        if let Ok(uuid) = PluralKitUuid::try_from(value) {
            return Ok(uuid.into());
        }

        Err(format!("no valid member ref in string {value}").into())
    }
}

impl FromStr for MemberRef {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // SystemRef
    #[test]
    fn try_from_id() {
        let id = "exmpl";
        let expected_id = PluralKitId::try_from(id).expect("failed to parse string to id");

        assert_eq!(
            SystemRef::try_from(id).expect("failed to parse id into SystemRef"),
            SystemRef::Id(expected_id)
        )
    }

    #[test]
    fn try_from_uuid() {
        let uuid = "00000000-0000-0000-0000-000000000000";
        let expected_uuid = PluralKitUuid::try_from(uuid).expect("error parsing uuid");

        assert_eq!(
            SystemRef::try_from(uuid).expect("failed to parse uuid into SystemRef"),
            SystemRef::Uuid(expected_uuid)
        )
    }

    #[test]
    fn try_from_snowflake() {
        let snowflake = "1";

        assert_eq!(
            SystemRef::try_from(snowflake).expect("failed to parse snowflake into SystemRef"),
            SystemRef::Snowflake(snowflake.to_string())
        )
    }

    #[test]
    fn try_from_invalid_fails() {
        assert!(SystemRef::try_from("not-valid").is_err())
    }

    #[test]
    fn serialize_system_ref_me() {
        assert_eq!(
            serde_json::to_string(&SystemRef::Me).expect("error serializing"),
            "\"@me\""
        );
    }
    #[test]
    fn serialize_system_ref_id() {
        assert_eq!(
            serde_json::to_string(&SystemRef::Id(
                "exmpl".try_into().expect("error parsing system id")
            ))
            .expect("error serializing"),
            "\"exmpl\""
        );
    }
    #[test]
    fn serialize_system_ref_uuid() {
        assert_eq!(
            serde_json::to_string(&SystemRef::Uuid(
                PluralKitUuid::try_from("00000000-0000-0000-0000-000000000000")
                    .expect("error parsing uuid")
            ))
            .expect("error serializing"),
            "\"00000000-0000-0000-0000-000000000000\""
        );
    }

    // GroupRef
    #[test]
    fn serialize_group_ref_id() {
        assert_eq!(
            serde_json::to_string(&GroupRef::Id(
                "exmpl".try_into().expect("error parsing group id")
            ))
            .expect("error serializing"),
            "\"exmpl\""
        );
    }
    #[test]
    fn serialize_group_ref_uuid() {
        assert_eq!(
            serde_json::to_string(&GroupRef::Uuid(
                PluralKitUuid::try_from("00000000-0000-0000-0000-000000000000")
                    .expect("error parsing uuid")
            ))
            .expect("error serializing"),
            "\"00000000-0000-0000-0000-000000000000\""
        );
    }

    // MemberRef
    #[test]
    fn serialize_member_ref_id() {
        assert_eq!(
            serde_json::to_string(&MemberRef::Id(
                "exmpl".try_into().expect("error parsing member id")
            ))
            .expect("error serializing"),
            "\"exmpl\""
        );
    }
    #[test]
    fn serialize_member_ref_uuid() {
        assert_eq!(
            serde_json::to_string(&MemberRef::Uuid(
                PluralKitUuid::try_from("00000000-0000-0000-0000-000000000000")
                    .expect("error parsing uuid")
            ))
            .expect("error serializing"),
            "\"00000000-0000-0000-0000-000000000000\""
        );
    }
}
