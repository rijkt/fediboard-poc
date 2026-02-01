use serde::{Deserialize, Serialize};
use sqlx::{Error, prelude::FromRow, types::Json};
use uuid::Uuid;

use crate::{
    board::Board,
    thread::{Post, Posts, Thread, ThreadError, ThreadPersistence},
};

mod query;

pub struct ThreadPgPersistence {
    db_pool: sqlx::Pool<sqlx::Postgres>,
}

impl ThreadPgPersistence {
    pub fn new(db_pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db_pool }
    }
}

impl ThreadPersistence for ThreadPgPersistence {
    async fn find_thread_by_id(
        &self,
        thread_id: &Uuid,
    ) -> Result<crate::thread::Thread, crate::thread::ThreadError> {
        let fetch_result = query::build_by_id_query(thread_id)
            .fetch_one(&self.db_pool)
            .await;
        match fetch_result {
            Ok(thread) => Ok(to_domain(&thread)),
            Err(err) => Err(map_error(err)),
        }
    }

    async fn find_threads_by_board(
        &self,
        board: &Board,
    ) -> Result<Vec<crate::thread::Thread>, crate::thread::ThreadError> {
        let fetch_result = query::build_by_board_id_query(&board.board_id)
            .fetch_all(&self.db_pool) // TODO: paginate
            .await;
        match fetch_result {
            Ok(threads) => Ok(threads.iter().map(to_domain).collect()),
            Err(_) => Err(ThreadError::DbError),
        }
    }

    async fn insert_thread(
        &self,
        board: crate::board::Board,
        thread_creation: crate::thread::ThreadCreation,
    ) -> Result<crate::thread::Thread, crate::thread::ThreadError> {
        let initial_post = PostSchema {
            id: Uuid::new_v4(),
            name: thread_creation.name,
            subject: thread_creation.subject,
            content: thread_creation.content,
            media_url: thread_creation.media_url,
        };
        let post_ser = Json(PostsSchema {
            posts: vec![initial_post],
        });
        let create_result = query::build_create_query(board.board_id, &post_ser)
            .fetch_one(&self.db_pool)
            .await;
        match create_result {
            Ok(created) => Ok(to_domain(&created)),
            Err(_) => Err(ThreadError::DbError),
        }
    }

    async fn insert_post(
        &self,
        thread: &Thread,
        post: Post,
    ) -> Result<Thread, crate::thread::ThreadError> {
        let mut to_update = thread.posts.posts.clone();
        to_update.push(post);
        let update = PostsSchema {
            posts: to_update
                .into_iter()
                .map(|p| PostSchema {
                    id: p.id,
                    name: p.name,
                    subject: p.subject,
                    content: p.content,
                    media_url: p.media_url,
                })
                .collect(),
        };
        let update_ser = Json(update);
        let query_result = query::update_posts_query(&update_ser, &thread.thread_id);
        match query_result.fetch_one(&self.db_pool).await {
            Ok(thread_schema) => Ok(to_domain(&thread_schema)),
            Err(_) => Err(ThreadError::DbError),
        }
    }
}

#[derive(FromRow)]
struct ThreadSchema {
    thread_id: Uuid,
    board_id: Uuid,
    posts: Json<PostsSchema>,
}

#[derive(Deserialize, Serialize)]
struct PostsSchema {
    posts: Vec<PostSchema>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostSchema {
    pub id: Uuid,
    pub name: Option<String>,
    pub subject: Option<String>,
    pub content: Option<String>,
    pub media_url: Option<String>,
}

fn to_domain(thread_schema: &ThreadSchema) -> Thread {
    let posts = &thread_schema.posts.posts;
    Thread {
        thread_id: thread_schema.thread_id,
        board_id: thread_schema.board_id,
        posts: Posts {
            posts: posts
                .iter()
                .map(|p| Post {
                    id: p.id,
                    name: p.name.clone(),
                    subject: p.subject.clone(),
                    content: p.content.clone(),
                    media_url: p.media_url.clone(),
                })
                .collect(), // TODO: simplify
        },
    }
}

fn map_error(err: sqlx::Error) -> ThreadError {
    match err {
        Error::RowNotFound => ThreadError::NotFound,
        _ => ThreadError::DbError,
    }
}
