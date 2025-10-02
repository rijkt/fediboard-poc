use sqlx::postgres::PgPoolOptions;

use crate::board::Board;

mod board;
mod file;
mod routing;
mod thread;

#[tokio::main]
async fn main() {
    let db_url =
        dotenvy::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this service.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await
        .unwrap();

    let boards = sqlx::query_as!(
        Board,
        r#"
            select board_id, name
            from board
        "#
    )
    .fetch_all(&pool)
    .await;
    println!("{:?}", boards);

    let app_routes = routing::build_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app_routes).await.unwrap();
}
