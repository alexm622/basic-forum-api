use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment{
    pub id: u64,
    pub post_id: u64,
    pub parent_id: u64,
    pub creator_id: u64,
    pub is_archived: bool,
    pub is_deleted: bool,
    pub content: String,
    pub creation_date: u64,
}
