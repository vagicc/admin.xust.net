use crate::handlers::summernote_h;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/upload/demo", get(summernote_h::demo_html))
        .route("/upload/demo", post(summernote_h::upload_demo))
        .route("/upload/summernote/", post(summernote_h::upload_summernote))
        .route("/upload/summernote", post(summernote_h::upload_summernote))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024 + 64 * 1024))
}
