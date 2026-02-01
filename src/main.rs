mod board;
mod infra;
mod thread;

#[tokio::main]
async fn main() {
    let app_state = infra::create_app_state().await;
    infra::serve(app_state).await
}
