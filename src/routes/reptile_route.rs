use crate::handlers::{reptile_handler, zhdc_handler};
use crate::models::reptile_zhdc_books_m::GetQuery;
use axum::extract::{Form, Path, Query};
use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/reptile/test", get(reptile_handler::test_html_select))
        .route(
            "/reptile/test/zhonghuadiancang",
            get(reptile_handler::test_zhonghuadiancang_detail),
        )
        .route(
            "/reptile/zhonghuadiancang/publish/{book_id}",
            get(zhdc_handler::book_publish),
        )
        .route(
            "/reptile/zhonghuadiancang/chapter/publish/{chapter_id}",
            get(zhdc_handler::book_chapter_publish),
        )
        .route(
            "/reptile/zhonghuadiancang/del/{id}",
            get(zhdc_handler::delete),
        )
        .route("/reptile/zhonghuadiancang/new", get(zhdc_handler::new_html))
        .route("/reptile/zhonghuadiancang/new", post(zhdc_handler::new))
        .route(
            "/reptile/zhonghuadiancang/book/{book_id}",
            get(zhdc_handler::book),
        )
        .route(
            "/reptile/zhonghuadiancang",
            get(
                |Query(get): Query<GetQuery>, session: crate::session::Session| async move {
                    zhdc_handler::list_page(1, get, session).await
                },
            ),
        )
        .route(
            "/reptile/zhonghuadiancang/{page}",
            get(
                |Path(page): Path<u32>,
                 Query(get): Query<GetQuery>,
                 session: crate::session::Session| async move {
                    zhdc_handler::list_page(page, get, session).await
                },
            ),
        )
}
