use axum::{Router, extract::Multipart, routing::post};

pub(crate) fn routes() -> Router {
    Router::new().route("/", post(upload_file))
}

async fn upload_file(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap(); // check result for errors (e.g. 2MB maximum)
        println!("Length of `{}` is {} bytes", name, data.len());
    }
    // TODO: upload to store, return url
}
