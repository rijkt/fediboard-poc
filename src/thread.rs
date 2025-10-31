mod thread_handler;
mod post;
mod query;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use uuid::Uuid;

use crate::thread::{
    thread_handler::{create_thread, get_thread, get_threads},
    post::{Posts, get_post, get_posts},
};

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", get(get_threads))
        .route("/", post(create_thread))
        .route("/{thread_id}", get(get_thread))
        .route("/{thread_id}/posts", get(get_posts))
        .route("/{thread_id}/posts/{post_id}", get(get_post))
}

#[derive(FromRow)]
struct Thread {
    pub(crate) thread_id: Uuid,
    pub(crate) board_id: Uuid,
    pub(crate) posts: Json<Posts>,
}
