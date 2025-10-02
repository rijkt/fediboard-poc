use axum::{Extension, Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::thread;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Board {
    pub(crate) board_id: String,
    pub(crate) name: String,
    // pub(crate) tagline: Option<String>,
}

pub(crate) fn routes() -> Router {
    Router::new()
    .route("/", get(get_boards))
    .nest("/{board_name}/threads", thread::routes())
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
