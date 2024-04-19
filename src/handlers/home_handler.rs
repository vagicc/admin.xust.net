use handlebars::{to_json, Handlebars};
use warp::{Rejection, Reply};
// use crate::template::to_html_base;
use serde_json::value::Map;
use crate::template::view;

// type ResultWarp<T> = std::result::Result<T, Rejection>;

/* 响应/请求的返回 */
/// # Example
///
/// ```
/// use warp::{http::Uri, Filter};
///
/// let route = warp::path("v1")
///     .map(|| {
///         warp::redirect(Uri::from_static("/v2"))
///     });
/// ```
pub async fn index(session: crate::session::Session) -> std::result::Result<impl Reply, Rejection> {
    log::warn!("[测试]：{:#?}", session);

    let mut data = Map::new();
    // let html = to_html_base("home.html", data);

    let html = view("index.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回
}
