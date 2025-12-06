use crate::board::{BoardUseCase, validate_board_name};
use crate::infra::AppState;
use crate::thread::post::{PostCreation, PostsView, form_to_post, to_post_view};
use crate::thread::query::{self as thread_query, build_by_id_query};
use crate::thread::{Posts, Thread, ThreadUseCase};
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Form, Json, extract::Path};
use serde::{Deserialize, Serialize};
use sqlx::{
    PgPool,
    types::{Json as Sqlx_json, Uuid},
};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub(super) struct ThreadView {
    pub(super) thread_id: String,
    pub(super) board_id: String,
    pub(super) posts: PostsView,
}

pub(super) async fn get_threads(
    State(app_state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<Vec<ThreadView>>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let board_use_case = app_state.di.board_use_case();
    let board = match board_use_case.get_board_by_name(board_name).await {
        Ok(board) => board,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    let fetch_result = thread_query::build_by_board_id_query(&board.board_id)
        .fetch_all(&app_state.db_pool) // TODO: paginate
        .await;
    let threads = match fetch_result {
        Ok(threads) => threads,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    let views = threads.iter().map(to_thread_view).collect();
    Ok(Json(views))
}

pub(super) async fn get_thread(
    State(app_state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<ThreadView>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let thread_id = parse_thread_id(&params)?;
    let thread_use_case = app_state.di.thread_use_case();
    let thread = match thread_use_case.get_thread_by_id(thread_id, board_name).await {
        Ok(thread) => thread,
        Err(thread_error) => {
            match thread_error {
                super::ThreadError::IdError => return Err(StatusCode::BAD_REQUEST),
                super::ThreadError::DbError => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        },
    };
    Ok(Json(to_thread_view(&thread)))
}

pub(super) async fn create_thread(
    State(app_state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
    Form(post_creation): Form<PostCreation>,
) -> Result<Json<ThreadView>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let board_use_case = app_state.di.board_use_case();
    let board = match board_use_case.get_board_by_name(board_name).await {
        Ok(board) => board,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    let original_post = form_to_post(post_creation);
    let post_ser = Sqlx_json(Posts {
        posts: vec![original_post],
    });
    let create_result = thread_query::build_create_query(board.board_id, &post_ser)
        .fetch_one(&app_state.db_pool)
        .await;
    match create_result {
        Ok(created) => {
            let view = to_thread_view(&created);
            Ok(Json(view))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub(super) fn validate_thread_id(params: &HashMap<String, String>) -> Result<Uuid, StatusCode> {
    let thread_id_param = parse_thread_id(params)?;
    match Uuid::parse_str(thread_id_param) {
        Ok(parsed) => Ok(parsed),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub(super) fn parse_thread_id(params: &HashMap<String, String>) -> Result<&str, StatusCode> {
    match params.get("thread_id") {
        Some(param) => Ok(param),
        None => Err(StatusCode::BAD_REQUEST),
    }
}

pub(super) async fn fetch_thread_by_id(
    thread_id: Uuid,
    _board_name: &str,
    db_pool: &PgPool,
) -> Result<Thread, StatusCode> {
    // TODO: validate with board_name param
    let fetch_result = build_by_id_query(&thread_id).fetch_one(db_pool).await;
    match fetch_result {
        Ok(thread) => Ok(thread),
        Err(_) => Err(StatusCode::NOT_FOUND), // TODO: translate db-level error
    }
}

fn to_thread_view(thread: &Thread) -> ThreadView {
    let posts: &Posts = &thread.posts;
    let post_arr = &posts.posts;
    ThreadView {
        thread_id: thread.thread_id.into(),
        board_id: thread.board_id.into(),
        posts: PostsView {
            posts: post_arr.iter().map(to_post_view).collect(),
        },
    }
}
