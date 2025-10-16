use crate::board::{fetch_board_from_params, validate_board_name};
use crate::thread::query::{self as thread_query, build_by_id_query};
use crate::thread::{Post, Posts, Thread};
use axum::http::StatusCode;
use axum::{Extension, Form, Json, extract::Path};
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

#[derive(Serialize, Deserialize)]
pub(super) struct PostsView {
    pub(super) posts: Vec<PostView>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct PostView {
    pub(super) id: String,
    pub(super) name: Option<String>, // poster name
    pub(super) subject: Option<String>,
    pub(super) content: Option<String>,
    pub(super) media_url: Option<String>,
}

pub(super) async fn get_threads(
    Path(params): Path<HashMap<String, String>>,
    db_pool: Extension<PgPool>,
) -> Json<Vec<ThreadView>> {
    let board = fetch_board_from_params(params, &db_pool)
        .await
        .expect("expected board");
    let threads = thread_query::build_by_board_id_query(&board.board_id)
        .fetch_all(&*db_pool) // TODO: paginate
        .await
        .expect("Error reading threads");
    let views = threads.iter().map(to_thread_view).collect();
    Json(views)
}

pub(super) async fn get_thread(
    Path(params): Path<HashMap<String, String>>,
    db_pool: Extension<PgPool>,
) -> Result<Json<ThreadView>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let thread_id = validate_thread_id(&params)?;
    let thread = fetch_thread_by_id(thread_id, board_name, db_pool).await?;
    Ok(Json(to_thread_view(&thread)))
}

#[derive(Serialize, Deserialize)]
pub(super) struct PostCreation {
    pub(super) name: Option<String>, // poster name
    pub(super) subject: Option<String>,
    pub(super) content: Option<String>,
    pub(super) media_url: Option<String>,
}

pub(super) async fn create_thread(
    Path(params): Path<HashMap<String, String>>,
    db_pool: Extension<PgPool>,
    Form(post_creation): Form<PostCreation>,
) -> Json<ThreadView> {
    let board = fetch_board_from_params(params, &db_pool)
        .await
        .expect("expect board");
    let original_post = Post {
        id: Uuid::new_v4(),
        name: post_creation.name,
        subject: post_creation.subject,
        content: post_creation.content,
        media_url: post_creation.media_url,
    };
    let post_ser = Sqlx_json(Posts {
        posts: vec![original_post],
    });
    let created = thread_query::build_create_query(board.board_id, &post_ser)
        .fetch_one(&*db_pool)
        .await
        .expect("Error creating thread");
    Json(to_thread_view(&created))
}

pub(super) async fn get_posts(
    Path(params): Path<HashMap<String, String>>,
    db_pool: Extension<PgPool>,
) -> Result<Json<Vec<PostView>>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let thread_id = validate_thread_id(&params)?;
    let thread = fetch_thread_by_id(thread_id, board_name, db_pool).await?;
    let post_views = thread.posts.posts.iter().map(to_post_view).collect();
    Ok(Json(post_views))
}

pub(super) async fn get_post(
    Path(params): Path<HashMap<String, String>>,
    db_pool: Extension<PgPool>,
) -> Result<Json<PostView>, StatusCode> {
    let (board_name, thread_id, post_id) = validate_post_params(&params)?;
    let thread = fetch_thread_by_id(thread_id, board_name, db_pool).await?;
    let posts = &thread.posts.posts;
    let matching_post = posts.iter().find(|post| post.id == post_id);
    match matching_post {
        Some(post) => Ok(Json(to_post_view(post))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

fn validate_post_params(
    params: &HashMap<String, String>,
) -> Result<(&str, Uuid, Uuid), StatusCode> {
    let board_name: &str = validate_board_name(params)?;
    let thread_id = validate_thread_id(params)?;
    let post_id = validate_post_id(params)?;
    Ok((board_name, thread_id, post_id))
}

fn validate_thread_id(params: &HashMap<String, String>) -> Result<Uuid, StatusCode> {
    let thread_id_param = match params.get("thread_id") {
        Some(param) => param,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    match Uuid::parse_str(thread_id_param) {
        Ok(parsed) => Ok(parsed),
        Err(_) => Err(StatusCode::NOT_FOUND),
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

async fn fetch_thread_by_id(
    thread_id: Uuid,
    _board_name: &str,
    db_pool: Extension<sqlx::Pool<sqlx::Postgres>>,
) -> Result<Thread, StatusCode> {
    // TODO: validate with board_name param
    let fetch_result = build_by_id_query(&thread_id).fetch_one(&*db_pool).await;
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

fn to_post_view(post: &Post) -> PostView {
    PostView {
        id: post.id.to_string(),
        name: post.name.clone(),
        subject: post.subject.clone(),
        content: post.content.clone(),
        media_url: post.media_url.clone(),
    }
}
