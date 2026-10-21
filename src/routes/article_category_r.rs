use crate::handlers::article_category_h;
use crate::models::article_category_m::GetQuery;
use axum::extract::{Form, Path, Query};
use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/article-category/create", get(article_category_h::create))
        .route("/article-category/create", post(article_category_h::new))
        .route(
            "/article-category/delete/{id}",
            get(article_category_h::delete),
        )
        .route("/article-category/edit/{id}", get(article_category_h::edit))
        .route(
            "/article-category/edit/{id}",
            post(article_category_h::do_edit),
        )
        .route(
            "/article-category/index",
            get(
                |Query(get): Query<GetQuery>, session: crate::session::Session| async move {
                    article_category_h::list_page(1, get, session).await
                },
            ),
        )
        .route(
            "/article-category/index/{page}",
            get(
                |Path(page): Path<u32>,
                 Query(get): Query<GetQuery>,
                 session: crate::session::Session| async move {
                    article_category_h::list_page(page, get, session).await
                },
            ),
        )
}
