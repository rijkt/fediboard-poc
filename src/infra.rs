use sqlx::postgres::PgPoolOptions;

use axum::extract::FromRef;

use sqlx::PgPool;

use crate::{
    board::{BoardUseCase, BoardUseCaseImpl},
    use_case_registry::UseCaseRegistry,
};

#[derive(Clone)]
pub struct AppState {
    pub port: String,
    pub db_pool: PgPool,
    pub di: DepenencyInjector,
}

#[derive(Clone, FromRef)]
pub struct DepenencyInjector {
    pub use_case_registry: UseCaseRegistry,
}

impl FromRef<AppState> for DepenencyInjector {
    fn from_ref(app_state: &AppState) -> DepenencyInjector {
        app_state.di.clone()
    }
}

impl DepenencyInjector {
    pub fn board_use_case(&self) -> impl BoardUseCase {
        self.use_case_registry.board_use_case()
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

    let use_case_registry = build_registry(&db_pool);

    AppState {
        port,
        db_pool: db_pool.clone(),
        di: DepenencyInjector { use_case_registry },
    }
}

fn build_registry(db_pool: &sqlx::Pool<sqlx::Postgres>) -> UseCaseRegistry {
    UseCaseRegistry::new(BoardUseCaseImpl {
        db_pool: db_pool.clone(),
    })
}
