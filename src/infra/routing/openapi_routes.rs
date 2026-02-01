use axum::{Json, Router, routing::get};
use utoipa::OpenApi;
use utoipauto::utoipauto;

pub(super) fn routes() -> Router {
    Router::new().route("/", get(openapi))
}

#[utoipauto]
#[derive(OpenApi)]
#[openapi]
pub(super) struct ApiDoc;

/// Return JSON version of an OpenAPI schema
#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = (), content_type = "application/json")
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
