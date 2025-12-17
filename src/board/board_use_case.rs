use super::Board;
use crate::board::board_query::BoardSchema;
use sqlx::PgPool;

pub enum BoardError {
    NotFound,
    DbError,
}

pub trait BoardUseCase {
    fn get_board_by_name(
        &self,
        board_name: &str,
    ) -> impl Future<Output = Result<Board, BoardError>> + Send;
    fn get_all_boards(&self) -> impl Future<Output = Result<Vec<Board>, BoardError>> + Send;
}

pub fn board_use_case(db_pool: PgPool) -> impl BoardUseCase {
    BoardUseCaseImpl { db_pool }
}

struct BoardUseCaseImpl {
    db_pool: PgPool,
}

impl BoardUseCase for BoardUseCaseImpl {
    async fn get_board_by_name(&self, board_name: &str) -> Result<Board, BoardError> {
        let fetch_result = super::board_query::board_by_name_query(board_name)
            .fetch_one(&self.db_pool)
            .await;
        match fetch_result {
            Ok(schema) => Ok(to_board(&schema)),
            Err(e) => Err(map_error(e)),
        }
    }

    async fn get_all_boards(&self) -> Result<Vec<Board>, BoardError> {
        let fetch_result = super::board_query::all_boards_query()
            .fetch_all(&self.db_pool)
            .await;
        match fetch_result {
            Ok(boards) => Ok(boards
                .iter()
                .map(|schema: &BoardSchema| to_board(schema))
                .collect()),
            Err(e) => Err(map_error(e)),
        }
    }
}

fn map_error(e: sqlx::Error) -> BoardError {
    match e {
        sqlx::Error::RowNotFound => BoardError::NotFound,
        _ => BoardError::DbError,
    }
}

fn to_board(schema: &BoardSchema) -> Board {
    Board {
        board_id: schema.board_id,
        name: schema.name.to_owned(),
    }
}
