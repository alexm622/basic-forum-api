pub mod post{
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Login{
        pub uname: String,
        pub pwd: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct MFA{
        pub code: u64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct MakePost{
        pub cat: u64,
        pub name: String,
        pub contents: String,
        pub user: u64,
        pub user_token: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct MakeComment{
        pub cat: u64,
        pub post: u64,
        pub contents: String,
        pub user: u64,
        pub user_token: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Logout{ //this should log ip address
        pub uid: u64,
        pub user_token: String, 
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct NewUser{ //log ip addresses
        pub username: String,
        pub email: String,
        pub passwd: String,
    }
}

pub mod get{
    
}