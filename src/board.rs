use crate::thread;
use axum::{Extension, Json, Router, extract::Path, http::StatusCode, routing::get};
use serde::Serialize;
use sqlx::{PgPool, Postgres, postgres::PgArguments, prelude::FromRow};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(FromRow, Serialize)]
pub(crate) struct Board {
    pub(crate) board_id: Uuid,
    pub(crate) name: String,
    // pub(crate) tagline: Option<String>,
}

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", get(get_boards))
        .route("/{board_name}", get(get_board_by_name))
        .nest("/{board_name}/threads", thread::routes())
}

async fn get_board_by_name(
    Path(params): Path<HashMap<String, String>>,
    db_pool: Extension<PgPool>,
) -> Json<Board> {
    let board_name: &String = params
        .get("board_name")
        .expect("board_name is required to get board by name");
    let board = board_by_name_query(board_name)
        .fetch_one(&*db_pool)
        .await
        .expect("Failure fetching boards");
    Json(board)
}

async fn get_boards(db_pool: Extension<PgPool>) -> Json<Vec<Board>> {
    let boards = all_boards_query()
        .fetch_all(&*db_pool)
        .await
        .expect("Failure fetching boards");
    Json(boards)
}

pub(crate) type BoardQuery<'q> = sqlx::query::QueryAs<'q, Postgres, Board, PgArguments>;

fn all_boards_query() -> BoardQuery<'static> {
    sqlx::query_as::<_, Board>(
        r#"
            select board_id, name
            from board
        "#,
    )
}

pub(crate) async fn fetch_board_from_params(
    params: HashMap<String, String>,
    db_pool: &Extension<sqlx::Pool<sqlx::Postgres>>,
) -> Result<Board, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let fetch_result = board_by_name_query(board_name).fetch_one(&**db_pool).await;
    match fetch_result {
        Ok(board) => Ok(board),
        Err(_) => Err(StatusCode::NOT_FOUND), // TODO: return db-level error
    }
}

pub(crate) fn validate_board_name(params: &HashMap<String, String>) -> Result<&str, StatusCode> {
    match params.get("board_name") {
        Some(param) => Ok(param),
        None => Err(StatusCode::BAD_REQUEST),
    }
}

fn board_by_name_query(board_name: &str) -> BoardQuery<'_> {
    sqlx::query_as::<_, Board>(
        r#"
            select board_id, name
            from board
            where $1 = name
        "#,
    )
    .bind(board_name)
}
