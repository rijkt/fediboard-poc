use crate::{
    board::{self},
    file::{self},
};
use axum::{Extension, Json, Router, routing::get};
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;
use utoipauto::utoipauto;

pub(crate) fn build_routes(db_pool: Pool<Postgres>) -> Router {
    let api_routes = Router::new()
        .route("/", get(hello_handler))
        .nest("/boards", board::routes())
        .nest("/files", file::routes())
        .route("/openapi.json", get(openapi));

    Router::new()
        .route("/", get(async || "Hello from the fediboard".to_string()))
        .nest("/api", api_routes)
        .layer(Extension(db_pool))
}

async fn hello_handler() -> String {
    "Hello from the fediboard api".to_string()
}

#[utoipauto]
#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/api/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
