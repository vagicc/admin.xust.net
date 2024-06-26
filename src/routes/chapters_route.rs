use crate::handlers::book_chapters_h;
use crate::session::with_session;
use warp::Filter;
use crate::models::book_chapters_m::GetQuery;

/// GET: /book/chapters/list/{1}
pub fn list() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("book"))
        .and(warp::path("chapters"))
        .and(warp::path("list"))
        .and(warp::query::<GetQuery>())
        .and(warp::path::end())
        .and(with_session())
        .and_then(|get, session| async { book_chapters_h::list_page(1, get, session).await })
        .or(warp::get()
            .and(warp::path("book"))
            .and(warp::path("chapters"))
            .and(warp::path("list"))
            .and(warp::path::param())
            .and(warp::query::<GetQuery>())
            .and(warp::path::end())
            .and(with_session())
            .and_then(book_chapters_h::list_page))
        .or(get_content())
}

/// GET: /book/chapters/content/{1}
pub fn get_content(
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("book"))
        .and(warp::path("chapters"))
        .and(warp::path("content"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_session())
        .and_then(book_chapters_h::get_chapters_content)
}
