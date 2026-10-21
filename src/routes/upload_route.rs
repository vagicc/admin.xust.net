use crate::handlers::summernote_h;
use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/upload/demo", get(summernote_h::demo_html))
        .route("/upload/demo", post(summernote_h::upload_demo))
}
