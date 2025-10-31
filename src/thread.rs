mod post;
mod query;
mod thread_handler;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use uuid::Uuid;

use crate::thread::{
    post::Posts,
    thread_handler::{create_thread, get_thread, get_threads},
};

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", get(get_threads))
        .route("/", post(create_thread))
        .route("/{thread_id}", get(get_thread))
        .nest("/{thread_id}/posts", post::routes())
}

#[derive(FromRow)]
struct Thread {
    pub(crate) thread_id: Uuid,
    pub(crate) board_id: Uuid,
    pub(crate) posts: Json<Posts>,
}
