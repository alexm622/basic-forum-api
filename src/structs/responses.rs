pub mod post{
    use serde::{Deserialize, Serialize};


    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct LoginResponse{
        pub outcome: bool, //false if failed
        pub login_token: Option<String>, //return token and true if login info is correct
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct StatusResponse{
        pub response_code: u64,
        pub redirect: Option<String>, //the link to whatever page they just opened
    }
}