use std::str::FromStr;

use jiff::civil::DateTime;
use serde::{Deserialize, Serialize};

use crate::models::{
    marker::{MemberMarker, SystemMarker},
    pluralkit_id::PluralKitId,
    pluralkit_uuid::PluralKitUuid,
    privacy::Privacy,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Member {
    pub id: PluralKitId<MemberMarker>,
    pub uuid: PluralKitUuid<MemberMarker>,
    #[serde(default)]
    pub system: Option<PluralKitId<SystemMarker>>,
    pub name: String,
    pub display_name: Option<String>,
    pub color: Option<String>,
    pub birthday: Option<String>,
    pub pronouns: Option<String>,
    pub avatar_url: Option<String>,
    pub webhook_avatar_url: Option<String>,
    pub banner: Option<String>,
    pub description: Option<String>,
    #[serde(deserialize_with = "crate::models::datetime::deserialize_optional")]
    pub created: Option<DateTime>,
    pub proxy_tags: Vec<ProxyTag>,
    pub keep_proxy: bool,
    pub tts: bool,
    pub autoproxy_enabled: Option<bool>,
    pub message_count: Option<u64>,
    #[serde(deserialize_with = "crate::models::datetime::deserialize_optional")]
    pub last_message_timestamp: Option<DateTime>,
    pub privacy: Option<MemberPrivacy>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProxyTag {
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

impl ProxyTag {
    // NOTE: `ProxyTag::new` should be private to avoid invalid state of
    //        both prefix and suffix being None
    fn new(prefix: Option<String>, suffix: Option<String>) -> Self {
        assert!(
            prefix.is_some() || suffix.is_some(),
            "prefix and suffix can't both be `None`"
        );

        Self { prefix, suffix }
    }

    pub fn with_prefix(prefix: impl Into<String>) -> Self {
        Self::new(Some(prefix.into()), None)
    }

    pub fn with_suffix(suffix: impl Into<String>) -> Self {
        Self::new(None, Some(suffix.into()))
    }

    pub fn with_prefix_suffix(prefix: impl Into<String>, suffix: impl Into<String>) -> Self {
        Self::new(Some(prefix.into()), Some(suffix.into()))
    }
}

impl TryFrom<&str> for ProxyTag {
    type Error = crate::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split_once("text") {
            Some(("", "")) | None => Err(format!("invalid proxy tag string: {value}").into()),
            Some((prefix, "")) => Ok(Self {
                prefix: Some(prefix.to_string()),
                suffix: None,
            }),
            Some(("", suffix)) => Ok(Self {
                prefix: None,
                suffix: Some(suffix.to_string()),
            }),
            Some((prefix, suffix)) => Ok(Self {
                prefix: Some(prefix.to_string()),
                suffix: Some(suffix.to_string()),
            }),
        }
    }
}

impl TryFrom<String> for ProxyTag {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl FromStr for ProxyTag {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemberPrivacy {
    pub visibility: Privacy,
    pub name_privacy: Privacy,
    pub description_privacy: Privacy,
    pub birthday_privacy: Privacy,
    pub pronoun_privacy: Privacy,
    pub avatar_privacy: Privacy,
    pub banner_privacy: Privacy,
    pub metadata_privacy: Privacy,
    pub proxy_privacy: Privacy,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_proxy_tag_errors_empty_suffix_prefix() {
        assert!(ProxyTag::try_from("text").is_err())
    }

    #[test]
    fn test_proxy_tag_errors_invalid() {
        assert!(ProxyTag::try_from("foo").is_err())
    }

    #[test]
    fn test_proxy_tag_parses_suffix_prefix() {
        assert_eq!(
            ProxyTag::try_from("foo/text/bar").unwrap(),
            ProxyTag::with_prefix_suffix("foo/", "/bar")
        )
    }

    #[test]
    fn test_proxy_tag_parses_suffix() {
        assert_eq!(
            ProxyTag::try_from("text/bar").unwrap(),
            ProxyTag::with_suffix("/bar")
        )
    }

    #[test]
    fn test_proxy_tag_parses_prefix() {
        assert_eq!(
            ProxyTag::try_from("foo/text").unwrap(),
            ProxyTag::with_prefix("foo/")
        )
    }

    #[test]
    fn test_proxy_tag_implements_fromstr_correctly() {
        let tag: ProxyTag = "foo/text".parse().unwrap();
        assert_eq!(tag, ProxyTag::with_prefix("foo/"));
    }

    #[test]
    fn test_proxy_tag_implements_tryfrom_string_correctly() {
        assert_eq!(
            ProxyTag::try_from(String::from("foo/text")).unwrap(),
            ProxyTag::with_prefix("foo/")
        );
    }
}
