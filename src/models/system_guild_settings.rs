use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemGuildSettings {
    #[serde(default)]
    pub guild_id: Option<String>,
    pub proxying_enabled: bool,
    pub tag: Option<String>,
    pub tag_enabled: bool,
    pub avatar_url: Option<String>,
    pub display_name: Option<String>,
}
