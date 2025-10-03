mod handler;

use axum::{Router, routing::get, routing::post};
use serde::{Deserialize, Serialize};

use crate::thread::handler::{create_thread, get_post, get_posts, get_thread, get_threads};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PostCreation {
    pub(crate) name: String, // poster name
    pub(crate) subject: String,
    pub(crate) content: String,
    pub(crate) media_url: String,
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
    pub(crate) id: String,   // OID?
    pub(crate) name: String, // poster name
    pub(crate) subject: String,
    pub(crate) content: String,
    pub(crate) media_url: String,
}

fn mock_post() -> Post {
    Post {
        id: "1".to_string(),
        name: "anon".to_string(),
        subject: "test".to_string(),
        content: "hello, world".to_string(),
        media_url: "https://example.com/".to_string(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Thread {
    pub(crate) id: String, // OID?
    pub(crate) board_id: String,
    pub(crate) posts: Vec<Post>,
}

fn mock_thread() -> Thread {
    Thread {
        id: "1".to_string(),
        board_id: "1".to_string(),
        posts: vec![mock_post()],
    }
}
