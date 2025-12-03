use crate::{
    AppState,
    board::{self},
    file::{self},
};
use axum::{Router, routing::get};

pub(crate) fn build_routes(app_state: AppState) -> Router {
    let api_routes = Router::new()
        .route("/", get(hello_handler))
        .nest("/boards", board::routes(app_state))
        .nest("/files", file::routes());
    Router::new()
        .route("/", get(async || "Hello from the fediboard".to_string()))
        .nest("/api", api_routes)
}

async fn hello_handler() -> String {
    "Hello from the fediboard api".to_string()
}
