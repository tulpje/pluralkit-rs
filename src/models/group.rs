use jiff::civil::DateTime;
use serde::{Deserialize, Serialize};

use crate::models::{
    marker::{GroupMarker, MemberMarker, SystemMarker},
    pluralkit_id::PluralKitId,
    pluralkit_uuid::PluralKitUuid,
    privacy::Privacy,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Group {
    pub id: PluralKitId<GroupMarker>,
    pub uuid: PluralKitUuid<GroupMarker>,
    #[serde(default)]
    pub system: Option<PluralKitId<SystemMarker>>,
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub created: Option<DateTime>,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub color: Option<String>,
    pub privacy: GroupPrivacy,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GroupWithMembers {
    pub id: PluralKitId<GroupMarker>,
    pub uuid: PluralKitUuid<GroupMarker>,
    #[serde(default)]
    pub system: Option<PluralKitId<SystemMarker>>,
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    #[serde(deserialize_with = "crate::models::datetime::deserialize_optional")]
    pub created: Option<DateTime>,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub color: Option<String>,
    pub privacy: GroupPrivacy,
    pub members: Vec<PluralKitUuid<MemberMarker>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GroupPrivacy {
    pub name_privacy: Privacy,
    pub description_privacy: Privacy,
    pub banner_privacy: Privacy,
    pub icon_privacy: Privacy,
    pub list_privacy: Privacy,
    pub metadata_privacy: Privacy,
    pub visibility: Privacy,
}
