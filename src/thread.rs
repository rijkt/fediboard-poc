mod post;
mod thread_use_case;

use uuid::Uuid;

pub use post::{
    Post, PostError, PostUseCase, Posts, extract_post_by_id, extract_posts, post_use_case,
};
pub use thread_use_case::{
    ThreadCreation, ThreadError, ThreadPersistence, ThreadUseCase, thread_use_case,
};

pub struct Thread {
    pub thread_id: Uuid,
    pub board_id: Uuid,
    pub posts: Posts,
}
