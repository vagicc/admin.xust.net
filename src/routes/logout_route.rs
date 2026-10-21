use crate::handlers::logout_handler;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new().route("/logout", get(logout_handler::quit))
}
