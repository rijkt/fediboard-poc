use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Board {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) tagline: String,
}
