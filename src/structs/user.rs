
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub avatar: Option<String>,
    pub bot: bool,
    pub mfa_enabled: bool,
    pub locale: String, //default this to utf-8
    pub flags: u64,
    pub premium_type: u64,
    pub public_flags: u64, //convert to bin to get flags, it is done in right to left
    pub auth_id: u64,
    pub moderation_id:u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flags{
    pub moderator: bool,
    pub disabled: bool,
    pub banned: bool,
    pub muted: bool,

}
