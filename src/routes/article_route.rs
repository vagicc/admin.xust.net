use crate::handlers::article_handler;
use crate::session::with_session;
use warp::Filter;

/// GET: /book/list/{1}
pub fn index() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let new = warp::get()
        .and(warp::path("article"))
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(with_session())
        .and_then(article_handler::create)
        .or(warp::post()
            .and(warp::path("article"))
            .and(warp::path("create"))
            .and(warp::path::end())
            .and(warp::body::form())
            .and(with_session())
            .and_then(article_handler::new));

    let edit = warp::get()
        .and(warp::path("article"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_session())
        .and_then(article_handler::edit)
        .or(warp::post()
            .and(warp::path("article"))
            .and(warp::path("edit"))
            .and(warp::path::param())
            .and(warp::path::end())
            .and(warp::body::form())
            .and(with_session())
            .and_then(article_handler::do_edit));

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
        .or(new)
        .or(edit)
}
