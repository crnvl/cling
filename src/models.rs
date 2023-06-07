use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i64,
    pub board: String,
    pub thumb_url: String,
    pub content: String,
    pub username: String,
    pub ref_id: i64,
    pub time: String,
}

pub struct UserPost {
    pub thumb_url: Option<String>,
    pub content: String,
    pub username: Option<String>,
    pub ref_id: Option<i64>,
}