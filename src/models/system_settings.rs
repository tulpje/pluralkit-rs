use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemSettings {
    timezone: String,
    pings_enabled: bool,
    latch_timeout: Option<u16>,
    member_default_private: bool,
    group_default_private: bool,
    show_private_info: bool,
    member_limit: u32,
    group_limit: u32,
    case_sensitive_proxy_tags: bool,
    proxy_error_message_enabled: bool,
    hid_display_split: bool,
    hid_display_caps: bool,
    hid_list_padding: IdPaddingFormat,
    proxy_switch: ProxySwitchAction,
    name_format: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublicSystemSettings {
    pings_enabled: bool,
    latch_timeout: Option<u16>,
    case_sensitive_proxy_tags: bool,
    proxy_error_message_enabled: bool,
    hid_display_split: bool,
    hid_display_caps: bool,
    hid_list_padding: IdPaddingFormat,
    proxy_switch: ProxySwitchAction,
    name_format: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum IdPaddingFormat {
    Off,
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ProxySwitchAction {
    Off,
    New,
    Add,
}
