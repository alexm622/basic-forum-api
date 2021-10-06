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
        pub parent: u64,
        pub post: u64,
        pub cat_id: u64,
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
    pub struct Logout{ //this should log ip address/check against ip address, so I cannot force a user to logout from a different ip address
        pub uid: u64,
        pub user_token: String, 
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct UnameExistsCheck{
        pub uname:String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct EmailExistsCheck{
        pub email:String,
    }

    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct GetCategories{ //force user to use token eventually?
        pub offset:u64,
    }
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct GetPosts{ //force user to use token eventually?
        pub offset:u64,
        pub cat_id:u64,
    }
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct GetComments{ //force user to use token eventually?
        pub offset:u64,
        pub parent_id:Option<u64>,
        pub post_id:u64,
        pub cat_id:u64,
    }
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct GetCommentsNoParent{ //force user to use token eventually?
        pub offset:u64,
        pub post_id:u64,
        pub cat_id:u64,
    }
       
}