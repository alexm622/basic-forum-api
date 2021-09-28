pub mod auth{
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Auth {
        pub auth_id: u64,
        pub creation_ip: String, //the ip address the user was created from
        pub last_ip: String, //the last ip the user used
        pub email: String, //this should be required and private
        pub hash: String, //salted crypt hash should be used for this
        pub uname: String, //md5 hash the user's username
        pub last_change: u64, //the date of the last change to password/username
        pub created: u64, //creation date
        pub active_token: Option<String>, //active session tokens
        pub token_ip: Option<String>,
        pub token_date: Option<u64>,
    }
}