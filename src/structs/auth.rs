pub mod auth{
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Auth {
        pub auth_id: u64,
        pub creation_ip: String, //the ip address the user was created from
        pub ips: Option<Vec<String>>, //all ips that this user has used
        pub last_ip: String, //the last ip the user used
        pub email: String, //this should be required and private
        pub hash: String, //salted crypt hash should be used for this
        pub uname: String, //md5 hash the user's username
        pub last_change: u64, //the date of the last change to password/username
        pub created: u64, //creation date
        pub active_tokens: Vec<Token>, //active session tokens
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Token{
        pub token: String,
        pub ip: String,
    }
}