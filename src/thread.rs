mod handler;

use axum::{Router, routing::get, routing::post};
use serde::{Deserialize, Serialize};
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
struct Post {
    id: String,
    name: Option<String>, // poster name
    subject: Option<String>,
    content: Option<String>,
    media_url: Option<String>,
}

fn mock_post() -> Post {
    Post {
        id: "1".to_string(),
        name: Some("anon".to_string()),
        subject: Some("test".to_string()),
        content: Some("hello, world".to_string()),
        media_url: Some("https://example.com/".to_string()),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Thread {
    pub(crate) thread_id: String,
    pub(crate) board_id: String,
    pub(crate) posts: Json<Vec<Post>>,
}

fn mock_thread() -> Thread {
    Thread {
        thread_id: "1".to_string(),
        board_id: "1".to_string(),
        posts: Json(vec![mock_post()]),
    }
}
