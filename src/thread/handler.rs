use axum::{Extension, Form, Json, extract::Path};
use sqlx::{PgPool, types::Uuid, types::Json as Sqlx_json};
use std::collections::HashMap;

use crate::{
    board::Board,
    thread::{mock_post, mock_thread, Post, PostCreation, PostView, Posts, Thread, ThreadView},
};

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
        id: Uuid::new_v4().to_string(),
        name: post_creation.name,
        subject: post_creation.subject,
        content: post_creation.content,
        media_url: post_creation.media_url,
    };

    let post_ser = Sqlx_json(Posts{posts: vec![original_post]});

    let created = sqlx::query_as::<_, Thread>(
        r#"
        insert into thread(board_id, posts)
                values ($1, $2)
                returning thread_id, board_id, posts
        "#,
    ).bind(board.board_id)
    .bind(post_ser)
    .fetch_one(&*db_pool)
    .await
    .expect("Error creating thread");

    Json(ThreadView {
        thread_id: created.thread_id,
        board_id: created.board_id,
    })
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
