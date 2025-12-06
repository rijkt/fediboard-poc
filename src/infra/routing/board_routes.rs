use std::collections::HashMap;

use axum::{Json, Router, extract::{Path, State}, http::StatusCode, routing::get};

use crate::{board::{Board, BoardUseCase, validate_board_name}, infra::{AppState, DepenencyInjector}, thread};

pub(crate) fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_boards))
        .route("/{board_name}", get(get_board_by_name))
        .with_state(app_state.clone())
        .nest("/{board_name}/threads", thread::routes(app_state))
}

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

async fn get_boards(
    State(di): State<DepenencyInjector>,
) -> Result<Json<Vec<Board>>, StatusCode> {
    let use_case = di.board_use_case();
    match use_case.get_all_boards().await {
        Ok(boards) => Ok(Json(boards)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
