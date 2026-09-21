use std::default::Default;

use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Privacy {
    Public,
    Private,
    #[default]
    Unset,
}

impl Serialize for Privacy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Public => serializer.serialize_str("public"),
            Self::Private => serializer.serialize_str("private"),
            Self::Unset => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for Privacy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_option(PrivacyVisitor)
    }
}

struct PrivacyVisitor;
impl<'de> Visitor<'de> for PrivacyVisitor {
    type Value = Privacy;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("null, \"public\" or \"private\"")
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(Self)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Privacy::Unset)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "public" => Ok(Privacy::Public),
            "private" => Ok(Privacy::Private),
            _ => Err(de::Error::unknown_variant(
                v,
                &["null", "\"public\"", "\"private\""],
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_public() {
        assert_eq!(
            serde_json::from_str::<Privacy>("\"public\"").unwrap(),
            Privacy::Public
        )
    }
    #[test]
    fn test_deserialize_private() {
        assert_eq!(
            serde_json::from_str::<Privacy>("\"private\"").unwrap(),
            Privacy::Private
        )
    }
    #[test]
    fn test_deserialize_unset() {
        assert_eq!(
            serde_json::from_str::<Privacy>("null").unwrap(),
            Privacy::Unset
        )
    }
    #[test]
    fn test_deserialize_invalid() {
        assert!(
            serde_json::from_str::<Privacy>("\"invalid\"")
                .unwrap_err()
                .to_string()
                .starts_with("unknown variant")
        )
    }

    #[test]
    fn test_serialize_public() {
        assert_eq!(
            serde_json::to_string(&Privacy::Public).unwrap(),
            "\"public\""
        )
    }

    #[test]
    fn test_serialize_private() {
        assert_eq!(
            serde_json::to_string(&Privacy::Private).unwrap(),
            "\"private\""
        )
    }

    #[test]
    fn test_serialize_unset() {
        assert_eq!(serde_json::to_string(&Privacy::Unset).unwrap(), "null")
    }
}
