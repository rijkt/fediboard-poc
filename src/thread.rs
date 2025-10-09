mod handler;

use axum::{
    Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use uuid::Uuid;

use crate::thread::handler::{create_thread, get_post, get_posts, get_thread, get_threads};

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", get(get_threads))
        .route("/", post(create_thread))
        .route("/{thread_id}", get(get_thread))
        .route("/{thread_id}/posts", get(get_posts))
        .route("/{thread_id}/posts/{post_id}", get(get_post))
}

#[derive(Serialize, Deserialize)]
struct Post {
    id: Uuid,
    name: Option<String>, // poster name
    subject: Option<String>,
    content: Option<String>,
    media_url: Option<String>,
}
#[derive(Serialize, Deserialize)]
struct Posts {
    posts: Vec<Post>,
}

#[derive(FromRow)]
struct Thread {
    pub(crate) thread_id: Uuid,
    pub(crate) board_id: Uuid,
    pub(crate) posts: Json<Posts>,
}
