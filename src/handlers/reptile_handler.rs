use crate::models::reptile_zhdc_books_m;
use crate::models::reptile_zhdc_chapters_m;
use crate::session::Session;
use crate::template::to_html_single;
use crate::template::view;
use handlebars::{to_json, Handlebars};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;
use warp::{Rejection, Reply};
  

pub async fn test_html_select(_: Session) -> std::result::Result<impl Reply, Rejection> {
    Ok(warp::reply::html("这里用来测试HTML")) //直接返回html
}

pub async fn test_zhonghuadiancang_detail(
    _: Session,
) -> std::result::Result<impl Reply, Rejection> {
    let url = "https://www.zhonghuadiancang.com/xuanxuewushu/18783/344485.html".to_string();
    zhonghuadiancang_book_chapter(url).await;
    Ok(warp::reply::html("这里用来测试HTML")) //直接返回html
}

//抓取书详细章节内容
//测试url:https://www.zhonghuadiancang.com/xuanxuewushu/18783/344485.html
async fn zhonghuadiancang_book_chapter(url: String) {
    let result = crate::http::http_request(&url).await;
    let response = result.unwrap();
    // println!("response: {:?}", response);
    let html = response.html.as_str();

    // println!("抓取到的html=========={}", html);
    println!("到这===");
    let data = crate::parse::zhdc_book_chapter_select(html).await;
}
