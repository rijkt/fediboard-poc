mod handler;
mod post;
mod thread;

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
