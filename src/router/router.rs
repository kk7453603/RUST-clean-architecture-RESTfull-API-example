use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};


pub struct Handler {
    Url: String,
    pub app: Router,
}

impl Handler {
    pub fn new() -> Self {
        let app = Router::new().route("/", get(|| async { "ok" }));
        Self { app }
    }
}