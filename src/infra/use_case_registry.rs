use sqlx::PgPool;

use crate::{
    board::{self, Board, BoardPersistence, BoardUseCase},
    thread::{PostUseCase, ThreadUseCase},
};

#[derive(Clone)]
pub struct UseCaseRegistry {
    db_pool: sqlx::Pool<sqlx::Postgres>,
}

impl UseCaseRegistry {
    pub fn board_use_case(&self) -> impl BoardUseCase {
        board::board_use_case(self.db_pool.clone(), BoardPersistenceImpl{})
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

struct BoardPersistenceImpl {

}

impl BoardPersistence for BoardPersistenceImpl {
    async fn find_board_by_name(&self, board_name: &str) -> Result<Board, board::BoardError> {
        todo!()
    }
    
    async fn find_all_boards(&self) -> Result<Vec<Board>, board::BoardError> {
        todo!()
    }

}