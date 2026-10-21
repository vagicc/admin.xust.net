use crate::handlers::check_link_h;
use axum::extract::Form;
use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/check/link", get(check_link_h::new_html))
        .route("/check/link", post(check_link_h::check_all_url))
}
