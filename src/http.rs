use sqlx::{Pool, Postgres};

use crate::routing;

pub(crate) async fn serve(db_pool: Pool<Postgres>, port: String) -> () {
    let app_routes = routing::build_routes(db_pool);
    let addr = format!("0.0.0.0:{}", port);
    println!("Serving at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app_routes)
        .await
        .expect("Failure to serve API")
}
