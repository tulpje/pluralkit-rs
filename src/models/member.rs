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
