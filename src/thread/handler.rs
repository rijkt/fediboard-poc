use axum::{Extension, Form, Json, extract::Path};
use sqlx::{PgPool, types::Uuid};
use std::collections::HashMap;

use crate::{
    board::Board,
    thread::{Post, PostCreation, Thread, mock_post, mock_thread},
};

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
    db_pool: Extension<PgPool>,
    Form(post_creation): Form<PostCreation>,
) -> Json<Thread> {
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
        id: Uuid::new_v4().to_string(),
        name: post_creation.name,
        subject: post_creation.subject,
        content: post_creation.content,
        media_url: post_creation.media_url,
    };

    let created = sqlx::query_as!(
        Thread,
        r#"
        insert into thread(board_id, posts)
                values ($1, $2)
                returning id, board_id, posts
        "#,
        board.board_id,
        Json(vec![original_post])
    )
    .fetch_one(&*db_pool)
    .await;

    Json(thread)
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
