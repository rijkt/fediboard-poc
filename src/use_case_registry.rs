use crate::{
    board::{BoardUseCase, BoardUseCaseImpl},
    thread::{ThreadUseCase, ThreadUseCaseImpl},
};

#[derive(Clone)]
pub struct UseCaseRegistry {
    board_use_case: BoardUseCaseImpl,
    thread_use_case: ThreadUseCaseImpl,
}

impl UseCaseRegistry {
    pub fn board_use_case(&self) -> impl BoardUseCase {
        self.board_use_case.clone()
    }

    pub fn thread_use_case(&self) -> impl ThreadUseCase {
        self.thread_use_case.clone()
    }
}

pub fn build_registry(db_pool: &sqlx::Pool<sqlx::Postgres>) -> UseCaseRegistry {
    UseCaseRegistry {
        board_use_case: BoardUseCaseImpl {
            db_pool: db_pool.clone(),
        },
        thread_use_case: ThreadUseCaseImpl {
            db_pool: db_pool.clone(),
        },
    }
}
