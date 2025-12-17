mod post;
mod query;
mod thread_use_case;

use uuid::Uuid;

pub use post::{Post, PostUseCase, Posts, extract_post_by_id, extract_posts, post_use_case};
pub use thread_use_case::{ThreadCreation, ThreadError, ThreadUseCase, thread_use_case};

pub struct Thread {
    pub(crate) thread_id: Uuid,
    pub(crate) board_id: Uuid,
    pub(crate) posts: Posts,
}
