use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Post {
    pub(crate) id: String,   // OID?
    pub(crate) name: String, // poster name
    pub(crate) subject: String,
    pub(crate) content: String,
    pub(crate) media_url: String,
}

pub(super) fn mock_post() -> Post {
    Post {
        id: "1".to_string(),
        name: "anon".to_string(),
        subject: "test".to_string(),
        content: "hello, world".to_string(),
        media_url: "https://example.com/".to_string(),
    }
}
