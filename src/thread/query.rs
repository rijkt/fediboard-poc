use crate::thread::Posts;

use sqlx::types::{Json, Uuid};

use crate::thread::Thread;

pub(super) type CreateThreadQuery =
    sqlx::query::QueryAs<'static, sqlx::Postgres, Thread, sqlx::postgres::PgArguments>; // TODO: does this need to have a static lifetime?

pub(super) fn build_create_query(board_id: Uuid, post_ser: Json<Posts>) -> CreateThreadQuery {
    sqlx::query_as::<_, Thread>(
        r#"
        insert into thread(board_id, posts)
                values (uuid($1), $2)
                returning thread_id, board_id, posts
        "#,
    )
    .bind(board_id)
    .bind(post_ser)
}
