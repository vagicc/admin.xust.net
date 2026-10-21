use crate::handlers::role_handler;
use axum::extract::{Form, Path, Query};
use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/role/index", get(|session: crate::session::Session| async move {
            role_handler::list(1, session).await
        }))
        .route(
            "/role/index/{page}",
            get(|Path(page): Path<u32>, session: crate::session::Session| async move {
                role_handler::list(page, session).await
            }),
        )
        .route("/role/create", get(role_handler::create_html))
        .route(
            "/role/create",
            post(role_handler::do_new),
        )
        .route(
            "/role/delete",
            post(|session: crate::session::Session, Form(form): Form<std::collections::HashMap<String, String>>| async move {
                let del = form.get("del_ids");
                let ids = del
                    .map(|s| s.split(',').filter_map(|id| id.parse::<i32>().ok()).collect())
                    .unwrap_or_default();
                role_handler::expurgate(ids, session).await
            }),
        )
        .route("/role/delete/{id}", get(role_handler::delete))
        .route("/role/edit/{id}", get(role_handler::edit))
        .route(
            "/role/edit/{id}",
            post(role_handler::do_edit),
        )
}
