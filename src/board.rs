use crate::{
    infra::{AppState, DepenencyInjector},
    thread,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::Serialize;
use sqlx::{PgPool, prelude::FromRow};
use std::collections::HashMap;
use uuid::Uuid;
mod board_query;

#[derive(FromRow, Serialize)]
pub(crate) struct Board {
    pub(crate) board_id: Uuid,
    pub(crate) name: String,
    // pub(crate) tagline: Option<String>,
}

pub enum BoardError {
    DbError,
}

pub trait BoardUseCase {
    fn get_board_by_name(
        &self,
        board_name: &str,
    ) -> impl Future<Output = Result<Board, BoardError>> + Send;
    fn get_all_boards(&self) -> impl Future<Output = Result<Vec<Board>, BoardError>> + Send;
}

#[derive(Clone)]
pub struct BoardUseCaseImpl {
    pub db_pool: PgPool,
}

impl BoardUseCase for BoardUseCaseImpl {
    async fn get_board_by_name(&self, board_name: &str) -> Result<Board, BoardError> {
        let fetch_result = board_query::board_by_name_query(board_name)
            .fetch_one(&self.db_pool)
            .await;
        match fetch_result {
            Ok(board) => Ok(board),
            Err(_) => Err(BoardError::DbError),
        }
    }

    async fn get_all_boards(&self) -> Result<Vec<Board>, BoardError> {
        let fetch_result = board_query::all_boards_query().fetch_all(&self.db_pool).await;
        match fetch_result {
            Ok(boards) => Ok(boards),
            Err(_) => Err(BoardError::DbError),
        }
    }
}

pub(crate) fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_boards))
        .route("/{board_name}", get(get_board_by_name))
        .with_state(app_state.clone())
        .nest("/{board_name}/threads", thread::routes(app_state))
}

#[axum::debug_handler]
async fn get_board_by_name(
    State(di): State<DepenencyInjector>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<Board>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let use_case = di.board_use_case();
    match use_case.get_board_by_name(board_name).await {
        Ok(board) => Ok(Json(board)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

#[axum::debug_handler]
async fn get_boards(
    State(di): State<DepenencyInjector>,
) -> Result<Json<Vec<Board>>, StatusCode> {
    let use_case = di.board_use_case();
    match use_case.get_all_boards().await {
        Ok(boards) => Ok(Json(boards)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub(crate) async fn fetch_board_from_params(
    params: HashMap<String, String>,
    db_pool: &PgPool,
) -> Result<Board, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let fetch_result = board_query::board_by_name_query(board_name).fetch_one(db_pool).await;
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


