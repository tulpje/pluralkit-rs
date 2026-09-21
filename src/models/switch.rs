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
    pub timestamp: DateTime,
    pub members: SwitchMembers,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum SwitchMembers {
    Ids(Vec<PluralKitId<MemberMarker>>),
    Structs(Vec<Member>),
}

#[cfg(test)]
mod test {
    use std::assert_matches;

    use super::*;

    #[test]
    fn test_deserialize_member_ids() {
        let members = serde_json::from_str::<SwitchMembers>("[\"aaa-aa\"]").unwrap();
        assert_matches!(members, SwitchMembers::Ids(_))
    }

    #[test]
    fn test_deserialize_member_structs() {
        let members = serde_json::from_str::<SwitchMembers>(
            r#"
                [{
                    "id": "aaa-aa",
                    "uuid": "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa",
                    "name": "a",
                    "display_name": null,
                    "color": null,
                    "birthday": null,
                    "pronouns": null,
                    "avatar_url": null,
                    "webhook_avatar_url": null,
                    "banner": null,
                    "description": null,
                    "created": null,
                    "proxy_tags": [],
                    "keep_proxy": false,
                    "tts": false,
                    "autoproxy_enabled": null,
                    "message_count": null,
                    "last_message_timestamp": null,
                    "privacy": null
                }]
            "#,
        )
        .unwrap();
        assert_matches!(members, SwitchMembers::Structs(_))
    }
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
