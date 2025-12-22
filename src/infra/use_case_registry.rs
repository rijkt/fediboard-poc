use sqlx::PgPool;

use crate::{
    board::{self, BoardUseCase},
    infra::persistence::BoardPgPersistence,
    thread::{PostUseCase, ThreadUseCase},
};

#[derive(Clone)]
pub struct UseCaseRegistry {
    db_pool: sqlx::Pool<sqlx::Postgres>,
}

impl UseCaseRegistry {
    pub fn board_use_case(&self) -> impl BoardUseCase {
        board::board_use_case(BoardPgPersistence::new(self.db_pool.clone()))
    }

    pub fn thread_use_case(&self) -> impl ThreadUseCase {
        crate::thread::thread_use_case(self.db_pool.clone())
    }

    pub fn post_use_case(&self) -> impl PostUseCase {
        crate::thread::post_use_case(self.db_pool.clone())
    }
}

pub fn build_registry(db_pool: PgPool) -> UseCaseRegistry {
    UseCaseRegistry { db_pool }
}
