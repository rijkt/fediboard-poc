use crate::board::BoardUseCase;
use crate::infra::AppState;
use crate::infra::routing::board_routes::validate_board_name;
use crate::infra::routing::post_routes::{self, PostCreation, PostsView, to_post_view};
use crate::thread::{Posts, Thread, ThreadCreation, ThreadError, ThreadUseCase};
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Form, Json, extract::Path};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, ToSchema)]
pub(super) struct ThreadView {
    pub(super) thread_id: String,
    pub(super) board_id: String,
    pub(super) posts: PostsView,
}

pub(super) fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_threads))
        .route("/", post(create_thread))
        .route("/{thread_id}", get(get_thread))
        .with_state(app_state.clone())
        .nest("/{thread_id}/posts", post_routes::routes(app_state))
}

pub(super) fn parse_thread_id(params: &HashMap<String, String>) -> Result<&str, StatusCode> {
    match params.get("thread_id") {
        Some(param) => Ok(param),
        None => Err(StatusCode::BAD_REQUEST),
    }
}

#[utoipa::path(
    context_path = "/api/boards/{board_name}/threads/",
    get,
    path = "",
    responses(
        (status = 200, body = Vec<ThreadView>, content_type = "application/json")
    )
)]
async fn get_threads(
    State(app_state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<Vec<ThreadView>>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let board_use_case = app_state.di.board_use_case();
    let thread_use_case = app_state.di.thread_use_case();
    let threads_result = thread_use_case
        .get_threads_by_board(board_name, board_use_case)
        .await;
    let threads = match threads_result {
        Ok(threads) => threads,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    let views = threads.iter().map(to_thread_view).collect();
    Ok(Json(views))
}

async fn get_thread(
    State(app_state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<ThreadView>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let thread_id = parse_thread_id(&params)?;
    let thread_use_case = app_state.di.thread_use_case();
    let thread = match thread_use_case
        .get_thread_by_id(thread_id, board_name)
        .await
    {
        Ok(thread) => thread,
        Err(thread_error) => return Err(to_status_code(thread_error)),
    };
    Ok(Json(to_thread_view(&thread)))
}

async fn create_thread(
    State(app_state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
    Form(post_creation): Form<PostCreation>,
) -> Result<Json<ThreadView>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let board_use_case = app_state.di.board_use_case();
    let thread_use_case = app_state.di.thread_use_case();
    let board = match board_use_case.get_board_by_name(board_name).await {
        Ok(board) => board,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    let thread_creation = to_thread_creation(post_creation);
    let create_result = thread_use_case.create_thread(board, thread_creation).await;
    match create_result {
        Ok(created) => {
            let view = to_thread_view(&created);
            Ok(Json(view))
        }
        Err(err) => Err(to_status_code(err)),
    }
}

fn to_thread_creation(post_creation: PostCreation) -> crate::thread::ThreadCreation {
    ThreadCreation {
        name: post_creation.name,
        subject: post_creation.subject,
        content: post_creation.content,
        media_url: post_creation.media_url,
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

fn to_status_code(err: ThreadError) -> StatusCode {
    match err {
        ThreadError::NotFound => StatusCode::NOT_FOUND,
        ThreadError::DbError => StatusCode::INTERNAL_SERVER_ERROR,
        ThreadError::IdError => StatusCode::BAD_REQUEST,
    }
}
