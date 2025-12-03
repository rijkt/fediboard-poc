use sqlx::postgres::PgPoolOptions;

use axum::extract::FromRef;

use sqlx::PgPool;

use crate::board::{BoardUseCase, BoardUseCaseImpl};

#[derive(Clone)]
pub struct AppState {
    pub port: String,
    pub db_pool: PgPool,
    pub di: DepenencyInjectorImpl,
}

pub trait DepenencyInjector {
    fn board_use_case(&self) -> impl BoardUseCase;
}

#[derive(Clone, FromRef)]
pub struct DepenencyInjectorImpl {
    pub db_pool: PgPool
}


impl FromRef<AppState> for DepenencyInjectorImpl {
    fn from_ref(app_state: &AppState) -> DepenencyInjectorImpl {
        app_state.di.clone()
    }
}


impl DepenencyInjector for DepenencyInjectorImpl {
    fn board_use_case(&self) -> impl BoardUseCase {
        BoardUseCaseImpl{
            db_pool: self.db_pool.clone()
        }
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
        di: DepenencyInjectorImpl { db_pool }
    }
}
