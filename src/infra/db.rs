use sqlx::postgres::PgPoolOptions;

pub(crate) async fn init_db_pool(db_url: String) -> sqlx::Pool<sqlx::Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await
        .expect("Could not connect to database")
}
