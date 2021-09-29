pub mod posts{
    use serde::{Serialize, Deserialize};
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Post{
        pub id: u64,
        pub cat_id: u64,
        pub creator_id: u64,
        pub is_archived: bool,
        pub is_deleted: bool,
        pub name: String,
        pub content: String,
        pub creation_date: u64,
    }
}