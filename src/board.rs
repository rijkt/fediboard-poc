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
) -> Result<Json<Board>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let fetch_result = board_by_name_query(board_name).fetch_one(&*db_pool).await;
    match fetch_result {
        Ok(board) => Ok(Json(board)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_boards(db_pool: Extension<PgPool>) -> Result<Json<Vec<Board>>, StatusCode> {
    let fetch_result = all_boards_query().fetch_all(&*db_pool).await;
    match fetch_result {
        Ok(boards) => Ok(Json(boards)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
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

pub(crate) type BoardQuery<'q> = sqlx::query::QueryAs<'q, Postgres, Board, PgArguments>;

fn all_boards_query() -> BoardQuery<'static> {
    sqlx::query_as::<_, Board>(
        r#"
            select board_id, name
            from board
        "#,
    )
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
