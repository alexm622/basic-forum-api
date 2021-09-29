pub mod post{
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct MakePost{
        pub cat: u64,
        pub name: String,
        pub contents: String,
        pub user: u64,
        pub user_token: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct MakeCat{
        pub name: String,
        pub desc: String,
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
    pub struct NewUser{ //log ip addresses
        pub uname: String,
        pub email: String,
        pub pw: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Login{
        pub uname: String,
        pub pw: String,
    }
}

pub mod get{
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct MFA{
        pub code: u64,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Logout{ //this should log ip address
        pub uid: u64,
        pub user_token: String, 
    }
    
}