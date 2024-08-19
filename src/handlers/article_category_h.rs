use crate::models::article_category_m;
use crate::session::Session;
use crate::template::view;
use handlebars::{to_json};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;
use warp::{Rejection, Reply};

//文章分类列表：/article-category/index
pub async fn list_page(
    page: u32,
    get: article_category_m::GetQuery,
    session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("文章分类列表GET：/article-category/index");

    let (count, list, pages) = article_category_m::list_page(
        Some(page),
        Some(crate::constants::PER_PAGE),
        Some(get.clone()),
    );

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));
    data.insert("get".to_string(), to_json(get));

    let html = view("article-category/list.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}
