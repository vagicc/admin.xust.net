use crate::handlers::article_handler;
use crate::models::article_model::GetQuery;
use axum::extract::{Form, Path, Query};
use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/article/create", get(article_handler::create))
        .route("/article/create", post(article_handler::new))
        .route("/article/edit/{id}", get(article_handler::edit))
        .route("/article/edit/{id}", post(article_handler::do_edit))
        .route(
            "/article/index",
            get(
                |Query(get): Query<GetQuery>, session: crate::session::Session| async move {
                    article_handler::list_page(1, get, session).await
                },
            ),
        )
        .route(
            "/article/index/{page}",
            get(
                |Path(page): Path<u32>,
                 Query(get): Query<GetQuery>,
                 session: crate::session::Session| async move {
                    article_handler::list_page(page, get, session).await
                },
            ),
        )
}
