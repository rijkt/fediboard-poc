mod board;
mod file;
mod http;
mod routing;
mod thread;
mod infra;

#[tokio::main]
async fn main() {
    let app_state = infra::create_app_state().await;
    http::serve(app_state).await
}


