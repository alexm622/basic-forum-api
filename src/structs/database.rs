pub mod database{
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct NewUser{
        pub username: String,
        pub pw: String,
        pub ip: String,
        pub email: String,
    }
}