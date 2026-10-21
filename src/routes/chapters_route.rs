use crate::handlers::book_chapters_h;
use crate::models::book_chapters_m::GetQuery;
use axum::extract::{Path, Query};
use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route(
            "/book/chapters/list",
            get(
                |Query(get): Query<GetQuery>, session: crate::session::Session| async move {
                    book_chapters_h::list_page(1, get, session).await
                },
            ),
        )
        .route(
            "/book/chapters/list/{page}",
            get(
                |Path(page): Path<u32>,
                 Query(get): Query<GetQuery>,
                 session: crate::session::Session| async move {
                    book_chapters_h::list_page(page, get, session).await
                },
            ),
        )
        .route(
            "/book/chapters/content/{chapter_id}",
            get(book_chapters_h::get_chapters_content),
        )
}
