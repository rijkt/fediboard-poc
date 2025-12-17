mod post;
mod query;

use crate::thread::query::{PostSchema, PostsSchema, ThreadSchema};
use crate::{
    board::{Board, BoardUseCase},
    thread::query::{build_by_board_id_query, build_by_id_query},
};
use sqlx::PgPool;
use sqlx::types::Json;
use uuid::Uuid;

pub use post::{Post, PostUseCase, Posts, extract_post_by_id, extract_posts, post_use_case};

pub struct Thread {
    pub(crate) thread_id: Uuid,
    pub(crate) board_id: Uuid,
    pub(crate) posts: Posts,
}

pub struct ThreadCreation {
    pub name: Option<String>,
    pub subject: Option<String>,
    pub content: Option<String>,
    pub media_url: Option<String>,
}

pub enum ThreadError {
    IdError,
    DbError,
}
pub trait ThreadUseCase {
    fn get_thread_by_id(
        &self,
        thread_id: &str,
        board_name: &str,
    ) -> impl Future<Output = Result<Thread, ThreadError>> + Send;

    fn get_threads_by_board(
        &self,
        board_name: &str,
        board_use_case: impl BoardUseCase + Send,
    ) -> impl Future<Output = Result<Vec<Thread>, ThreadError>> + Send;

    fn create_thread(
        &self,
        board: Board,
        thread_creation: ThreadCreation,
    ) -> impl Future<Output = Result<Thread, ThreadError>> + Send;
}

pub fn thread_use_case(db_pool: PgPool) -> impl ThreadUseCase {
    ThreadUseCaseImpl {
        db_pool: db_pool.clone(),
    }
}

#[derive(Clone)]
struct ThreadUseCaseImpl {
    db_pool: PgPool,
}

impl ThreadUseCase for ThreadUseCaseImpl {
    async fn get_thread_by_id(
        &self,
        thread_id: &str,
        _board_name: &str,
    ) -> Result<Thread, ThreadError> {
        // TODO: validate with board_name param
        let uuid_result = Uuid::parse_str(thread_id);
        let thread_uuid = match uuid_result {
            Ok(id) => id,
            Err(_) => return Err(ThreadError::IdError),
        };
        let fetch_result = build_by_id_query(&thread_uuid)
            .fetch_one(&self.db_pool)
            .await;
        match fetch_result {
            Ok(thread) => Ok(to_domain(&thread)),
            Err(_) => Err(ThreadError::DbError),
        }
    }

    fn get_threads_by_board(
        &self,
        board_name: &str,
        board_use_case: impl BoardUseCase + Send,
    ) -> impl Future<Output = Result<Vec<Thread>, ThreadError>> + Send {
        async move {
            let board = match board_use_case.get_board_by_name(board_name).await {
                Ok(board) => board,
                Err(_) => return Err(ThreadError::DbError), // TODO
            };
            let fetch_result = build_by_board_id_query(&board.board_id)
                .fetch_all(&self.db_pool) // TODO: paginate
                .await;
            match fetch_result {
                Ok(threads) => Ok(threads.iter().map(to_domain).collect()),
                Err(_) => Err(ThreadError::DbError),
            }
        }
    }

    async fn create_thread(
        &self,
        board: Board,
        thread_creation: ThreadCreation,
    ) -> Result<Thread, ThreadError> {
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
