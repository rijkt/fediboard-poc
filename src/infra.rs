mod db;
mod dependency_injection;
mod http;
mod routing;
mod use_case_registry;
mod persistence;

pub use dependency_injection::DepenencyInjector;
pub use http::serve;

#[derive(Clone)]
pub struct AppState {
    pub port: String,
    pub di: DepenencyInjector,
}

pub async fn create_app_state() -> AppState {
    let db_url =
        dotenvy::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this service.");
    let port: String = dotenvy::var("PORT").unwrap_or("80".to_owned());
    let db_pool = db::init_db_pool(db_url).await;
    let use_case_registry = use_case_registry::build_registry(db_pool);
    AppState {
        port,
        di: DepenencyInjector { use_case_registry },
    }
}
