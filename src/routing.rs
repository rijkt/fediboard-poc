use crate::{
    board::{self},
    file::upload_file,
    thread::{create_thread, get_post, get_posts, get_thread, get_threads},
};
use axum::{
    Extension, Router,
    routing::{get, post},
};
use sqlx::{Pool, Postgres};

pub(crate) fn build_routes(db_pool: Pool<Postgres>) -> Router {
    let api_routes = Router::new()
        .route(
            "/",
            get(async || "Hello from the fediboard api".to_string()),
        )
        .nest("/boards", board::routes())
        .route("/threads", get(get_threads))
        .route("/threads", post(create_thread))
        .route("/threads/{thread_id}", get(get_thread))
        .route("/threads/{thread_id}/posts", get(get_posts))
        .route("/threads/{thread_id}/posts/{post_id}", get(get_post))
        .route("/files", post(upload_file));
    Router::new()
        .route("/", get(async || "Hello from the fediboard".to_string()))
        .nest("/api", api_routes)
        .layer(Extension(db_pool))
}
