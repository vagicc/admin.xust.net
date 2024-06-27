use crate::routes::admins_route;
use crate::routes::demo_route;
use crate::routes::home_route;
use crate::routes::login_route;
use crate::routes::menus_route;
use crate::routes::role_route;
use warp::Filter;

pub fn all_routes(
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let favicon = warp::get()
        .and(warp::path("favicon.ico"))
        .and(warp::path::end())
        .and(warp::fs::file("./static/favicon.ico"));

    //.well-known  此目录是申请免费https证书用到
    let well = warp::path(".well-known").and(warp::fs::dir("./static/.well-known"));
    //静态文件目录
    let dir = warp::path("static").and(warp::fs::dir("./static"));
    let login = login_route::index();
    let menus = menus_route::index();
    let admins = admins_route::index();
    let role = role_route::index();
    let rights = crate::routes::rights_route::index();
    let logout = crate::routes::logout_route::quit();
    let home = home_route::index();

    let hello = warp::path!("hello" / String).map(|name| format!("你好，{}!", name));
    let demo = demo_route::index();

    let reptile = crate::routes::reptile_route::new();
    let check = crate::routes::check_route::check_link();
    let book = crate::routes::book_route::index();
    let chapters = crate::routes::chapters_route::list();
    let article = crate::routes::article_route::index();

    let routes = home
        .or(favicon)
        .or(well)
        .or(dir)
        .or(login)
        .or(menus)
        .or(admins)
        .or(role)
        .or(rights)
        .or(logout)
        .or(hello)
        .or(demo)
        .or(check)
        .or(book)
        .or(chapters)
        .or(reptile)
        .or(article)
        .recover(crate::session::inaccessible);
    routes
}
