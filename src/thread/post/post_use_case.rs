use super::Post;
use crate::thread::Thread;
use crate::thread::ThreadPersistence;

pub enum PostError {
    DbError,
}

pub trait PostUseCase {
    fn post_into_thread(
        &self,
        thread: Thread,
        new_post: Post,
    ) -> impl Future<Output = Result<Post, PostError>> + Send;
}

pub fn post_use_case(thread_persistence: impl ThreadPersistence + Sync) -> impl PostUseCase {
    PostUseCaseImpl { thread_persistence }
}

pub(crate) struct PostUseCaseImpl<T>
where
    T: ThreadPersistence,
{
    pub(crate) thread_persistence: T,
}

impl<T: ThreadPersistence + Sync> PostUseCase for PostUseCaseImpl<T> {
    async fn post_into_thread(&self, thread: Thread, new_post: Post) -> Result<Post, PostError> {
        let updated = self.thread_persistence.insert_post(&thread, new_post).await;
        let mut updated: Thread = match updated {
            Ok(updated_thread) => updated_thread,
            Err(_) => return Err(PostError::DbError),
        };
        match updated.posts.posts.pop() {
            Some(p) => Ok(p),
            None => Err(PostError::DbError),
        }
    }
}
