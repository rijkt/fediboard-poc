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

use crate::{http::AppState, thread::{
    post::Posts,
    thread_handler::{create_thread, get_thread, get_threads},
}};

pub(crate) fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_threads))
        .route("/", post(create_thread))
        .route("/{thread_id}", get(get_thread))
        .with_state(app_state.clone())
        .nest("/{thread_id}/posts", post::routes(app_state))
}

#[derive(FromRow)]
struct Thread {
    pub(crate) thread_id: Uuid,
    pub(crate) board_id: Uuid,
    pub(crate) posts: Json<Posts>,
}
