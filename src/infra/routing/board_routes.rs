use std::collections::HashMap;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    board::{Board, BoardError, BoardUseCase},
    infra::{AppState, DepenencyInjector, routing::thread_routes},
};

#[derive(Serialize, Deserialize, ToSchema)]
pub(super) struct BoardView {
    pub(crate) board_id: String,
    pub(crate) name: String,
}

pub(super) fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_boards))
        .route("/{board_name}", get(get_board_by_name))
        .with_state(app_state.clone())
        .nest("/{board_name}/threads", thread_routes::routes(app_state))
}

pub(super) fn validate_board_name(params: &HashMap<String, String>) -> Result<&str, StatusCode> {
    match params.get("board_name") {
        Some(param) => Ok(param),
        None => Err(StatusCode::BAD_REQUEST),
    }
}

#[utoipa::path(
    context_path = "/api/boards/",
    get,
    path = "/{board_name}",
    responses(
        (status = 200, body = BoardView, content_type = "application/json"),
        (status = 404)
    )
)]
async fn get_board_by_name(
    State(di): State<DepenencyInjector>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<BoardView>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let use_case = di.board_use_case();
    match use_case.get_board_by_name(board_name).await {
        Ok(board) => Ok(Json(to_view(board))),
        Err(err) => Err(to_status_code(err)),
    }
}

#[utoipa::path(
    context_path = "/api/boards",
    get,
    path = "",
    responses(
        (status = 200, body = Vec<BoardView>, content_type = "application/json")
    )
)]
async fn get_boards(
    State(di): State<DepenencyInjector>,
) -> Result<Json<Vec<BoardView>>, StatusCode> {
    let use_case = di.board_use_case();
    match use_case.get_all_boards().await {
        Ok(boards) => Ok(Json(boards.into_iter().map(to_view).collect())),
        Err(err) => Err(to_status_code(err)),
    }
}

fn to_view(board: Board) -> BoardView {
    BoardView {
        board_id: board.board_id.to_string(),
        name: board.name,
    }
}

fn to_status_code(err: BoardError) -> StatusCode {
    match err {
        BoardError::NotFound => StatusCode::NOT_FOUND,
        BoardError::DbError => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
