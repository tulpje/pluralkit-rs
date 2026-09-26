use std::{fmt::Display, hash::Hash, marker::PhantomData, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct PluralKitId<T> {
    pub(crate) value: String,
    marker: PhantomData<T>,
}

impl<T> PluralKitId<T> {
    pub(crate) fn new(value: String) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }
}

impl<T> Display for PluralKitId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> Clone for PluralKitId<T> {
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<T> PluralKitId<T> {
    pub fn get(&self) -> &str {
        &self.value
    }
}

impl<T> Deref for PluralKitId<T> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> PartialEq for PluralKitId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<T> Eq for PluralKitId<T> {}

impl<T> PartialOrd for PluralKitId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for PluralKitId<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> Hash for PluralKitId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.value.as_bytes());
    }
}

impl<T> TryFrom<&str> for PluralKitId<T> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let min_len = if value.contains('-') { 6 } else { 5 };
        let max_len = if value.contains('-') { 7 } else { 6 };
        let value_len = value.len();

        if !(min_len..=max_len).contains(&value_len) {
            return Err(format!("id length should be 5-7 characters but is {value_len}").into());
        }

        if !value
            .chars()
            .all(|c| char::is_ascii_alphanumeric(&c) || c == '-')
        {
            return Err(
                "id contains invalid characters only a-z, A-Z, 0-9 and - are allowed".into(),
            );
        }

        Ok(PluralKitId::new(
            value.trim().replace("-", "").to_ascii_lowercase(),
        ))
    }
}

impl<T> TryFrom<String> for PluralKitId<T> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        PluralKitId::try_from(value.as_ref())
    }
}

impl<T> FromStr for PluralKitId<T> {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl<T> Serialize for PluralKitId<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.get())
    }
}

#[cfg(test)]
mod test {
    use crate::models::marker::SystemMarker;

    use super::*;

    #[test]
    fn test_valid_lengths() {
        assert_eq!(
            PluralKitId::<SystemMarker>::try_from("aaa-aaa")
                .unwrap()
                .get(),
            "aaaaaa"
        );
        assert_eq!(
            PluralKitId::<SystemMarker>::try_from("aaa-aa")
                .unwrap()
                .get(),
            "aaaaa"
        );
        assert_eq!(
            PluralKitId::<SystemMarker>::try_from("aaaaaa")
                .unwrap()
                .get(),
            "aaaaaa"
        );
        assert_eq!(
            PluralKitId::<SystemMarker>::try_from("aaaaa")
                .unwrap()
                .get(),
            "aaaaa"
        );
    }

    #[test]
    fn test_invalid_lengths() {
        assert!(
            PluralKitId::<SystemMarker>::try_from("aaa-aaaa")
                .unwrap_err()
                .to_string()
                .starts_with("id length")
        );
        assert!(
            PluralKitId::<SystemMarker>::try_from("aa-aa")
                .unwrap_err()
                .to_string()
                .starts_with("id length")
        );
        assert!(
            PluralKitId::<SystemMarker>::try_from("aaaaaaa")
                .unwrap_err()
                .to_string()
                .starts_with("id length")
        );
        assert!(
            PluralKitId::<SystemMarker>::try_from("aaaa")
                .unwrap_err()
                .to_string()
                .starts_with("id length")
        );
    }

    #[test]
    fn test_invalid_characters() {
        assert!(
            PluralKitId::<SystemMarker>::try_from("aa!aa")
                .unwrap_err()
                .to_string()
                .starts_with("id contains invalid")
        );
    }

    #[test]
    fn test_serde_deserialize() {
        assert_eq!(
            serde_json::from_str::<PluralKitId<SystemMarker>>("\"aaa-aa\"")
                .unwrap()
                .get(),
            "aaaaa"
        );
    }

    #[test]
    fn test_serde_serialize() {
        assert_eq!(
            serde_json::to_string(&PluralKitId::<SystemMarker> {
                value: "aaaaa".to_string(),
                marker: PhantomData,
            })
            .unwrap(),
            "\"aaaaa\""
        );
    }

    #[test]
    fn test_serde_deserialize_invalid() {
        assert!(
            serde_json::from_str::<PluralKitId<SystemMarker>>("\"aa!aa\"")
                .unwrap_err()
                .to_string()
                .starts_with("id contains invalid")
        );
    }
}
