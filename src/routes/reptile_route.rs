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

    warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("test"))
        .and(warp::path("zhonghuadiancang"))
        .and(warp::path::end())
        .and(with_session())
        .and_then(reptile_handler::test_zhonghuadiancang_detail)
        .or(test)
        .or(zhdc())
}

//中华典藏网爬虫
//GET: /reptile/zhonghuadiancang
//GET: /reptile/zhonghuadiancang/new
//GET: /reptile/zhonghuadiancang/publish/{{id}}
//GET: /reptile/zhonghuadiancang/chapter/publish/{{id}}
//GET: /reptile/zhonghuadiancang/del/{{id}}
//POST: /reptile/zhonghuadiancang/new
pub fn zhdc() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let delete = warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("zhonghuadiancang"))
        .and(warp::path("del"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_session())
        .and_then(zhdc_handler::delete);

    let publish = warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("zhonghuadiancang"))
        .and(warp::path("publish"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_session())
        .and_then(zhdc_handler::book_publish);

    let chapter_publish = warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("zhonghuadiancang"))
        .and(warp::path("chapter"))
        .and(warp::path("publish"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_session())
        .and_then(zhdc_handler::book_chapter_publish);

    let book = warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("zhonghuadiancang"))
        .and(warp::path("book"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_session())
        .and_then(zhdc_handler::book);

    let new = warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("zhonghuadiancang"))
        .and(warp::path("new"))
        .and(warp::path::end())
        .and(with_session())
        .and_then(zhdc_handler::new_html)
        .or(warp::post()
            .and(warp::path("reptile"))
            .and(warp::path("zhonghuadiancang"))
            .and(warp::path("new"))
            .and(warp::path::end())
            .and(warp::body::form())
            .and(with_session())
            .and_then(zhdc_handler::new));

    //GET: /reptile/zhonghuadiancang
    use crate::handlers::zhdc_handler;
    use crate::models::reptile_zhdc_books_m;

    //接收GET查询条件
    let _opt_query = warp::query::<reptile_zhdc_books_m::GetQuery>()
        .map(Some)
        .or_else(|_| async {
            Ok::<(Option<reptile_zhdc_books_m::GetQuery>,), std::convert::Infallible>((None,))
        });

    let first_page = warp::get()
        .and(warp::path!("reptile" / "zhonghuadiancang"))
        .and(warp::path::end())
        .and(warp::query::<reptile_zhdc_books_m::GetQuery>())
        .and(with_session())
        .and_then(|get, session: crate::session::Session| async {
            zhdc_handler::list_page(1, get, session).await
        });

    warp::get()
        .and(warp::path("reptile"))
        .and(warp::path("zhonghuadiancang"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::query::<reptile_zhdc_books_m::GetQuery>())
        .and(with_session())
        .and_then(zhdc_handler::list_page)
        .or(first_page)
        .or(new)
        .or(book)
        .or(publish)
        .or(chapter_publish)
        .or(delete)
}
