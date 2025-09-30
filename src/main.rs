use std::env;

use sqlx::postgres::PgPoolOptions;

mod board;
mod file;
mod routing;
mod thread;

#[tokio::main]
async fn main() {
    let db_url =
        env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this service.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await
        .unwrap();

    let _ = sqlx::migrate!("./migrations").run(&pool).await;
    let app_routes = routing::build_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app_routes).await.unwrap();
}
