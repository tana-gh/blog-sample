use serde::{
    Deserialize,
    Serialize,
};
use crate::db;

#[derive(Serialize, Deserialize)]
pub struct Post {
    title: String,
    body: String,
}

impl From<db::models::Post> for Post {
    fn from(post: db::models::Post) -> Self {
        Self {
            title: post.title,
            body: post.body,
        }
    }
}
