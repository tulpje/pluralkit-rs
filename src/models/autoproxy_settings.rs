use jiff::civil::DateTime;
use serde::{Deserialize, Serialize};

use crate::models::{marker::MemberMarker, pluralkit_id::PluralKitId};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AutoproxySettings {
    #[serde(default)]
    guild_id: Option<String>,
    #[serde(default)]
    channel_id: Option<String>,
    autoproxy_mode: AutoproxyMode,
    autoproxy_member: Option<PluralKitId<MemberMarker>>,
    last_latch_timestamp: DateTime,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AutoproxyMode {
    Off,
    Front,
    Latch,
    Member,
}
