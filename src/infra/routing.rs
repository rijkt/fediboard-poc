use crate::infra::AppState;
use axum::{Router, routing::get};

mod board_routes;
mod file_routes;
mod post_routes;
mod thread_routes;

pub(crate) fn build_routes(app_state: AppState) -> Router {
    let api_routes = Router::new()
        .route("/", get(hello_handler))
        .nest("/boards", board_routes::routes(app_state))
        .nest("/files", file_routes::routes());
    Router::new()
        .route("/", get(async || "Hello from the fediboard".to_string()))
        .nest("/api", api_routes)
}

async fn hello_handler() -> String {
    "Hello from the fediboard api".to_string()
}
