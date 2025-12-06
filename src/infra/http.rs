use crate::infra::{AppState, routing};

pub async fn serve(app_state: AppState) -> () {
    let port = app_state.port.clone();
    let app_routes = routing::build_routes(app_state);
    let addr = format!("0.0.0.0:{}", port);
    println!("Serving at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app_routes)
        .await
        .expect("Failure to serve API")
}
