use super::Thread;
use crate::board::Board;
use crate::board::BoardUseCase;
use uuid::Uuid;

pub struct ThreadCreation {
    pub name: Option<String>,
    pub subject: Option<String>,
    pub content: Option<String>,
    pub media_url: Option<String>,
}

pub enum ThreadError {
    IdError,
    DbError,
    NotFound,
}

pub trait ThreadPersistence {
    fn find_thread_by_id(
        &self,
        thread_id: &Uuid,
    ) -> impl Future<Output = Result<Thread, ThreadError>> + Send;

    fn find_threads_by_board(
        &self,
        board: &Board,
    ) -> impl Future<Output = Result<Vec<Thread>, ThreadError>> + Send;

    fn insert_thread(
        &self,
        board: Board,
        thread_creation: ThreadCreation,
    ) -> impl Future<Output = Result<Thread, ThreadError>> + Send;
}

pub trait ThreadUseCase {
    fn get_thread_by_id(
        &self,
        thread_id: &str,
        board_name: &str,
    ) -> impl Future<Output = Result<Thread, ThreadError>> + Send;

    fn get_threads_by_board(
        &self,
        board_name: &str,
        board_use_case: impl BoardUseCase + Send,
    ) -> impl Future<Output = Result<Vec<Thread>, ThreadError>> + Send;

    fn create_thread(
        &self,
        board: Board,
        thread_creation: ThreadCreation,
    ) -> impl Future<Output = Result<Thread, ThreadError>> + Send;
}

pub fn thread_use_case(persistence: impl ThreadPersistence + Sync) -> impl ThreadUseCase {
    ThreadUseCaseImpl { persistence }
}

struct ThreadUseCaseImpl<T>
where
    T: ThreadPersistence,
{
    persistence: T,
}

impl<T: ThreadPersistence + Sync> ThreadUseCase for ThreadUseCaseImpl<T> {
    async fn get_thread_by_id(
        &self,
        thread_id: &str,
        _board_name: &str,
    ) -> Result<Thread, ThreadError> {
        // TODO: validate with board_name param, maybe pass board
        let uuid_result = Uuid::parse_str(thread_id);
        let thread_uuid = match uuid_result {
            Ok(id) => id,
            Err(_) => return Err(ThreadError::IdError),
        };
        self.persistence.find_thread_by_id(&thread_uuid).await
    }

    fn get_threads_by_board(
        &self,
        board_name: &str,
        board_use_case: impl BoardUseCase + Send,
    ) -> impl Future<Output = Result<Vec<Thread>, ThreadError>> + Send {
        async move {
            let board: Board = match board_use_case.get_board_by_name(board_name).await {
                Ok(board) => board,
                Err(_) => return Err(ThreadError::DbError), // TODO
            };
            self.persistence.find_threads_by_board(&board).await
        }
    }

    async fn create_thread(
        &self,
        board: Board,
        thread_creation: ThreadCreation,
    ) -> Result<Thread, ThreadError> {
        self.persistence.insert_thread(board, thread_creation).await
    }
}
