use serde::{Deserialize, Serialize};
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
    pub(super) posts: Json<PostsSchema>,
}

#[derive(Deserialize, Serialize)]
pub(super) struct PostsSchema {
    pub(super) posts: Vec<PostSchema>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostSchema {
    pub id: Uuid,
    pub name: Option<String>,
    pub subject: Option<String>,
    pub content: Option<String>,
    pub media_url: Option<String>,
}

pub(super) type ThreadQuery<'q> = sqlx::query::QueryAs<'q, Postgres, ThreadSchema, PgArguments>;

pub(super) fn update_posts_query<'q>(
    posts: &'q Json<PostsSchema>,
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
