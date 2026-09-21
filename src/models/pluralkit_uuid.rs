use serde::{Deserialize, Serialize};
#[cfg(feature = "sqlx")]
use sqlx::postgres::PgHasArrayType;
use std::{fmt::Display, hash::Hash, marker::PhantomData, ops::Deref};
use uuid::Uuid;

#[derive(Debug)]
pub struct PluralKitUuid<T> {
    value: Uuid,
    marker: PhantomData<T>,
}

impl<T> PluralKitUuid<T> {
    fn new(value: Uuid) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }
}

impl<T> Clone for PluralKitUuid<T> {
    fn clone(&self) -> Self {
        *self
    }
}

// NOTE: Uuid's internal buffer is 16 bytes, same as a u128 which is also `Copy`
impl<T> Copy for PluralKitUuid<T> {}

impl<T> Deref for PluralKitUuid<T> {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> Display for PluralKitUuid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> PartialEq for PluralKitUuid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<T> Eq for PluralKitUuid<T> {}

impl<T> PartialOrd for PluralKitUuid<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for PluralKitUuid<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> Hash for PluralKitUuid<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.value.as_bytes());
    }
}

impl<T> From<Uuid> for PluralKitUuid<T> {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl<T> TryFrom<String> for PluralKitUuid<T> {
    type Error = uuid::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl<T> TryFrom<&str> for PluralKitUuid<T> {
    type Error = uuid::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Uuid::try_from(value).map(Self::new)
    }
}

impl<'de, T> Deserialize<'de> for PluralKitUuid<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Uuid::deserialize(deserializer).map(Self::new)
    }
}

impl<T> Serialize for PluralKitUuid<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

#[cfg(test)]
mod test {
    use crate::models::marker::SystemMarker;

    use super::*;

    #[test]
    fn test_serialize_pluralkit_uuid() {
        assert_eq!(
            serde_json::to_string(&PluralKitUuid::<SystemMarker> {
                value: Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa").unwrap(),
                marker: PhantomData,
            })
            .unwrap(),
            "\"aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa\""
        );
    }

    #[test]
    fn test_deserialize_pluralkit_uuid() {
        assert_eq!(
            serde_json::from_str::<PluralKitUuid<SystemMarker>>(
                "\"aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa\""
            )
            .unwrap()
            .value,
            Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa").unwrap()
        );
    }
}
