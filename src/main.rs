use sqlx::postgres::PgPoolOptions;

use crate::{board::BoardUseCaseImpl, http::AppState};

mod board;
mod file;
mod http;
mod routing;
mod thread;

#[tokio::main]
async fn main() {
    let db_url =
        dotenvy::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this service.");
    let port: String = dotenvy::var("PORT").unwrap_or("80".to_owned());

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await
        .expect("Could not connect to database");

    let app_state = AppState {
        db_pool: db_pool.clone(),
        board_state: http::BoardState {
            board_use_case: BoardUseCaseImpl { db_pool: db_pool },
        },
    };
    http::serve(port, app_state).await
}
