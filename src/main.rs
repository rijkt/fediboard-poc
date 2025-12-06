mod board;
mod file;
mod http;
mod routing;
mod thread;
mod infra;
mod use_case_registry;

#[tokio::main]
async fn main() {
    let app_state = infra::create_app_state().await;
    http::serve(app_state).await
}
