use sqlx::{Pool, Postgres};

use crate::{board::{AppState}, routing};

pub(crate) async fn serve<'db>(port: String, app_state: AppState<'db>) -> () {
    let app_routes = routing::build_routes(db_pool, app_state);
    let addr = format!("0.0.0.0:{}", port);
    println!("Serving at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app_routes)
        .await
        .expect("Failure to serve API")
}
