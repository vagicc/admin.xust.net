use crate::handlers::reptile_handler;
use crate::session::with_session;
use warp::Filter;

pub fn new() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let test = warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("test"))
        .and(warp::path::end())
        .and(with_session())
        .and_then(reptile_handler::test_html_select);

    let test_zhonghuadiancang = warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("test"))
        .and(warp::path("zhonghuadiancang"))
        .and(warp::path::end())
        .and(with_session())
        .and_then(reptile_handler::test_zhonghuadiancang_detail);

    let post = warp::post()
        .and(warp::path("reptile"))
        .and(warp::path("new"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and(with_session())
        .and_then(reptile_handler::zhonghuadiancang);
    warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("new"))
        .and(warp::path::end())
        .and(with_session())
        .and_then(reptile_handler::new_html)
        .or(post)
        .or(test)
        .or(test_zhonghuadiancang)
}
