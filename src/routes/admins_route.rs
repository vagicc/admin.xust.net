use crate::handlers::admins_handler;
use crate::models::admins_model::AdminPost;
use axum::extract::{Form, Path, Query};
use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route(
            "/admins/index",
            get(|session: crate::session::Session| async move {
                admins_handler::list(1, session).await
            }),
        )
        .route(
            "/admins/index/{page}",
            get(
                |Path(page): Path<u32>, session: crate::session::Session| async move {
                    admins_handler::list(page, session).await
                },
            ),
        )
        .route("/admins/create", get(admins_handler::create_html))
        .route("/admins/create", post(admins_handler::do_new))
        .route("/admins/edit/{id}", get(admins_handler::edit))
        .route("/admins/edit/{id}", post(admins_handler::do_edit))
        .route("/admins/delete/{id}", get(admins_handler::delete))
}
