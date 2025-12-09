use sqlx::postgres::PgArguments;
use sqlx::prelude::FromRow;

use sqlx::Postgres;
use uuid::Uuid;

// TODO: private to db wrapper
#[derive(FromRow)]
pub(super) struct BoardSchema {
    pub(super) board_id: Uuid,
    pub(super) name: String,
}

pub(super) type BoardQuery<'q> = sqlx::query::QueryAs<'q, Postgres, BoardSchema, PgArguments>;

pub(super) fn all_boards_query() -> BoardQuery<'static> {
    sqlx::query_as::<_, BoardSchema>(
        r#"
            select board_id, name
            from board
        "#,
    )
}

pub(super) fn board_by_name_query(board_name: &str) -> BoardQuery<'_> {
    sqlx::query_as::<_, BoardSchema>(
        r#"
            select board_id, name
            from board
            where $1 = name
        "#,
    )
    .bind(board_name)
}
