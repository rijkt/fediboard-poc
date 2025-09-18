use axum::{Router, debug_handler, response::Json, routing::get};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/threads", get(get_threads));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
    tagline: String
}

#[debug_handler]
async fn get_threads() -> Json<Vec<Thread>> {
    Json(vec![Thread {
        id: "1".to_string(),
        board_id: "1".to_string(),
        posts: vec![Post {
            id: "1".to_string(),
            name: "anon".to_string(),
            subject: "test".to_string(),
            content: "hello, world".to_string(),
            media_url: "https://example.com/".to_string(),
        }],
    }])
}
