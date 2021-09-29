pub mod database{
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct NewUser{
        pub username: String,
        pub pw: String,
        pub ip: String,
        pub email: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct IdInfo{
        pub uid: u64,
        pub aid: u64,
        pub mid: u64,
    }
}