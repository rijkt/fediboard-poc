use uuid::Uuid;

mod board_query;
mod board_use_case;

pub use board_use_case::{BoardError, BoardPersistence, BoardUseCase, board_use_case};

pub struct Board {
    pub board_id: Uuid,
    pub name: String,
}
