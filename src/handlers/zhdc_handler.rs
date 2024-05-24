use crate::models::reptile_zhdc_books_m;
use crate::models::reptile_zhdc_chapters_m;
use crate::session::Session;
use crate::template::to_html_single;
use crate::template::view;
use handlebars::{to_json, Handlebars};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;
use warp::{Rejection, Reply};

//GET: /reptile/zhonghuadiancang
pub async fn list_old(session: Session) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("GET: /reptile/zhonghuadiancang");
    let mut data = Map::new();
    // let html = to_html_single("reptile_new.html", data);
    let html = view("zhdc/list.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn list_page(
    page: u32,
    get: Option<GetQuery>,
    session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("GET: /reptile/zhonghuadiancang");

    let per: u32 = 8; //每页总数
    let (count, list, pages) = reptile_zhdc_books_m::list_page(Some(page), Some(per), get);


    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));

    // let html = to_html_single("reptile_new.html", data);
    let html = view("zhdc/list.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

// GET查询条件
#[derive(Debug, Deserialize, Serialize)]
pub struct GetQuery {
    pub book_name: String,  //书名
    pub is_published: bool, //是否已推送
}
