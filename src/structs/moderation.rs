pub mod ban{
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Ban{
        pub mod_id: u64,
        pub is_temp: bool, //is this ban temporary
        pub is_active: bool, //is this ban active
        pub temp_time_end: u64, //this is -1 for perm bans
        pub temp_time_initial: u64, //this is -1 for perm bans
        pub cat_id: u64,//the id of the category
        pub moderator_id: u64,
        pub user_id: u64,
    }
}
pub mod mute{
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Mute{
        pub mod_id: u64,
        pub is_temp: bool, //is this a temporary mute
        pub is_active: bool, //is it currently active
        pub temp_time_end: u64, //this is -1 for perm bans
        pub temp_time_initial: u64, //this is -1 for perm bans
        pub cat_id: u64,
        pub moderator_id: u64,
        pub user_id: u64,
    }
}
pub mod global{
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct GlobalModeration{
        pub global_id: u64,
        pub is_disabled: bool, //is this account disabled
        pub is_ip_banned: bool, //are they ip banned
        pub moderator_id: u64,
        pub user_id: u64,
    }
}

pub mod moderation{
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ModerationRecord{
        pub moderation_id: u64, //the user's moderationid
        pub bans: Option<String>, //the bans this user currently has
        pub mutes: Option<String>, //the mutes this user has recieved
        pub global_info: Option<u64>, //global moderation actions taken against this user
        pub infraction_counter: u64,
        pub global_infractions:u64,
    }
}