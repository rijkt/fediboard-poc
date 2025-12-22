use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};
use uuid::Uuid;

mod query;

#[derive(FromRow)]
struct ThreadSchema {
    thread_id: Uuid,
    board_id: Uuid,
    posts: Json<PostsSchema>,
}

#[derive(Deserialize, Serialize)]
struct PostsSchema {
    posts: Vec<PostSchema>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostSchema {
    pub id: Uuid,
    pub name: Option<String>,
    pub subject: Option<String>,
    pub content: Option<String>,
    pub media_url: Option<String>,
}
