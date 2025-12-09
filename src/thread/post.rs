use crate::thread::Thread;
use crate::thread::query::update_posts_query;
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use sqlx::types::{Json as Sqlx_json, Uuid};

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

#[derive(Clone)]
pub struct PostUseCaseImpl {
    db_pool: PgPool,
}

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

impl PostUseCaseImpl {
    pub(crate) fn new(db_pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db_pool }
    }
}

impl PostUseCase for PostUseCaseImpl {
    async fn post_into_thread(&self, thread: Thread, new_post: Post) -> Result<Post, PostError> {
        let mut to_update = thread.posts.posts.clone();
        to_update.push(new_post);
        let update = Posts {
            posts: to_update.to_vec(),
        };
        let update_ser = Sqlx_json(update);
        let updated = match update_posts_query(&update_ser, &thread.thread_id)
            .fetch_one(&self.db_pool)
            .await
        {
            Ok(thread) => thread,
            Err(_) => return Err(PostError::DbError),
        };
        match (updated.posts).posts.last() {
            Some(post) => Ok(post.to_owned()),
            None => Err(PostError::DbError),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: Uuid,
    pub name: Option<String>, // poster name
    pub subject: Option<String>,
    pub content: Option<String>,
    pub media_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Posts {
    pub posts: Vec<Post>,
}
