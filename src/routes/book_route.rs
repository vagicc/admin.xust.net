use crate::handlers::book_handler;
use crate::session::with_session;
use warp::Filter;

/// GET: /book/list/{1}
pub fn index() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let test = warp::get()
        .and(warp::path("book"))
        .and(warp::path("test"))
        .and(warp::query::<book_handler::SearchParams>())
        .and(warp::path::end())
        .and_then(book_handler::test);

    warp::get()
        .and(warp::path("book"))
        .and(warp::path("list"))
        .and(warp::query::<book_handler::GetQuery>())
        .and(warp::path::end())
        .and(with_session())
        .and_then(|get, session| async { book_handler::list_page(1, get, session).await })
        .or(warp::get()
            .and(warp::path("book"))
            .and(warp::path("list"))
            .and(warp::path::param())
            .and(warp::query::<book_handler::GetQuery>())
            .and(warp::path::end())
            .and(with_session())
            .and_then(book_handler::list_page))
        .or(test)
        .or(chapters())
}

/// GET: /book/chapters/{1}
pub fn chapters(
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("book"))
        .and(warp::path("chapters"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_session())
        .and_then(book_handler::chapters)
}
