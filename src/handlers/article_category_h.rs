use crate::models::article_category_m;
use crate::session::Session;
use crate::template::to_html_single;
use crate::template::view;
use handlebars::to_json;
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

/* 响应GET： /article-category/create */
pub async fn create(session: Session) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("[调试信息]访问了“/article-category/create/”");

    let mut data = Map::new();
    // let html = to_html_single("reptile_new.html", data);
    let html = view("article-category/create.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回状态码
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ArticleCategoryPost {
    pub category: String,
    pub order_by: i16,
    pub show: i16, ////是否显示：默认1显示，0不显示
    pub seo_title: String,
    pub seo_keywords: String,
    pub seo_description: String,
}
impl ArticleCategoryPost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.category.is_empty() {
            return Err("分类名不能为空");
        }

        if self.show > 1 {
            return Err("是否显示：默认1显示，0不显示");
        }

        Ok(self.clone())
    }
}

pub async fn new(
    form: ArticleCategoryPost,
    session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("接收到的post:{:#?}", form);
    let mut message = String::new();

    match form.validate() {
        Ok(post) => {
            let now = crate::common::now_naive_date_time();

            let data = article_category_m::NewArticleCategory {
                category: post.category,
                seo_title: Some(post.seo_title),
                seo_keywords: Some(post.seo_keywords),
                seo_description: Some(post.seo_description),
                show: post.show, //是否显示：默认1显示，0不显示
                order_by: Some(post.order_by),
                modify_id: None,
                modify_time: None,
                create_id: Some(session.admin.id),
                create_time: Some(now),
            };
            if data.insert() != 0 {
                message = "新增文章分类成功！".to_string();
            } else {
                message = "插入文章分类失败".to_string();
            }
        }
        Err(e) => {
            message = format!("POST表单认证不通过：{}", e);
        }
    }

    let mut data = Map::new();
    data.insert("jump_url".to_string(), to_json("/article-category/index"));
    data.insert("message".to_string(), to_json(message));

    let html = to_html_single("hint.html", data);
    Ok(warp::reply::html(html)) //直接返回html
}
