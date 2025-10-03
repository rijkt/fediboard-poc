use axum::{extract::Path, Form, Json};
use sqlx::types::Uuid;
use std::collections::HashMap;

use crate::thread::{Post, PostCreation, Thread, mock_post, mock_thread};

pub(super) async fn get_threads(Path(params): Path<HashMap<String, String>>) -> Json<Vec<Thread>> {
    let _board_name: &String = params
        .get("board_name")
        .expect("board_name is required to get all threads");
    let thread = mock_thread();
    Json(vec![thread])
}

pub(super) async fn get_thread(Path(params): Path<HashMap<String, String>>) -> Json<Thread> {
    let _board_name = params
        .get("board_name")
        .expect("board_name is required to get all threads");
    let _thread_id = params.get("thread_id");
    Json(mock_thread())
}

pub(super) async fn create_thread(
    Path(params): Path<HashMap<String, String>>,
    Form(post_creation): Form<PostCreation>
) -> Json<Thread> {
    let board_name: &String = params
        .get("board_name")
        .expect("board_name is required to create a threads");
    let original_post = Post {
        id: Uuid::new_v4().to_string(),
        name: post_creation.name,
        subject: post_creation.subject,
        content: post_creation.content,
        media_url: post_creation.media_url,
    };
    Json(Thread {
        id: Uuid::new_v4().to_string(),
        board_id: board_name.into(),
        posts: vec![original_post],
    })
}

pub(super) async fn get_posts(Path(params): Path<HashMap<String, String>>) -> Json<Vec<Post>> {
    let _thread_id = params.get("thread_id");
    let _post_id = params.get("post_id");
    Json(vec![mock_post()])
}

pub(super) async fn get_post(Path(params): Path<HashMap<String, String>>) -> Json<Post> {
    let _thread_id = params.get("thread_id");
    let _post_id = params.get("post_id");
    Json(mock_post())
}
