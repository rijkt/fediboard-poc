mod board;
mod file;
mod infra;
mod thread;
mod use_case_registry;

#[tokio::main]
async fn main() {
    let app_state = infra::create_app_state().await;
    infra::serve(app_state).await
}
