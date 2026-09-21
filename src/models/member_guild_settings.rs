use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemberGuildSettings {
    #[serde(default)]
    pub guild_id: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub keep_proxy: Option<bool>,
}
