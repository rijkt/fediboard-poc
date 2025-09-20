mod board;
mod file;
mod routing;
mod thread;

#[tokio::main]
async fn main() {
    let app_routes = routing::build_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app_routes).await.unwrap();
}
