use crate::{
    board::Board,
    file::upload_file,
    thread::{create_thread, get_post, get_posts, get_thread, get_threads},
};
use axum::{
    Extension, Router,
    response::Json,
    routing::{get, post},
};
use sqlx::{PgPool, Pool, Postgres};

pub(crate) fn build_routes(db_pool: Pool<Postgres>) -> Router {
    let api_routes = Router::new()
        .route(
            "/",
            get(async || "Hello from the fediboard api".to_string()),
        )
        .route("/boards", get(get_boards))
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

async fn get_boards(db_pool: Extension<PgPool>) -> Json<Vec<Board>> {
    let boards = sqlx::query_as!(
        Board,
        r#"
            select board_id, name
            from board
        "#
    )
    .fetch_all(&*db_pool)
    .await
    .expect("Failure fetching boards");
    Json(boards)
}
