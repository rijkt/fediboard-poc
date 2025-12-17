use crate::thread::Thread;
use crate::thread::query::PostSchema;
use crate::thread::query::PostsSchema;
use crate::thread::query::update_posts_query;
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

pub fn post_use_case(db_pool: PgPool) -> impl PostUseCase {
    PostUseCaseImpl {
        db_pool,
    }

}

#[derive(Clone)]
struct PostUseCaseImpl {
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

impl PostUseCase for PostUseCaseImpl {
    async fn post_into_thread(&self, thread: Thread, new_post: Post) -> Result<Post, PostError> {
        let mut to_update = thread.posts.posts.clone();
        to_update.push(new_post);
        let update = PostsSchema {
            posts: to_update
                .into_iter()
                .map(|p| PostSchema {
                    id: p.id,
                    name: p.name,
                    subject: p.subject,
                    content: p.content,
                    media_url: p.media_url,
                })
                .collect(),
        };
        let update_ser = Sqlx_json(update);
        let mut updated = match update_posts_query(&update_ser, &thread.thread_id)
            .fetch_one(&self.db_pool)
            .await
        {
            Ok(thread_schema) => super::to_domain(&thread_schema),
            Err(_) => return Err(PostError::DbError),
        };
        match updated.posts.posts.pop() {
            Some(p) => Ok(p),
            None => Err(PostError::DbError),
        }
    }
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
