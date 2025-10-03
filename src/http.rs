use sqlx::{Pool, Postgres};

use crate::routing;

pub(crate) async fn serve(db_pool: Pool<Postgres>) -> () {
    let app_routes = routing::build_routes(db_pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app_routes)
        .await
        .expect("Failure to serve API")
}
