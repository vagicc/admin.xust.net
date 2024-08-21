use crate::handlers::article_category_h;
use crate::session::with_session;
use warp::Filter;

/// GET: /article-category/index/{1}
pub fn index() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let new = warp::get()
        .and(warp::path("article-category"))
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(with_session())
        .and_then(article_category_h::create)
        .or(warp::post()
            .and(warp::path("article-category"))
            .and(warp::path("create"))
            .and(warp::path::end())
            .and(warp::body::form())
            .and(with_session())
            .and_then(article_category_h::new));

    let edit = warp::get()
        .and(warp::path("article-category"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_session())
        .and_then(article_category_h::edit)
        .or(warp::post()
            .and(warp::path("article-category"))
            .and(warp::path("edit"))
            .and(warp::path::param())
            .and(warp::path::end())
            .and(warp::body::form())
            .and(with_session())
            .and_then(article_category_h::do_edit));

    warp::get()
        .and(warp::path("article-category"))
        .and(warp::path("index"))
        .and(warp::query::<crate::models::article_category_m::GetQuery>())
        .and(warp::path::end())
        .and(with_session())
        .and_then(|get, session| async { article_category_h::list_page(1, get, session).await })
        .or(warp::get()
            .and(warp::path("article-category"))
            .and(warp::path("index"))
            .and(warp::path::param())
            .and(warp::query::<crate::models::article_category_m::GetQuery>())
            .and(warp::path::end())
            .and(with_session())
            .and_then(article_category_h::list_page))
        .or(new)
        .or(edit)
}
