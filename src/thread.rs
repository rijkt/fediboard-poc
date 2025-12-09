mod post;
mod query;
use sqlx::types::Json;
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::{
    board::{Board, BoardUseCase},
    thread::query::{build_by_board_id_query, build_by_id_query},
};

pub use post::{Post, PostUseCase, PostUseCaseImpl, Posts, extract_post_by_id, extract_posts}; // TODO: only export trait

#[derive(FromRow)]
pub struct Thread {
    pub(crate) thread_id: Uuid,
    pub(crate) board_id: Uuid,
    pub(crate) posts: Json<Posts>,
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
        original_post: Post,
    ) -> impl Future<Output = Result<Thread, ThreadError>> + Send;
}

#[derive(Clone)]
pub struct ThreadUseCaseImpl {
    db_pool: PgPool,
}

impl ThreadUseCaseImpl {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
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
            Ok(thread) => Ok(thread),
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
                Ok(threads) => Ok(threads),
                Err(_) => Err(ThreadError::DbError),
            }
        }
    }

    async fn create_thread(
        &self,
        board: Board,
        original_post: Post,
    ) -> Result<Thread, ThreadError> {
        let post_ser = Json(Posts {
            posts: vec![original_post],
        });
        let create_result = query::build_create_query(board.board_id, &post_ser)
            .fetch_one(&self.db_pool)
            .await;
        match create_result {
            Ok(created) => Ok(created),
            Err(_) => Err(ThreadError::DbError),
        }
    }
}
