use crate::handlers::demo_handler;
use warp::Filter;

/// GET: /demo/redirect
pub fn index() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    // Ok(warp::reply::html(html)) //直接返回html
    warp::get()
        .and(warp::path("demo"))
        .and(warp::path("redirect"))
        .and(warp::path::end())
        .and_then(demo_handler::redirect_index)
        .or(v())
        .or(redirect())
        .or(permanent())
        .or(found())
        .or(see_other())
        .or(temporary())
        .or(post_repeat())
}

///  GET: /demo/redirect/v
pub fn v() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("demo"))
        .and(warp::path("redirect"))
        .and(warp::path("v"))
        .and(warp::path::end())
        .map(|| "这里是V".to_string())
}

/// GET: /demo/redirect/v1
/// 301永久重定向到：/demo/redirect/v
/// 处理方法:GET 方法不会发生变更，其他方法有可能会变更为 GET 方法
/// 典型应用场景:网站重构
pub fn redirect() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("demo"))
        .and(warp::path("redirect"))
        .and(warp::path("v1"))
        .and(warp::path::end())
        .map(|| {
            log::info!("访问了：/demo/redirect/v1，301重定向到：/demo/redirect/v");
            warp::redirect(warp::http::Uri::from_static("/demo/redirect/v"))
        })
}

/// GET: /demo/redirect/v8
/// 308永久重定向到：/demo/redirect/v
/// 处理方法:方法和消息主体都不发生变化
/// 典型应用场景:网站重构，用于非 GET 方法。
pub fn permanent() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("demo"))
        .and(warp::path("redirect"))
        .and(warp::path("v8"))
        .and(warp::path::end())
        .map(|| {
            log::info!("访问：/demo/redirect/v8，308永久重定向到：/demo/redirect/v");
            warp::redirect::permanent(warp::http::Uri::from_static("/demo/redirect/v"))
        })
}

/// GET: /demo/redirect/v7
/// 307临时重定向到：/demo/redirect/v
/// 处理方法:方法和消息主体都不发生变化
/// 典型应用场景:由于不可预见的原因该页面暂不可用。在这种情况下，搜索引擎不会更新它们的链接。当站点支持非 GET 方法的链接或操作的时候，该状态码优于 302 状态码。
pub fn temporary() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("demo"))
        .and(warp::path("redirect"))
        .and(warp::path("v7"))
        .and(warp::path::end())
        .map(|| {
            log::info!("访问GET: /demo/redirect/v7，307临时重定向到：/demo/redirect/v");
            warp::redirect::temporary(warp::http::Uri::from_static("/demo/redirect/v"))
        })
}

/// GET: /demo/redirect/v2
/// 302临时重定向到：/demo/redirect/v
/// 302处理方法:GET 方法不会发生变更，其他方法有可能会变更为 GET 方法
/// 典型应用场景:由于不可预见的原因该页面暂不可用。在这种情况下，搜索引擎不会更新它们的链接。
pub fn found() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("demo"))
        .and(warp::path("redirect"))
        .and(warp::path("v2"))
        .and(warp::path::end())
        .map(|| {
            log::info!("访问：/demo/redirect/v2，302临时重定向到：/demo/redirect/v");
            warp::redirect::found(warp::http::Uri::from_static("/demo/redirect/v"))
        })
}

/// GET: /demo/redirect/v3
/// 303临时重定向到：/demo/redirect/v
/// 处理方法:GET 方法不会发生变更，其他方法会变更为 GET 方法（消息主体会丢失）
/// 典型应用场景:用于PUT 或 POST 请求完成之后进行页面跳转来防止由于页面刷新导致的操作的重复触发
pub fn see_other() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("demo"))
        .and(warp::path("redirect"))
        .and(warp::path("v3"))
        .and(warp::path::end())
        .map(|| {
            log::info!("访问GET: /demo/redirect/v3，303临时重定向到：/demo/redirect/v");
            warp::redirect::see_other(warp::http::Uri::from_static("/demo/redirect/v"))
        })
}

/// GET: /demo/redirect/post_repeat/
/// POST: /demo/redirect/post_repeat/
/// 重复提交表单
pub fn post_repeat() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let post = warp::post()
        .and(warp::path("demo"))
        .and(warp::path("redirect"))
        .and(warp::path("post_repeat"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and_then(demo_handler::repeat_post);
    warp::get()
        .and(warp::path("demo"))
        .and(warp::path("redirect"))
        .and(warp::path("post_repeat"))
        .and(warp::path::end())
        .and_then(demo_handler::show_repeat_html)
        .or(post)
}
