use std::collections::HashMap;

use axum::{
    Form, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    infra::{
        AppState,
        routing::{board_routes::validate_board_name, thread_routes::parse_thread_id},
    },
    thread::{self, Post, PostError, PostUseCase, ThreadUseCase},
};

#[derive(Serialize, Deserialize)]
pub(super) struct PostsView {
    pub(super) posts: Vec<PostView>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct PostView {
    pub(super) id: String,
    pub(super) name: Option<String>,
    pub(super) subject: Option<String>,
    pub(super) content: Option<String>,
    pub(super) media_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct PostCreation {
    pub(super) name: Option<String>,
    pub(super) subject: Option<String>,
    pub(super) content: Option<String>,
    pub(super) media_url: Option<String>,
}

pub(super) fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_posts))
        .route("/", post(create_post))
        .route("/{post_id}", get(get_post))
        .with_state(app_state)
}

pub(super) fn to_post_view(post: &Post) -> PostView {
    PostView {
        id: post.id.to_string(),
        name: post.name.clone(),
        subject: post.subject.clone(),
        content: post.content.clone(),
        media_url: post.media_url.clone(),
    }
}

async fn get_posts(
    State(app_state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<Vec<PostView>>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let thread_id = parse_thread_id(&params)?;
    let thread_use_case = app_state.di.thread_use_case();
    let thread = match thread_use_case
        .get_thread_by_id(thread_id, board_name)
        .await
    {
        Ok(thread) => thread,
        Err(_) => return Err(StatusCode::NOT_FOUND), // no further info given
    };
    let post_views = thread::extract_posts(thread)
        .iter()
        .map(to_post_view)
        .collect();
    Ok(Json(post_views))
}

async fn get_post(
    State(app_state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<PostView>, StatusCode> {
    let board_name: &str = validate_board_name(&params)?;
    let post_id = validate_post_id(&params)?;
    let thread_id = parse_thread_id(&params)?;
    let thread_use_case = app_state.di.thread_use_case();
    let thread_result = thread_use_case
        .get_thread_by_id(thread_id, board_name)
        .await;
    let thread = match thread_result {
        Ok(thread) => thread,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    match thread::extract_post_by_id(post_id, thread) {
        Some(post) => Ok(Json(to_post_view(&post))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

fn validate_post_id(params: &HashMap<String, String>) -> Result<Uuid, StatusCode> {
    let post_id_param = match params.get("post_id") {
        Some(param) => param,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    match Uuid::parse_str(post_id_param) {
        Ok(parsed) => Ok(parsed),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_post(
    State(app_state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
    Form(post_creation): Form<PostCreation>,
) -> Result<Json<PostView>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let new_post = form_to_post(post_creation);
    let thread_id = parse_thread_id(&params)?;
    let thread_use_case = app_state.di.thread_use_case();
    let thread = match thread_use_case
        .get_thread_by_id(thread_id, board_name)
        .await
    {
        Ok(thread) => thread,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    let post_use_case = app_state.di.post_use_case();
    let created = post_use_case.post_into_thread(thread, new_post).await;
    match created {
        Ok(post) => Ok(Json(to_post_view(&post))),
        Err(err) => Err(to_status_code(err)),
    }
}

fn form_to_post(post_creation: PostCreation) -> Post {
    Post {
        id: Uuid::new_v4(),
        name: post_creation.name,
        subject: post_creation.subject,
        content: post_creation.content,
        media_url: post_creation.media_url,
    }
}

fn to_status_code(err: PostError) -> StatusCode {
    match err {
        PostError::DbError => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
