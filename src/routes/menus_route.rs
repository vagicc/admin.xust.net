use crate::handlers::menus_handler;
use axum::extract::{Form, Path, Query};
use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/menus/index", get(|session: crate::session::Session| async move {
            menus_handler::list(0, session).await
        }))
        .route(
            "/menus/index/{page}",
            get(|Path(page): Path<i32>, session: crate::session::Session| async move {
                menus_handler::list(page, session).await
            }),
        )
        .route("/menus/create/{parent_id}", get(menus_handler::create_html))
        .route(
            "/menus/create/{parent_id}",
            post(menus_handler::do_new),
        )
        .route("/menus/edit/{id}", get(menus_handler::edit))
        .route(
            "/menus/edit/{id}",
            post(menus_handler::do_edit),
        )
        .route("/menus/delete/{id}", get(menus_handler::delete))
        .route(
            "/menus/delete",
            post(|session: crate::session::Session, Form(form): Form<std::collections::HashMap<String, String>>| async move {
                let ids = form
                    .get("del_ids")
                    .map(|s| s.split(',').filter_map(|id| id.parse::<i32>().ok()).collect())
                    .unwrap_or_default();
                menus_handler::expurgate(ids, session).await
            }),
        )
}
