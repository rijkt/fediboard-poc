use axum::extract::FromRef;
use sqlx::PgPool;

use crate::{board::BoardUseCaseImpl, routing};


#[derive(Clone)]
pub struct AppState{
     pub db_pool: PgPool,
     pub board_state: BoardState
     
}

#[derive(Clone)]
pub struct BoardState {
    pub board_use_case: BoardUseCaseImpl
}

impl FromRef<AppState> for BoardState {
    fn from_ref(app_state: &AppState) -> BoardState {
        app_state.board_state.clone()
    }
}

pub(crate) async fn serve<'db>(port: String, app_state: AppState) -> () {
    let app_routes = routing::build_routes(app_state);
    let addr = format!("0.0.0.0:{}", port);
    println!("Serving at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app_routes)
        .await
        .expect("Failure to serve API")
}
