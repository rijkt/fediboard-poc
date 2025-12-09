use crate::board::validate_board_name;
use crate::infra::AppState;
use crate::thread::Thread;
use crate::thread::ThreadUseCase;
use crate::thread::query::update_posts_query;
use crate::thread::thread_handler::parse_thread_id;
use axum::Form;
use axum::Json;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::routing::post;
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use sqlx::types::{Json as Sqlx_json, Uuid};
use std::collections::HashMap;

pub enum PostError {
    DbError,
}
pub trait PostUseCase {
    async fn post_into_thread(
        &self,
        thread: Thread,
        new_post: Post,
    ) -> Result<Post, PostError>;

    fn get_posts(thread: Thread) -> Result<Vec<Post>, PostError>;

    fn get_post(post_id: Uuid, thread: Thread) -> Result<Post, PostError>;
}

#[derive(Clone)]
pub struct PostUseCaseImpl {
    db_pool: PgPool,
}

impl PostUseCaseImpl {
    pub(crate) fn new(db_pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db_pool}
    }
}

impl PostUseCase for PostUseCaseImpl {
    async fn post_into_thread(
        &self,
        thread: Thread,
        new_post: Post,
    ) -> Result<Post, PostError> {
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
            None => return Err(PostError::DbError),
        }
    }

    fn get_posts(thread: Thread) -> Result<Vec<Post>, PostError> {
        todo!()
    }

    fn get_post(post_id: Uuid, thread: Thread) -> Result<Post, PostError> {
        todo!()
    }
}

pub(super) fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_posts))
        .route("/", post(create_post))
        .route("/{post_id}", get(get_post))
        .with_state(app_state)
}

#[derive(Serialize, Deserialize, Clone)]
pub(super) struct Post {
    pub(super) id: Uuid,
    pub(super) name: Option<String>, // poster name
    pub(super) subject: Option<String>,
    pub(super) content: Option<String>,
    pub(super) media_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct Posts {
    pub(super) posts: Vec<Post>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct PostCreation {
    pub(super) name: Option<String>, // poster name
    pub(super) subject: Option<String>,
    pub(super) content: Option<String>,
    pub(super) media_url: Option<String>,
}

async fn get_posts(
    State(app_state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<Vec<PostView>>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let thread_id = parse_thread_id(&params)?;
    let thread_use_case = app_state.di.thread_use_case();
    let thread = match thread_use_case
        .get_thread_by_id(thread_id, board_name)
        .await
    {
        Ok(thread) => thread,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    let post_views = thread.posts.posts.iter().map(to_post_view).collect(); // TODO: to use case method
    Ok(Json(post_views))
}

async fn get_post(
    State(app_state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
) -> Result<Json<PostView>, StatusCode> {
    let board_name: &str = validate_board_name(&params)?;
    let post_id = validate_post_id(&params)?;
    let thread_id = parse_thread_id(&params)?;
    let thread_use_case = app_state.di.thread_use_case();
    let thread = match thread_use_case
        .get_thread_by_id(thread_id, board_name)
        .await
    {
        Ok(thread) => thread,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    let posts = &thread.posts.posts;
    let matching_post = posts.iter().find(|post| post.id == post_id);
    match matching_post {
        Some(post) => Ok(Json(to_post_view(post))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_post(
    State(app_state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
    Form(post_creation): Form<PostCreation>,
) -> Result<Json<PostView>, StatusCode> {
    let board_name = validate_board_name(&params)?;
    let new_post = form_to_post(post_creation);
    let thread_id = parse_thread_id(&params)?;
    let thread_use_case = app_state.di.thread_use_case();
    let thread = match thread_use_case.get_thread_by_id(thread_id, board_name).await {
        Ok(thread) => thread,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };    let mut to_update = thread.posts.posts.clone();
    to_update.push(new_post);
    let update = Posts {
        posts: to_update.to_vec(),
    };
    let update_ser = Sqlx_json(update);
    let updated = match update_posts_query(&update_ser, &thread.thread_id)
        .fetch_one(&app_state.db_pool)
        .await
    {
        Ok(thread) => thread,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let last_post: &Post = match (updated.posts).posts.last() {
        Some(post) => post,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    Ok(Json(to_post_view(last_post)))
}

pub(super) fn validate_post_id(params: &HashMap<String, String>) -> Result<Uuid, StatusCode> {
    let post_id_param = match params.get("post_id") {
        Some(param) => param,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    match Uuid::parse_str(post_id_param) {
        Ok(parsed) => Ok(parsed),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub(super) fn form_to_post(post_creation: PostCreation) -> Post {
    Post {
        id: Uuid::new_v4(),
        name: post_creation.name,
        subject: post_creation.subject,
        content: post_creation.content,
        media_url: post_creation.media_url,
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct PostsView {
    pub(super) posts: Vec<PostView>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct PostView {
    pub(super) id: String,
    pub(super) name: Option<String>, // poster name
    pub(super) subject: Option<String>,
    pub(super) content: Option<String>,
    pub(super) media_url: Option<String>,
}

pub(super) fn to_post_view(post: &Post) -> PostView {
    PostView {
        id: post.id.to_string(),
        name: post.name.clone(),
        subject: post.subject.clone(),
        content: post.content.clone(),
        media_url: post.media_url.clone(),
    }
}
