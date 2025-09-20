use crate::{
    file::upload_file,
    thread::{create_thread, get_post, get_posts, get_thread, get_threads},
};
use axum::{
    Router,
    routing::{get, post},
};

pub(crate) fn build_routes() -> Router {
    let api_routes = Router::new()
        .route(
            "/",
            get(async || "Hello from the fediboard api".to_string()),
        )
        .route("/threads", get(get_threads))
        .route("/threads", post(create_thread))
        .route("/threads/{thread_id}", get(get_thread))
        .route("/threads/{thread_id}/posts", get(get_posts))
        .route("/threads/{thread_id}/posts/{post_id}", get(get_post))
        .route("/files", post(upload_file));
    Router::new()
        .route("/", get(async || "Hello from the fediboard".to_string()))
        .nest("/api", api_routes)
}
