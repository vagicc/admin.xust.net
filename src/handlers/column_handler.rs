use crate::models::column_model;
use crate::session::Session;
use crate::template::to_html_single;
use crate::template::view;
use handlebars::to_json;
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;
use warp::{Rejection, Reply};

/* 响应GET： /column/create */
pub async fn create(session: Session) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("[调试信息]访问了“/column/create/”");

    let mut data = Map::new();
    // let html = to_html_single("reptile_new.html", data);
    let html = view("column/create.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回状态码
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ColumnPost {
    pub title: String,                //标题
    pub subhead: String,              //副标题
    pub surface_plot: Option<String>, //封面图
    pub author: Option<String>,       //作者
    pub excerpt: Option<String>,      //简介
    pub price: Option<f64>,           //价格
    pub visit: Option<i64>,           //阅读次数
    pub collect: Option<i64>,         //收藏次数
    pub amount: Option<i32>,          //专栏文章数
    pub complete: Option<i32>,        //已发布文章数
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
}
impl ColumnPost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.title.is_empty() {
            return Err("專欄标题名不能为空");
        }

        Ok(self.clone())
    }
}

/* 响应POST： /column/create */
pub async fn new(form: ColumnPost, session: Session) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("接收到的post:{:#?}", form);
    let mut message = String::new();

    match form.validate() {
        Ok(post) => {
            let now = crate::common::now_naive_date_time();

            let mut price: Option<diesel::data_types::Cents> = None;
            if post.price.is_some() {
                price = Some(diesel::data_types::Cents(post.price.unwrap() as i64 * 100));
            }
            // let price = Some(diesel::data_types::Cents(post.price.unwrap() as i64 * 100));

            let data = column_model::NewColumn {
                title: post.title,
                subhead: post.subhead,
                surface_plot: post.surface_plot,
                author: post.author,
                excerpt: post.excerpt,

                // price: Some(diesel::data_types::PgMoney(8)),
                // price: Some(diesel::data_types::Cents(8)),
                price: price,

                visit: post.visit.unwrap_or(0),
                collect: post.collect.unwrap_or(0),
                amount: post.amount,
                complete: post.complete.unwrap_or(0),
                seo_title: post.seo_title,
                seo_keywords: post.seo_keywords,
                seo_description: post.seo_description,
                create_id: Some(session.admin.id),
                create_time: Some(now),
            };
            if data.insert() != 0 {
                message = "新增專欄成功！".to_string();
            } else {
                message = "插入專欄失败".to_string();
            }
        }
        Err(e) => {
            message = format!("POST表单认证不通过：{}", e);
        }
    }

    let mut data = Map::new();
    data.insert("jump_url".to_string(), to_json("/column/index"));
    data.insert("message".to_string(), to_json(message));

    let html = to_html_single("hint.html", data);
    Ok(warp::reply::html(html)) //直接返回html
}

//單個刪除
pub async fn delete(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let _ = column_model::delete(id);
    // 跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/column/index",
    )))
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct IdsPost {
    // pub ids: Option<Vec<i32>>,
    pub ids: Vec<String>,
}

//多個刪除, post接收數組問題還沒解決，也許試試把i32轉爲String看看
pub async fn expurgate(
    post: IdsPost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    let mut message = String::new();
    let mut data = Map::new();

    log::error!("post:{:#?}", post);

    // if post.ids.is_none() {
    //     data.insert("jump_url".to_string(), to_json("/column/index"));
    //     data.insert(
    //         "message".to_string(),
    //         to_json("沒選中任何需要刪除的數據！？！"),
    //     );
    //     let html = to_html_single("hint.html", data);
    //     return Ok(warp::reply::html(html));
    // }

    // for id in post.ids.unwrap() {
    //     let deleted_row = column_model::delete(id);
    //     if deleted_row == 0 {
    //         message = format!("{} {}ID刪除失敗", message, id);
    //     }
    // }

    message = format!("{} 多選刪除成功！", message);

    data.insert("jump_url".to_string(), to_json("/column/index"));
    data.insert("message".to_string(), to_json(message));

    let html = to_html_single("hint.html", data);
    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn edit(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let mut data = Map::new();
    let edit = column_model::get_column(id);
    if edit.is_none() {
        log::warn!("查无此数据:column表无ID:{}", id);
        data.insert("jump_url".to_string(), to_json("/column/index"));
        data.insert("message".to_string(), to_json("查无此数据:article表"));
        let html = to_html_single("hint.html", data);
        return Ok(warp::reply::html(html));
    }

    data.insert("edit".to_string(), to_json(edit.unwrap()));
    let html = view("column/edit.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn do_edit(
    id: i32,
    form: ColumnPost,
    session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    let mut message = String::new();

    match form.validate() {
        Ok(post) => {
            let now = crate::common::now_naive_date_time();
            let mut price: Option<diesel::data_types::Cents> = None;
            if post.price.is_some() {
                price = Some(diesel::data_types::Cents(post.price.unwrap() as i64 * 100));
            }
            // let price = Some(diesel::data_types::Cents(post.price.unwrap() as i64 * 100));

            let update_data = column_model::NewColumn {
                title: post.title,
                subhead: post.subhead,
                surface_plot: post.surface_plot,
                author: post.author,
                excerpt: post.excerpt,

                // price: Some(diesel::data_types::PgMoney(8)),
                // price: Some(diesel::data_types::Cents(8)),
                price: price,

                visit: post.visit.unwrap_or(0),
                collect: post.collect.unwrap_or(0),
                amount: post.amount,
                complete: post.complete.unwrap_or(0),
                seo_title: post.seo_title,
                seo_keywords: post.seo_keywords,
                seo_description: post.seo_description,
                create_id: None,
                create_time: None,
            };

            let updated = column_model::modify(id, &update_data);
            if updated.is_none() {
                message = "專欄修改出错".to_string();
            } else {
                message = "文章專欄修改成功".to_string();
            }
        }
        Err(e) => {
            message = format!("文章專欄修改POST表单认证不通过：{}", e);
        }
    }

    let mut data = Map::new();
    data.insert("jump_url".to_string(), to_json("/column/index"));
    data.insert("message".to_string(), to_json(message));

    let html = to_html_single("hint.html", data);
    Ok(warp::reply::html(html)) //直接返回html
}

//文章專欄列表：/column/index
pub async fn list_page(
    page: u32,
    get: column_model::GetQuery,
    session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("文章專欄列表GET：/column/index");

    let (count, list, pages) = column_model::list_page(
        Some(page),
        Some(crate::constants::PER_PAGE),
        Some(get.clone()),
    );

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));
    data.insert("get".to_string(), to_json(get));

    let html = view("column/list.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}
