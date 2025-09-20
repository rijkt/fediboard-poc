use serde::{Deserialize, Serialize};

use crate::thread::{mock_post, post};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Thread {
    pub(crate) id: String, // OID?
    pub(crate) board_id: String,
    pub(crate) posts: Vec<post::Post>,
}

pub(super) fn mock_thread() -> Thread {
    Thread {
        id: "1".to_string(),
        board_id: "1".to_string(),
        posts: vec![mock_post()],
    }
}
