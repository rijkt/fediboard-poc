use crate::thread::query::{self as thread_query, build_by_id_query};
use crate::{
    board::board_by_name_query,
    thread::{Post, Posts, Thread},
};
use axum::{Extension, Form, Json, extract::Path};
use serde::{Deserialize, Serialize};
use sqlx::{
    PgPool,
    types::{Json as Sqlx_json, Uuid},
};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub(crate) struct PostsView {
    pub(crate) posts: Vec<PostView>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct ThreadView {
    pub(crate) thread_id: String,
    pub(crate) board_id: String,
    pub(crate) posts: PostsView,
}

pub(super) async fn get_threads(
    Path(params): Path<HashMap<String, String>>,
    db_pool: Extension<PgPool>,
) -> Json<Vec<ThreadView>> {
    let board_name: &String = params
        .get("board_name")
        .expect("board_name is required to get all threads");
    let board = board_by_name_query(board_name)
        .fetch_one(&*db_pool)
        .await
        .expect("Failure fetching board {board_name}");
    let threads = thread_query::build_by_board_id_query(board.board_id)
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
    let _board_name = params
        .get("board_name")
        .expect("board_name is required to get a thread");
    let thread_id_str = params
        .get("thread_id")
        .expect("thread_id is required to fetch by id");
    let thread_id = Uuid::parse_str(&thread_id_str).expect("thread_id needs to be Uuid");
    let thread = build_by_id_query(thread_id)
        .fetch_one(&*db_pool)
        .await
        .expect("Error fetching thread ");
    Json(to_thread_view(&thread)) // TODO: validate with board_name before returning
}

#[derive(Serialize, Deserialize)]
pub(super) struct PostCreation {
    name: Option<String>, // poster name
    subject: Option<String>,
    content: Option<String>,
    media_url: Option<String>,
}

pub(super) async fn create_thread(
    Path(params): Path<HashMap<String, String>>,
    db_pool: Extension<PgPool>,
    Form(post_creation): Form<PostCreation>,
) -> Json<ThreadView> {
    let board_name: &String = params
        .get("board_name")
        .expect("board_name is required to create a threads");

    let board = board_by_name_query(board_name)
        .fetch_one(&*db_pool)
        .await
        .expect("Failure fetching board {board_name}");

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

    let created = thread_query::build_create_query(board.board_id, post_ser)
        .fetch_one(&*db_pool)
        .await
        .expect("Error creating thread");

    Json(to_thread_view(&created))
}

#[derive(Serialize, Deserialize)]
pub(super) struct PostView {
    id: String,
    name: Option<String>, // poster name
    subject: Option<String>,
    content: Option<String>,
    media_url: Option<String>,
}

pub(super) async fn get_posts(
    Path(params): Path<HashMap<String, String>>,
    db_pool: Extension<PgPool>,
) -> Json<Vec<PostView>> {
    let _board_name = params
        .get("board_name")
        .expect("board_name is required to get a thread");
    let thread_id_str = params
        .get("thread_id")
        .expect("thread_id is required to fetch by id");
    let thread_id = Uuid::parse_str(&thread_id_str).expect("thread_id needs to be Uuid");
    let thread = build_by_id_query(thread_id)
        .fetch_one(&*db_pool)
        .await
        .expect("Error fetching thread ");
    // TODO: validate with board_name
    let posts = &*thread.posts;
    let post_views = posts.posts.iter().map(to_post_view).collect();
    Json(post_views)
}

pub(super) async fn get_post(
    Path(params): Path<HashMap<String, String>>,
    db_pool: Extension<PgPool>,
) -> Json<PostView> {
    let _board_name = params
        .get("board_name")
        .expect("board_name is required to get a thread");
    let thread_id_str = params
        .get("thread_id")
        .expect("thread_id is required to fetch by id");
    let thread_id = Uuid::parse_str(&thread_id_str).expect("thread_id needs to be Uuid");
    let post_id_str = params
        .get("post_id")
        .expect("post_id is required to fetch by id");
    let post_id = Uuid::parse_str(&post_id_str).expect("post_id needs to be Uuid");

    let thread = build_by_id_query(thread_id)
        .fetch_one(&*db_pool)
        .await
        .expect("Error fetching thread ");
    // TODO: validate with board_name
    let posts = &*thread.posts;
    let post = posts
        .posts
        .iter()
        .find(|post| post.id == post_id)
        .expect("thread_id must match"); // TODO: handle with 404
    Json(to_post_view(post))
}

fn to_thread_view(thread: &Thread) -> ThreadView {
    let posts: &Posts = &*thread.posts;
    let post_arr = &posts.posts;
    ThreadView {
        thread_id: thread.thread_id.into(),
        board_id: thread.board_id.into(),
        posts: PostsView {
            posts: post_arr.iter().map(to_post_view).collect(),
        },
    }
}

fn to_post_view(p: &Post) -> PostView {
    PostView {
        id: p.id.to_string(),
        name: p.name.clone(),
        subject: p.subject.clone(),
        content: p.content.clone(),
        media_url: p.media_url.clone(),
    }
}
