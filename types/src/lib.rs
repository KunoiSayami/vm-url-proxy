use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Body {
    url: String,
    auth: String,
}

impl Body {
    pub fn new(auth: String, url: String) -> Self {
        Self { url, auth }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn auth(&self) -> &str {
        &self.auth
    }
}
