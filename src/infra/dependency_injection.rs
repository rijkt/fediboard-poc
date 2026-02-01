use axum::extract::FromRef;

use crate::{
    board::BoardUseCase,
    infra::{AppState, use_case_registry::UseCaseRegistry},
    thread::{PostUseCase, ThreadUseCase},
};

#[derive(Clone, FromRef)]
pub struct DepenencyInjector {
    pub use_case_registry: UseCaseRegistry,
}

impl FromRef<AppState> for DepenencyInjector {
    fn from_ref(app_state: &AppState) -> DepenencyInjector {
        app_state.di.clone()
    }
}

impl DepenencyInjector {
    pub fn board_use_case(&self) -> impl BoardUseCase {
        self.use_case_registry.board_use_case()
    }

    pub fn thread_use_case(&self) -> impl ThreadUseCase {
        self.use_case_registry.thread_use_case()
    }

    pub fn post_use_case(&self) -> impl PostUseCase {
        self.use_case_registry.post_use_case()
    }
}
