use crate::handlers::column_handler;
use crate::session::with_session;
use warp::Filter;

/// GET: /column/index
/// GET: /column/index/{1}
/// GET: /column/create
/// POST: /column/create
/// GET: /column/edit
/// POST: /column/edit
/// GET: /column/delete/{1}
/// POST: /column/expurgate
pub fn index() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let new = warp::get()
        .and(warp::path("column"))
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(with_session())
        .and_then(column_handler::create)
        .or(warp::post()
            .and(warp::path("column"))
            .and(warp::path("create"))
            .and(warp::path::end())
            .and(warp::body::form())
            .and(with_session())
            .and_then(column_handler::new));

    let edit = warp::get()
        .and(warp::path("column"))
        .and(warp::path("edit"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_session())
        .and_then(column_handler::edit)
        .or(warp::post()
            .and(warp::path("column"))
            .and(warp::path("edit"))
            .and(warp::path::param())
            .and(warp::path::end())
            .and(warp::body::form())
            .and(with_session())
            .and_then(column_handler::do_edit));

    let delete = warp::get()
        .and(warp::path("column"))
        .and(warp::path("delete"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(crate::session::with_session())
        .and_then(column_handler::delete)
        .or(warp::post()
            .and(warp::path("column"))
            .and(warp::path("expurgate"))
            .and(warp::path::end())
            .and(warp::body::form())
            .and(with_session())
            .and_then(column_handler::expurgate));


    warp::get()
        .and(warp::path("column"))
        .and(warp::path("index"))
        .and(warp::query::<crate::models::column_model::GetQuery>())
        .and(warp::path::end())
        .and(with_session())
        .and_then(|get, session| async { column_handler::list_page(1, get, session).await })
        .or(warp::get()
            .and(warp::path("column"))
            .and(warp::path("index"))
            .and(warp::path::param())
            .and(warp::query::<crate::models::column_model::GetQuery>())
            .and(warp::path::end())
            .and(with_session())
            .and_then(column_handler::list_page))
        .or(new)
        .or(edit)
        .or(delete)
}
