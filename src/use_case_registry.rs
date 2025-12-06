use crate::board::{BoardUseCase, BoardUseCaseImpl};

#[derive(Clone)]
pub struct UseCaseRegistry {
    board_use_case: BoardUseCaseImpl,
}

impl UseCaseRegistry {

    pub fn new(board_use_case: BoardUseCaseImpl) -> Self {
        Self { board_use_case }
    }
    
    pub fn board_use_case(&self) -> impl BoardUseCase {
        self.board_use_case.clone()
    }
}

pub fn build_registry(db_pool: &sqlx::Pool<sqlx::Postgres>) -> UseCaseRegistry {
    UseCaseRegistry::new(BoardUseCaseImpl {
        db_pool: db_pool.clone(),
    })
}
