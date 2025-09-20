use std::collections::HashMap;

use axum::{Router, extract::Path, response::Json, routing::get};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let api_routes = Router::new()
        .route(
            "/",
            get(async || "Hello from the fediboard api".to_string()),
        )
        .route("/threads", get(get_threads))
        .route("/threads/{thread_id}", get(get_thread))
        .route("/threads/{thread_id}/posts", get(get_posts))
        .route("/threads/{thread_id}/posts/{post_id}", get(get_post));

    let app_routes = Router::new()
        .route("/", get(async || "Hello from the fediboard".to_string()))
        .nest("/api", api_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app_routes).await.unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: String,   // OID?
    name: String, // poster name
    subject: String,
    content: String,
    media_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Thread {
    id: String, // OID?
    board_id: String,
    posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Board {
    id: String,
    name: String,
    tagline: String,
}

async fn get_threads() -> Json<Vec<Thread>> {
    let thread = mock_thread();
    Json(vec![thread])
}

async fn get_thread(Path(params): Path<HashMap<String, String>>) -> Json<Thread> {
    let _thread_id = params.get("thread_id");
    Json(mock_thread())
}

async fn get_posts(Path(params): Path<HashMap<String, String>>) -> Json<Vec<Post>> {
    let _thread_id = params.get("thread_id");
    let _post_id = params.get("post_id");
    Json(vec![mock_post()])
}

async fn get_post(Path(params): Path<HashMap<String, String>>) -> Json<Post> {
    let _thread_id = params.get("thread_id");
    let _post_id = params.get("post_id");
    Json(mock_post())
}

fn mock_thread() -> Thread {
    Thread {
        id: "1".to_string(),
        board_id: "1".to_string(),
        posts: vec![mock_post()],
    }
}

fn mock_post() -> Post {
    Post {
        id: "1".to_string(),
        name: "anon".to_string(),
        subject: "test".to_string(),
        content: "hello, world".to_string(),
        media_url: "https://example.com/".to_string(),
    }
}
