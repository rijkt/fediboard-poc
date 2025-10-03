use axum::{Form, Json, extract::Path};
use std::collections::HashMap;

use crate::thread::{mock_post, mock_thread, Post, PostCreation, Thread};

pub(super) async fn get_threads(Path(params): Path<HashMap<String, String>>) -> Json<Vec<Thread>> {
    let _board_name = params
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

pub(super) async fn create_thread(Form(post_creation): Form<PostCreation>) -> Json<Thread> {
    let original_post = Post {
        id: "1".to_string(),
        name: post_creation.name,
        subject: post_creation.subject,
        content: post_creation.content,
        media_url: post_creation.media_url,
    };
    Json(Thread {
        id: "1".to_owned(),
        board_id: "1".to_owned(),
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
