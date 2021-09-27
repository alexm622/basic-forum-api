pub mod category{
    use serde::{Serialize, Deserialize};
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Category{
        pub id: u64,
        pub mod_ids: Vec<u64>,
        pub creator_id: u64,
        pub name: String,
        pub desc: String,
        pub creation_date: u64,
        pub moderation_id: u64,
        pub tags: Option<Vec<String>>,
    }
}