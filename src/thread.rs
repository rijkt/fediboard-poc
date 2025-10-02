mod post;
mod thread;

use crate::thread::post::{Post, mock_post};
use crate::thread::thread::{Thread, mock_thread};
use axum::Router;
use axum::response::Json;
use axum::routing::{get, post};
use axum::{Form, extract::Path};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PostCreation {
    pub(crate) name: String, // poster name
    pub(crate) subject: String,
    pub(crate) content: String,
    pub(crate) media_url: String,
}

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", get(get_threads))
        .route("/", post(create_thread))
        .route("/{thread_id}", get(get_thread))
        .route("/{thread_id}/posts", get(get_posts))
        .route("/{thread_id}/posts/{post_id}", get(get_post))
}

async fn get_threads(Path(params): Path<HashMap<String, String>>) -> Json<Vec<Thread>> {
    let _board_name = params.get("board_name").expect("board_name is required to get all threads");
    let thread = mock_thread();
    Json(vec![thread])
}

async fn get_thread(Path(params): Path<HashMap<String, String>>) -> Json<Thread> {
    let _board_name = params.get("board_name").expect("board_name is required to get all threads");
    let _thread_id = params.get("thread_id");
    Json(mock_thread())
}

async fn create_thread(Form(post_creation): Form<PostCreation>) -> Json<Thread> {
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

async fn get_posts(Path(params): Path<HashMap<String, String>>) -> Json<Vec<Post>> {
    let _thread_id = params.get("thread_id");
    let _post_id = params.get("post_id");
    Json(vec![mock_post()])
}

async fn get_post(Path(params): Path<HashMap<String, String>>) -> Json<Post> {
    let _thread_id = params.get("thread_id");
    let _post_id = params.get("post_id");
    Json(mock_post())
}
