pub mod post{
    use serde::{Deserialize, Serialize};


    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct LoginResponse{
        pub outcome: bool, //false if failed
        pub login_token: Option<String>,
        pub uid: Option<u64> //return token and true if login info is correct
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct StatusResponse{
        pub response_code: u64,
        pub redirect: Option<String>, //the link to whatever page they just opened
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct NewUserResponse{
        pub response_code:u64,
        pub outcome :bool,
        pub uid: Option<u64>,
        pub token: Option<String>,
    }
}
pub mod get{
    use serde::{Deserialize, Serialize};
    use crate::structs::database::database::*;


    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Exists{
        pub exists:bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Comments{
        pub has_results:bool,
        pub num_results:Option<usize>,
        pub results: Option<Vec<CommentInfo>>
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Posts{
        pub has_results:bool,
        pub num_results:Option<usize>,
        pub results: Option<Vec<PostInfo>>
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Post{
        pub has_result:bool,
        pub result: Option<PostInfo>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Categories{
        pub has_results:bool,
        pub num_results:Option<usize>,
        pub results: Option<Vec<CategoryInfo>>,
    }
}