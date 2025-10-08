mod handler;

use axum::{routing::{get, post}, Router};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::Json;

use crate::thread::handler::{create_thread, get_post, get_posts, get_thread, get_threads};

#[derive(Debug, Serialize, Deserialize)]
struct PostCreation {
    name: Option<String>, // poster name
    subject: Option<String>,
    content: Option<String>,
    media_url: Option<String>,
}

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", get(get_threads))
        .route("/", post(create_thread))
        .route("/{thread_id}", get(get_thread))
        .route("/{thread_id}/posts", get(get_posts))
        .route("/{thread_id}/posts/{post_id}", get(get_post))
}

#[derive(Debug, Serialize, Deserialize)]
struct PostView {
    id: String,
    name: Option<String>, // poster name
    subject: Option<String>,
    content: Option<String>,
    media_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Post {
    id: String,
    name: Option<String>, // poster name
    subject: Option<String>,
    content: Option<String>,
    media_url: Option<String>,
}

fn mock_post() -> PostView {
    PostView { 
        id: "1".to_string(),
        name: Some("anon".to_string()),
        subject: Some("test".to_string()),
        content: Some("hello, world".to_string()),
        media_url: Some("https://example.com/".to_string()),
    }
}

#[derive(Serialize, Deserialize)]
struct Posts {
    posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize)]

struct ThreadView {
    pub(crate) thread_id: String,
    pub(crate) board_id: String,
}

#[derive(FromRow)]
struct Thread {
    pub(crate) thread_id: String,
    pub(crate) board_id: String,
    //     #[sqlx(json)]
    pub(crate) posts: Json<Posts>, // TODO: https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html#json
}

fn mock_thread() -> ThreadView {
    ThreadView {
        thread_id: "1".to_string(),
        board_id: "1".to_string(),
    }
}
