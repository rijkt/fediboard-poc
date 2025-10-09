use sqlx::postgres::PgPoolOptions;

mod board;
mod file;
mod http;
mod routing;
mod thread;

#[tokio::main]
async fn main() {
    let db_url =
        dotenvy::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this service.");
    let port: String =dotenvy::var("PORT").unwrap_or("80".to_owned());

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await
        .unwrap();

    http::serve(db_pool, port).await
}
