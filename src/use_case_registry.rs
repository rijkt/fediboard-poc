use crate::{
    board::{BoardUseCase, BoardUseCaseImpl},
    thread::{PostUseCase, PostUseCaseImpl, ThreadUseCase, ThreadUseCaseImpl},
};

#[derive(Clone)]
pub struct UseCaseRegistry {
    board_use_case: BoardUseCaseImpl,
    thread_use_case: ThreadUseCaseImpl,
    post_use_case: PostUseCaseImpl
}

impl UseCaseRegistry {
    pub fn board_use_case(&self) -> impl BoardUseCase {
        self.board_use_case.clone()
    }

    pub fn thread_use_case(&self) -> impl ThreadUseCase {
        self.thread_use_case.clone()
    }

    pub fn post_use_case(&self) -> impl PostUseCase {
        self.post_use_case.clone()
    }
}

pub fn build_registry(db_pool: &sqlx::Pool<sqlx::Postgres>) -> UseCaseRegistry {
    UseCaseRegistry {
        board_use_case: BoardUseCaseImpl::new(db_pool.clone()),
        thread_use_case: ThreadUseCaseImpl::new(db_pool.clone()),
        post_use_case: PostUseCaseImpl::new(db_pool.clone()),
    }
}
