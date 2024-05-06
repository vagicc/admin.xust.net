use crate::handlers::check_link_h;
use crate::session::with_session;
use warp::Filter;

pub fn check_link() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::post()
        .and(warp::path("check"))
        .and(warp::path("link"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and(with_session())
        .and_then(check_link_h::check_all_url);
    warp::get()
        .and(warp::path("check"))
        .and(warp::path("link"))
        .and(warp::path::end())
        .and(with_session())
        .and_then(check_link_h::new_html)
        .or(post)
}
