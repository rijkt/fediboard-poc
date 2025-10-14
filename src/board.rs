use axum::{Extension, Json, Router, routing::get};
use serde::Serialize;
use sqlx::{PgPool, Postgres, postgres::PgArguments, prelude::FromRow};
use uuid::Uuid;

use crate::thread;

#[derive(FromRow, Serialize)]
pub(crate) struct Board {
    pub(crate) board_id: Uuid,
    pub(crate) name: String,
    // pub(crate) tagline: Option<String>,
}

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", get(get_boards))
        .nest("/{board_name}/threads", thread::routes())
        // TODO: add handler for board by name
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

pub(crate) type BoardQuery<'q> = sqlx::query::QueryAs<'q, Postgres, Board, PgArguments>;

pub(crate) fn board_by_name_query(board_name: &String) -> BoardQuery<'_> {
    sqlx::query_as::<_, Board>(
        r#"
            select board_id, name
            from board
            where $1 = name
        "#,
    )
    .bind(board_name.clone())
}
