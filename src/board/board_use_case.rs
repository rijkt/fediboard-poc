use super::Board;

pub enum BoardError {
    NotFound,
    DbError,
}

pub trait BoardPersistence {
    fn find_board_by_name(
        &self,
        board_name: &str,
    ) -> impl Future<Output = Result<Board, BoardError>> + Send;

    fn find_all_boards(&self) -> impl Future<Output = Result<Vec<Board>, BoardError>> + Send;
}

pub trait BoardUseCase {
    fn get_board_by_name(
        &self,
        board_name: &str,
    ) -> impl Future<Output = Result<Board, BoardError>> + Send;
    fn get_all_boards(&self) -> impl Future<Output = Result<Vec<Board>, BoardError>> + Send;
}

pub fn board_use_case(persistence: impl BoardPersistence + Sync) -> impl BoardUseCase {
    BoardUseCaseImpl { persistence }
}

struct BoardUseCaseImpl<T>
where
    T: BoardPersistence,
{
    persistence: T,
}

impl<T: BoardPersistence + Sync> BoardUseCase for BoardUseCaseImpl<T> {
    async fn get_board_by_name(&self, board_name: &str) -> Result<Board, BoardError> {
        self.persistence.find_board_by_name(board_name).await
    }

    async fn get_all_boards(&self) -> Result<Vec<Board>, BoardError> {
        self.persistence.find_all_boards().await
    }
}
