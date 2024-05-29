use crate::models::books_model;
use crate::session::Session;
use crate::template::view;
use handlebars::{to_json, Handlebars};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;
use warp::{Rejection, Reply};
// GET查询条件
#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct GetQuery {
    pub book_name: Option<String>,   //书名
    pub book_author: Option<String>, //作者
    pub c_id: Option<i32>,           //分类ID
}

//GET: /book/list/{1}
pub async fn list_page(
    page: u32,
    get: GetQuery,
    session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("GET: /book/list");

    let (count, list, pages) =
        books_model::list_page(Some(page), Some(crate::constants::PER_PAGE), Some(get.clone()));

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));
    data.insert("get".to_string(), to_json(get));

    // let html = to_html_single("reptile_new.html", data);
    let html = view("book/list.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

// 定义查询参数的结构体
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    name: Option<String>,
    age: Option<i32>,
}

pub async fn test(params: SearchParams) -> Result<impl warp::Reply, warp::Rejection> {
    println!("get参数：{:#?}", params);
    let html = format!("测试GET参数：{:#?}", params);
    Ok(warp::reply::html(html)) //直接返回html
}
