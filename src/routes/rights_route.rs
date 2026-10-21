use crate::handlers::rights_handler;
use axum::extract::{Form, Path, Query};
use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route(
            "/rights/index",
            get(|session: crate::session::Session| async move {
                rights_handler::list(1, session).await
            }),
        )
        .route(
            "/rights/index/{page}",
            get(
                |Path(page): Path<u32>, session: crate::session::Session| async move {
                    rights_handler::list(page, session).await
                },
            ),
        )
        .route("/rights/create", get(rights_handler::create_html))
        .route("/rights/create", post(rights_handler::do_new))
        .route("/rights/edit/{id}", get(rights_handler::edit))
        .route("/rights/edit/{id}", post(rights_handler::do_edit))
        .route("/rights/delete/{id}", get(rights_handler::delete))
}
