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
        .or(zhdc())
}

pub fn zhdc() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //GET: /reptile/zhonghuadiancang
    use crate::handlers::zhdc_handler;

    //接收GET查询条件
    let opt_query = warp::query::<zhdc_handler::GetQuery>()
        .map(Some)
        .or_else(|_| async {
            Ok::<(Option<zhdc_handler::GetQuery>,), std::convert::Infallible>((None,))
        });

    let first = warp::get()
        .and(warp::path!("reptile" / "zhonghuadiancang"))
        .and(warp::path::end())
        .and(opt_query)
        .and(with_session())
        .and_then(
            |get: Option<zhdc_handler::GetQuery>, session: crate::session::Session| async {
                zhdc_handler::list_page(1, get, session).await
            },
        );
        
    warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("zhonghuadiancang"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(opt_query)
        .and(with_session())
        .and_then(zhdc_handler::list_page)
        .or(first)

    // warp::get()
    //     .and(warp::path("reptile"))
    //     .and(warp::path("zhonghuadiancang"))
    //     .and(warp::path::end())
    //     .and(with_session())
    //     .and_then(zhdc_handler::list_old)
}
