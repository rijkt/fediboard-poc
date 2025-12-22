mod query;

use crate::board;
use crate::board::Board;
use crate::board::BoardError;
use crate::board::BoardPersistence;
use sqlx::prelude::FromRow;
use uuid::Uuid;

pub struct BoardPgPersistence {
    db_pool: sqlx::Pool<sqlx::Postgres>,
}

impl BoardPgPersistence {
    pub fn new(db_pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db_pool }
    }
}

impl BoardPersistence for BoardPgPersistence {
    async fn find_board_by_name(&self, board_name: &str) -> Result<Board, board::BoardError> {
        let fetch_result = query::board_by_name_query(board_name)
            .fetch_one(&self.db_pool)
            .await;
        match fetch_result {
            Ok(schema) => Ok(to_board(&schema)),
            Err(e) => Err(map_error(e)),
        }
    }

    async fn find_all_boards(&self) -> Result<Vec<Board>, board::BoardError> {
        let fetch_result = query::all_boards_query().fetch_all(&self.db_pool).await;
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

#[derive(FromRow)]
struct BoardSchema {
    board_id: Uuid,
    name: String,
}
