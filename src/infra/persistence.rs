use crate::board;
use crate::board::Board;

use crate::board::BoardPersistence;

pub(super) struct BoardPersistenceImpl {
}

impl BoardPersistence for BoardPersistenceImpl {
    async fn find_board_by_name(&self, board_name: &str) -> Result<Board, board::BoardError> {
        todo!()
    }

    async fn find_all_boards(&self) -> Result<Vec<Board>, board::BoardError> {
        todo!()
    }

}
