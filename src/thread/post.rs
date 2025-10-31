use crate::board::validate_board_name;
use crate::thread::thread_handler::fetch_thread_by_id;
use crate::thread::thread_handler::validate_thread_id;
use axum::Extension;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub(super) struct Post {
    pub(super) id: Uuid,
    pub(super) name: Option<String>, // poster name
    pub(super) subject: Option<String>,
    pub(super) content: Option<String>,
    pub(super) media_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct Posts {
    pub(super) posts: Vec<Post>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct PostCreation {
    pub(super) name: Option<String>, // poster name
    pub(super) subject: Option<String>,
    pub(super) content: Option<String>,
    pub(super) media_url: Option<String>,
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
    let board_name: &str = validate_board_name(&params)?;
    let thread_id = validate_thread_id(&params)?;
    let post_id = validate_post_id(&params)?;
    let thread = fetch_thread_by_id(thread_id, board_name, db_pool).await?;
    let posts = &thread.posts.posts;
    let matching_post = posts.iter().find(|post| post.id == post_id);
    match matching_post {
        Some(post) => Ok(Json(to_post_view(post))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub(super) fn validate_post_id(params: &HashMap<String, String>) -> Result<Uuid, StatusCode> {
    let post_id_param = match params.get("post_id") {
        Some(param) => param,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    match Uuid::parse_str(post_id_param) {
        Ok(parsed) => Ok(parsed),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
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

pub(super) fn to_post_view(post: &Post) -> PostView {
    PostView {
        id: post.id.to_string(),
        name: post.name.clone(),
        subject: post.subject.clone(),
        content: post.content.clone(),
        media_url: post.media_url.clone(),
    }
}
