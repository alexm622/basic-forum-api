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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryInfo{
    pub cat_id:u64,
    pub cat_name:String,
    pub cat_desc:String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostInfo{
    pub post_id:u64,
    pub name:String,
    pub content:String,
    pub creator_id:u64,
    pub creation_date:u64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentInfo{
    pub comment_id:u64,
    pub content:String,
    pub creator_id:u64,
    pub creation_date:u64
}
