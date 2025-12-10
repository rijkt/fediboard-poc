use crate::thread::Posts;
use sqlx::prelude::FromRow;
use sqlx::{
    Postgres,
    postgres::PgArguments,
    types::{Json, Uuid},
};

#[derive(FromRow)]
pub(super) struct ThreadSchema {
    pub(super) thread_id: Uuid,
    pub(super) board_id: Uuid,
    pub(super) posts: Json<Posts>,
}

pub(super) type ThreadQuery<'q> = sqlx::query::QueryAs<'q, Postgres, ThreadSchema, PgArguments>;

pub(super) fn build_create_query<'q>(board_id: Uuid, post_ser: &'q Json<Posts>) -> ThreadQuery<'q> {
    sqlx::query_as::<_, ThreadSchema>(
        r#"
        insert into thread(board_id, posts)
                values (uuid($1), $2)
                returning thread_id, board_id, posts
        "#,
    )
    .bind(board_id)
    .bind(post_ser)
}

pub(super) fn build_by_board_id_query(board_id: &Uuid) -> ThreadQuery<'_> {
    sqlx::query_as::<_, ThreadSchema>(
        r#"
        select * from thread
        where board_id = $1
        "#,
    )
    .bind(board_id)
}

pub(super) fn build_by_id_query(thread_id: &Uuid) -> ThreadQuery<'_> {
    sqlx::query_as::<_, ThreadSchema>(
        r#"
        select * from thread
        where thread_id = $1
        "#,
    )
    .bind(thread_id)
}

pub(super) fn update_posts_query<'q>(
    posts: &'q Json<Posts>,
    thread_id: &'q Uuid,
) -> ThreadQuery<'q> {
    sqlx::query_as::<_, ThreadSchema>(
        r#"
        update thread
        set posts = $1
        where thread_id = $2
        returning *
        "#,
    )
    .bind(posts)
    .bind(thread_id)
}
