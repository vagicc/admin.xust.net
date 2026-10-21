use crate::handlers::book_handler;
use crate::models::books_model::GetQuery;
use axum::extract::{Path, Query};
use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route(
            "/book/test",
            get(
                |Query(params): Query<book_handler::SearchParams>| async move {
                    book_handler::test(params).await
                },
            ),
        )
        .route(
            "/book/list",
            get(
                |Query(get): Query<GetQuery>, session: crate::session::Session| async move {
                    book_handler::list_page(1, get, session).await
                },
            ),
        )
        .route(
            "/book/list/{page}",
            get(
                |Path(page): Path<u32>,
                 Query(get): Query<GetQuery>,
                 session: crate::session::Session| async move {
                    book_handler::list_page(page, get, session).await
                },
            ),
        )
        .route("/book/chapters/{book_id}", get(book_handler::chapters))
}
