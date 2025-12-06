use sqlx::PgPool;

use crate::{
    board::BoardUseCaseImpl, use_case_registry::UseCaseRegistry
};

mod http;
mod routing;
mod dependency_injection;
mod db;

pub use http::serve;
pub use dependency_injection::DepenencyInjector;

#[derive(Clone)]
pub struct AppState {
    pub port: String,
    pub db_pool: PgPool,
    pub di: DepenencyInjector,
}

pub async fn create_app_state() -> AppState {
    let db_url =
        dotenvy::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this service.");
    let port: String = dotenvy::var("PORT").unwrap_or("80".to_owned());

    let db_pool = db::init_db_pool(db_url).await;

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
