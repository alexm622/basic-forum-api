pub mod database{
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct NewUser{
        pub username: String,
        pub hash: String,
        pub ip: String,
        pub email: String,
    }
}