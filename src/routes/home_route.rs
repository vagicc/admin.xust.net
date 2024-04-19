use warp::Filter;
use crate::handlers::home_handler;

// GET: / 
pub fn index() ->  impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let home = warp::get()
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(home_handler::index);
    home

}
