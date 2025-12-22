use super::Post;
use crate::thread::Posts;
use crate::thread::Thread;
use crate::thread::query::PostSchema;
use crate::thread::query::PostsSchema;
use crate::thread::query::ThreadSchema;
use sqlx::PgPool;
use sqlx::types::Json;

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
    PostUseCaseImpl { db_pool }
}

pub(crate) struct PostUseCaseImpl {
    pub(crate) db_pool: PgPool,
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
        let update_ser = Json(update);
        let query_result = super::super::query::update_posts_query(&update_ser, &thread.thread_id);
        let mut updated = match query_result.fetch_one(&self.db_pool).await {
            Ok(thread_schema) => to_domain(&thread_schema),
            Err(_) => return Err(PostError::DbError),
        };
        match updated.posts.posts.pop() {
            Some(p) => Ok(p),
            None => Err(PostError::DbError),
        }
    }
}

fn to_domain(thread_schema: &ThreadSchema) -> Thread {
    let posts = &thread_schema.posts.posts;
    Thread {
        thread_id: thread_schema.thread_id,
        board_id: thread_schema.board_id,
        posts: Posts {
            posts: posts
                .iter()
                .map(|p| Post {
                    id: p.id,
                    name: p.name.clone(),
                    subject: p.subject.clone(),
                    content: p.content.clone(),
                    media_url: p.media_url.clone(),
                })
                .collect(), // TODO: simplify
        },
    }
}