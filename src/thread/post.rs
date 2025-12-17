mod post_use_case;

use crate::thread::Thread;
use sqlx::types::Uuid;

pub use post_use_case::{PostError, PostUseCase, post_use_case};

pub fn extract_posts(thread: Thread) -> Vec<Post> {
    thread.posts.posts.clone()
}

pub fn extract_post_by_id(post_id: Uuid, thread: Thread) -> Option<Post> {
    let posts = thread.posts.posts.clone();
    posts
        .iter()
        .find(|post| post.id == post_id)
        .map(|p| p.to_owned())
}

#[derive(Clone)]
pub struct Post {
    pub id: Uuid,
    pub name: Option<String>, // poster name
    pub subject: Option<String>,
    pub content: Option<String>,
    pub media_url: Option<String>,
}

pub struct Posts {
    pub posts: Vec<Post>,
}
