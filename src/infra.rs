use sqlx::postgres::PgPoolOptions;

use axum::extract::FromRef;

use sqlx::PgPool;

use crate::board::BoardUseCaseImpl;

#[derive(Clone)]
pub struct AppState {
    pub port: String,
    pub db_pool: PgPool,
    pub board_state: BoardState,
}

#[derive(Clone)]
pub struct BoardState {
    pub board_use_case: BoardUseCaseImpl,
}

impl FromRef<AppState> for BoardState {
    fn from_ref(app_state: &AppState) -> BoardState {
        app_state.board_state.clone()
    }
}

pub async fn create_app_state() -> AppState {
    let db_url =
        dotenvy::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this service.");
    let port: String = dotenvy::var("PORT").unwrap_or("80".to_owned());

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await
        .expect("Could not connect to database");

    AppState {
        port,
        db_pool: db_pool.clone(),
        board_state: BoardState {
            board_use_case: BoardUseCaseImpl { db_pool: db_pool },
        },
    }
}
