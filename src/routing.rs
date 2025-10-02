use crate::{
    board::{self},
    file::{self},
};
use axum::{
    Extension, Router,
    routing::get,
};
use sqlx::{Pool, Postgres};

pub(crate) fn build_routes(db_pool: Pool<Postgres>) -> Router {
    let api_routes = Router::new()
        .route("/", get(hello_handler))
        .nest("/boards", board::routes())
        .nest("/files", file::routes());
    Router::new()
        .route("/", get(async || "Hello from the fediboard".to_string()))
        .nest("/api", api_routes)
        .layer(Extension(db_pool))
}

async fn hello_handler() -> String {
    "Hello from the fediboard api".to_string()
}
