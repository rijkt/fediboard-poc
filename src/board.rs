use sqlx::PgPool;
use uuid::Uuid;

use crate::board::board_query::BoardSchema;
mod board_query;

pub(crate) struct Board {
    pub(crate) board_id: Uuid,
    pub(crate) name: String,
}

pub enum BoardError {
    DbError,
}

pub trait BoardUseCase {
    fn get_board_by_name(
        &self,
        board_name: &str,
    ) -> impl Future<Output = Result<Board, BoardError>> + Send;
    fn get_all_boards(&self) -> impl Future<Output = Result<Vec<Board>, BoardError>> + Send;
}

#[derive(Clone)]
pub struct BoardUseCaseImpl {
    db_pool: PgPool,
}

impl BoardUseCaseImpl {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

impl BoardUseCase for BoardUseCaseImpl {
    async fn get_board_by_name(&self, board_name: &str) -> Result<Board, BoardError> {
        let fetch_result = board_query::board_by_name_query(board_name)
            .fetch_one(&self.db_pool)
            .await;
        match fetch_result {
            Ok(schema) => Ok(to_board(&schema)),
            Err(_) => Err(BoardError::DbError),
        }
    }

    async fn get_all_boards(&self) -> Result<Vec<Board>, BoardError> {
        let fetch_result = board_query::all_boards_query()
            .fetch_all(&self.db_pool)
            .await;
        match fetch_result {
            Ok(boards) => Ok(boards
                .iter()
                .map(|schema: &BoardSchema| to_board(schema))
                .collect()),
            Err(_) => Err(BoardError::DbError),
        }
    }
}

fn to_board(schema: &BoardSchema) -> Board {
    Board {
        board_id: schema.board_id,
        name: schema.name.to_owned(),
    }
}
