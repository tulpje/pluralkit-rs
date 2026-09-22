use jiff::civil::DateTime;
use serde::{Deserialize, Serialize};

use crate::models::{member::Member, system::System};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    #[serde(with = "crate::models::datetime")]
    timestamp: DateTime,
    id: String,
    original: String,
    sender: String,
    channel: String,
    guild: String,
    system: Option<System>,
    member: Option<Member>,
}
