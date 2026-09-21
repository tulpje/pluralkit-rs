use jiff::civil::DateTime;
use serde::{Deserialize, Serialize};

use crate::models::{
    MemberRef,
    marker::{MemberMarker, SwitchMarker},
    member::Member,
    pluralkit_id::PluralKitId,
    pluralkit_uuid::PluralKitUuid,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Switch {
    pub id: PluralKitUuid<SwitchMarker>,
    #[serde(with = "crate::models::datetime")]
    pub timestamp: DateTime,
    pub members: Vec<PluralKitId<MemberMarker>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SwitchWithMembers {
    pub id: PluralKitUuid<SwitchMarker>,
    #[serde(with = "crate::models::datetime")]
    pub timestamp: DateTime,
    pub members: Vec<Member>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub(crate) struct CreateSwitch {
    members: Vec<MemberRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<DateTime>,
}

impl CreateSwitch {
    pub(crate) fn new(members: Vec<MemberRef>, timestamp: Option<DateTime>) -> Self {
        Self { members, timestamp }
    }
}
