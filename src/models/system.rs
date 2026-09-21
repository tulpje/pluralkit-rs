use jiff::civil::DateTime;
use serde::{Deserialize, Serialize};

use crate::models::{
    marker::SystemMarker, pluralkit_id::PluralKitId, pluralkit_uuid::PluralKitUuid,
    privacy::Privacy,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct System {
    pub id: PluralKitId<SystemMarker>,
    pub uuid: PluralKitUuid<SystemMarker>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tag: Option<String>,
    pub pronouns: Option<String>,
    pub avatar_url: Option<String>,
    pub banner: Option<String>,
    pub color: Option<String>,
    pub created: DateTime,
    pub privacy: Option<SystemPrivacy>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemPrivacy {
    pub name_privacy: Privacy,
    pub description_privacy: Privacy,
    pub avatar_privacy: Privacy,
    pub banner_privacy: Privacy,
    pub pronoun_privacy: Privacy,
    pub member_list_privacy: Privacy,
    pub group_list_privacy: Privacy,
    pub front_privacy: Privacy,
    pub front_history_privacy: Privacy,
}
