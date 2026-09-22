use core::fmt;
use std::str::FromStr;

use jiff::{Timestamp, civil::DateTime, tz::TimeZone};
use serde::de::Visitor;

// NOTE: custom implementation to add the Z back when serializing
pub(crate) fn serialize<S>(datetime: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&format!("{datetime}Z"))
}

// NOTE: we need a custom serialize implementation because `jiff` does not like
//       parsing datetimes with a trailing Z directly into civil::DateTime
pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_str(DateTimeVisitor)
}

// NOTE: we need a custom serialize implementation because `jiff` does not like
//       parsing datetimes with a trailing Z directly into civil::DateTime
pub(crate) fn deserialize_optional<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_option(OptionalDateTimeVisitor)
}

struct OptionalDateTimeVisitor;
impl<'de> Visitor<'de> for OptionalDateTimeVisitor {
    type Value = Option<DateTime>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an optional datetime formatted as 1970-01-01T00:00:00.000000Z")
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Some(deserialize(deserializer)?))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }
}

struct DateTimeVisitor;
impl<'de> Visitor<'de> for DateTimeVisitor {
    type Value = DateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a datetime formatted as 1970-01-01T00:00:00.000000Z")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_borrowed_str(&v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let timestamp = Timestamp::from_str(v).map_err(|err| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &err.to_string().as_str(),
            )
        })?;

        Ok(timestamp.to_zoned(TimeZone::UTC).datetime())
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize)]
    struct OptionalDateStruct {
        #[serde(deserialize_with = "deserialize_optional")]
        datetime: Option<DateTime>,
    }

    #[test]
    fn test_deserialize_optional_none() {
        let result: OptionalDateStruct =
            serde_json::from_str(r#"{ "datetime": null }"#).expect("failed to deserialize");
        assert!(result.datetime.is_none())
    }

    #[test]
    fn test_deserialize_optional_some() {
        let result: OptionalDateStruct =
            serde_json::from_str(r#"{ "datetime": "1970-01-01T00:00:00.000000Z" }"#)
                .expect("failed to deserialize");

        assert_eq!(
            result.datetime.expect("datetime should not be null"),
            Timestamp::UNIX_EPOCH.to_zoned(TimeZone::UTC).datetime()
        )
    }
}
