use axum::{Extension, Form, Json, extract::Path};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, types::Json as Sqlx_json, types::Uuid};
use std::collections::HashMap;

use crate::{
    board::Board,
    thread::{Post, Posts, Thread},
};

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct ThreadView {
    pub(crate) thread_id: String,
    pub(crate) board_id: String,
}

fn mock_thread() -> ThreadView {
    ThreadView {
        thread_id: "1".to_string(),
        board_id: "1".to_string(),
    }
}

pub(super) async fn get_threads(
    Path(params): Path<HashMap<String, String>>,
) -> Json<Vec<ThreadView>> {
    let _board_name: &String = params
        .get("board_name")
        .expect("board_name is required to get all threads");
    let thread = mock_thread();
    Json(vec![thread])
}

pub(super) async fn get_thread(Path(params): Path<HashMap<String, String>>) -> Json<ThreadView> {
    let _board_name = params
        .get("board_name")
        .expect("board_name is required to get all threads");
    let _thread_id = params.get("thread_id");
    Json(mock_thread())
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

    let board = sqlx::query_as!(
        Board,
        r#"
            select board_id, name
            from board
            where $1 = name
        "#,
        board_name.clone()
    )
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

    let created = sqlx::query_as::<_, Thread>(
        r#"
        insert into thread(board_id, posts)
                values (uuid($1), $2)
                returning thread_id, board_id, posts
        "#,
    )
    .bind(board.board_id)
    .bind(post_ser)
    .fetch_one(&*db_pool)
    .await
    .expect("Error creating thread");

    Json(ThreadView {
        thread_id: created.thread_id.into(),
        board_id: created.board_id.into(),
    })
}

#[derive(Serialize, Deserialize)]
pub(super) struct PostView {
    id: String,
    name: Option<String>, // poster name
    subject: Option<String>,
    content: Option<String>,
    media_url: Option<String>,
}

fn mock_post() -> PostView {
    PostView {
        id: "1".to_string(),
        name: Some("anon".to_string()),
        subject: Some("test".to_string()),
        content: Some("hello, world".to_string()),
        media_url: Some("https://example.com/".to_string()),
    }
}

pub(super) async fn get_posts(Path(params): Path<HashMap<String, String>>) -> Json<Vec<PostView>> {
    let _thread_id = params.get("thread_id");
    let _post_id = params.get("post_id");
    Json(vec![mock_post()])
}

pub(super) async fn get_post(Path(params): Path<HashMap<String, String>>) -> Json<PostView> {
    let _thread_id = params.get("thread_id");
    let _post_id = params.get("post_id");
    Json(mock_post())
}
