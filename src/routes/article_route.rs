use crate::handlers::article_handler;
use crate::session::with_session;
use warp::Filter;

/// GET: /book/list/{1}
pub fn index() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::get()
        .and(warp::path("article"))
        .and(warp::path("index"))
        .and(warp::query::<crate::models::article_model::GetQuery>())
        .and(warp::path::end())
        .and(with_session())
        .and_then(|get, session| async { article_handler::list_page(1, get, session).await })
        .or(warp::get()
            .and(warp::path("article"))
            .and(warp::path("index"))
            .and(warp::path::param())
            .and(warp::query::<crate::models::article_model::GetQuery>())
            .and(warp::path::end())
            .and(with_session())
            .and_then(article_handler::list_page))
}
