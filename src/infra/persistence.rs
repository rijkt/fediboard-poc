use crate::board;
use crate::board::Board;

use crate::board::BoardPersistence;

pub(super) struct BoardPersistenceImpl {
    db_pool: sqlx::Pool<sqlx::Postgres>,
}

impl BoardPersistenceImpl {
    pub(super) fn new(db_pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db_pool }
    }
}

impl BoardPersistence for BoardPersistenceImpl {
    async fn find_board_by_name(&self, board_name: &str) -> Result<Board, board::BoardError> {
        todo!()
    }

    async fn find_all_boards(&self) -> Result<Vec<Board>, board::BoardError> {
        todo!()
    }
}
