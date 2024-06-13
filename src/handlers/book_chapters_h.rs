use crate::models::book_chapters_content_m;
use crate::models::book_chapters_m;
use crate::session::Session;
use crate::template::view;
use handlebars::to_json;
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;
use warp::{Rejection, Reply};

// GET查询条件
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetQuery {
    pub title: Option<String>, //章节标题
    pub book_id: Option<i32>,
    pub book_name: Option<String>,   //书名
    pub book_author: Option<String>, //作者
    pub publish: Option<i8>,         //是否已发布，0为未选择，1为发布，2为未发布
}

//书籍章节列表
//响应GET: /book/chapters/list/{1}
pub async fn list_page(
    page: u32,
    get: GetQuery,
    session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("GET: /book/chapters/list");

    let (count, list, pages) = book_chapters_m::list_page(
        Some(page),
        Some(crate::constants::PER_PAGE),
        Some(get.clone()),
    );

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));
    data.insert("get".to_string(), to_json(get));

    let html = view("chapters/list.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn get_chapters_content(
    chapter_id: i32,
    _session: Session,
) -> Result<impl Reply, Rejection> {
    let chapters_content = book_chapters_content_m::find_chapters_content(chapter_id);
    if chapters_content.is_none() {
        // NO_CONTENT//
        return crate::common::response_json(
            warp::http::StatusCode::NO_CONTENT,
            None,
            Some("204查无数据".to_owned()),
        );
    }

    let data = chapters_content.unwrap();

    crate::common::response_json(
        warp::http::StatusCode::OK,
        Some(&data),
        // Some(&upload_return),
        Some("取得文章章节正文成功".to_owned()),
    )
}
