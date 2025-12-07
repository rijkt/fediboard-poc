mod post;
mod query;
mod thread_handler;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::types::Json;
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::{
    board::BoardUseCase,
    infra::AppState,
    thread::{
        post::Posts,
        query::{build_by_board_id_query, build_by_id_query},
        thread_handler::{create_thread, get_thread, get_threads},
    },
};

pub(crate) fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_threads))
        .route("/", post(create_thread))
        .route("/{thread_id}", get(get_thread))
        .with_state(app_state.clone())
        .nest("/{thread_id}/posts", post::routes(app_state))
}

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
}
