use sqlx::postgres::PgPoolOptions;

mod board;
mod file;
mod routing;
mod http;
mod thread;

#[tokio::main]
async fn main() {
    let db_url =
        dotenvy::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this service.");

    let _ = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await
        .unwrap();

    http::serve().await
}
