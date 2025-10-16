use crate::board::fetch_board_from_params;
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
    let board = fetch_board_from_params(params, &db_pool).await;
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
) -> Json<ThreadView> {
    let thread = fetch_thread_from_params(params, db_pool).await;
    Json(to_thread_view(&thread))
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
    let board = fetch_board_from_params(params, &db_pool).await;
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
) -> Json<Vec<PostView>> {
    let thread = fetch_thread_from_params(params, db_pool).await;
    let posts = &*thread.posts;
    let post_views = posts.posts.iter().map(to_post_view).collect();
    Json(post_views)
}

pub(super) async fn get_post(
    Path(params): Path<HashMap<String, String>>,
    db_pool: Extension<PgPool>,
) -> Result<Json<PostView>, StatusCode> {
    let post_id_param = params.get("post_id");
    match post_id_param {
        Some(post_id_str) => {
            let post_id = Uuid::parse_str(post_id_str).expect("post_id is required to fetch by id");
            let thread = fetch_thread_from_params(params, db_pool).await;
            let posts = &*thread.posts;
            let post = posts
                .posts
                .iter()
                .find(|post| post.id == post_id)
                .expect("thread_id must match"); // TODO: handle with 404
            Ok(Json(to_post_view(post)))
        }
        None => Err(StatusCode::BAD_REQUEST),
    }
}

async fn fetch_thread_from_params(
    params: HashMap<String, String>,
    db_pool: Extension<sqlx::Pool<sqlx::Postgres>>,
) -> Thread {
    let _board_name = params
        .get("board_name")
        .expect("board_name is required to get a thread");
    let thread_id_str = params
        .get("thread_id")
        .expect("thread_id is required to fetch by id");
    let thread_id = Uuid::parse_str(thread_id_str).expect("thread_id needs to be Uuid");
    // TODO: validate with board_name param
    build_by_id_query(&thread_id)
        .fetch_one(&*db_pool)
        .await
        .expect("Error fetching thread ")
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
