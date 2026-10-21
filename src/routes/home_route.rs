use crate::handlers::home_handler;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new().route("/", get(home_handler::index))
}
