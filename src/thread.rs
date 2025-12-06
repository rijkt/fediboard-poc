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
    infra::AppState,
    thread::{
        post::Posts,
        thread_handler::{create_thread, fetch_thread_by_id, get_thread, get_threads},
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
    async fn get_thread_by_id(
        &self,
        thread_id: &str,
        board_name: &str,
    ) -> Result<Thread, ThreadError>;
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
        board_name: &str,
    ) -> Result<Thread, ThreadError> {
        let uuid_result = Uuid::parse_str(thread_id);
        let thread_uuid = match uuid_result {
            Ok(id) => id,
            Err(_) => return Err(ThreadError::IdError),
        };
        match fetch_thread_by_id(thread_uuid, board_name, &self.db_pool).await {
            Ok(thread) => Ok(thread),
            Err(_) => Err(ThreadError::DbError),
        }
    }
}
