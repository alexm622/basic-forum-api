use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category{
    pub id: u64,
    pub mod_ids: String, //this is gonna be a comma separated list of uids
    pub creator_id: u64,
    pub name: String,
    pub desc: String,
    pub creation_date: u64,
    pub tags: String, //this is a comma separated list of tags
}
