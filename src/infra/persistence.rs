use sqlx::Postgres;
use sqlx::postgres::PgArguments;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::board;
use crate::board::Board;

use crate::board::BoardError;
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
        let fetch_result = board_by_name_query(board_name)
            .fetch_one(&self.db_pool)
            .await;
        match fetch_result {
            Ok(schema) => Ok(to_board(&schema)),
            Err(e) => Err(map_error(e)),
        }
    }

    async fn find_all_boards(&self) -> Result<Vec<Board>, board::BoardError> {
        let fetch_result = all_boards_query().fetch_all(&self.db_pool).await;
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
    pub(super) board_id: Uuid,
    pub(super) name: String,
}

type BoardQuery<'q> = sqlx::query::QueryAs<'q, Postgres, BoardSchema, PgArguments>;

fn all_boards_query() -> BoardQuery<'static> {
    sqlx::query_as::<_, BoardSchema>(
        r#"
            select board_id, name
            from board
        "#,
    )
}

fn board_by_name_query(board_name: &str) -> BoardQuery<'_> {
    sqlx::query_as::<_, BoardSchema>(
        r#"
            select board_id, name
            from board
            where $1 = name
        "#,
    )
    .bind(board_name)
}
