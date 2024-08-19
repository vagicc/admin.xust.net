use crate::models::article_content_m;
use crate::models::article_model;
use crate::session::Session;
use crate::template::to_html_single;
use crate::template::view;
use handlebars::{to_json, Handlebars};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;
use warp::{Rejection, Reply};

//文章列表
//响应GET: /article/index/{1}
pub async fn list_page(
    page: u32,
    get: article_model::GetQuery,
    session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("文章列表GET: /book/list");

    let (count, list, pages) = article_model::list_page(
        Some(page),
        Some(crate::constants::PER_PAGE),
        Some(get.clone()),
    );

    let mut data = Map::new();
    data.insert("list_len".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));
    data.insert("get".to_string(), to_json(get));

    // let html = to_html_single("reptile_new.html", data);
    let html = view("article/list.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ArticlePost {
    pub title: String,            //文章标题
    pub username: String,         //展示文章发表人
    pub nav_id: Option<i32>,      //所属导航栏ID
    pub category_id: Option<i32>, //文章分类id
    pub columns_id: i32,          //专栏ID，0不属于任何专栏
    pub available: i16,           //阅读权限：0免费、1登录、2私密
    pub summary: Option<String>,  //文章摘要
    pub content: Option<String>,  //文章内容
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
}
impl ArticlePost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.title.is_empty() {
            return Err("文章标题不能为空");
        }
        // if self.content.is_empty() {
        //     return Err("文章内容不能为空");
        // }
        Ok(self.clone())
    }
}

/* 响应： new_html*/
pub async fn create(session: Session) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("[调试信息]访问了“/article/create/”");

    let mut data = Map::new();
    // let html = to_html_single("reptile_new.html", data);
    let html = view("article/create.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回状态码
}

pub async fn new(
    form: ArticlePost,
    _session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("接收到的post:{:#?}", form);
    let mut message = String::new();

    match form.validate() {
        Ok(post) => {
            let time = crate::common::time() as i64;
            let now = crate::common::now_naive_date_time();
            let data = article_model::NewArticle {
                title: post.title,
                cover: None,
                summary: post.summary,
                seo_title: post.seo_title,
                seo_keywords: post.seo_keywords,
                seo_description: post.seo_description,
                category_id: post.category_id,
                category: None,
                columns_id: post.columns_id,
                available: Some(post.available),
                nav_id: post.nav_id,
                visit: 0,
                collect: 0,
                share: 0,
                user_id: None,
                username: None,
                create: Some(time), //创建时间( Unix 时间戳)
                last_time: Some(now),
            };
            let article_id = data.insert();

            if article_id == 0 {
                message = format!("插入文章出错：{}", article_id);
            } else {
                //插入文章内容
                if post.content.is_some() {
                    let new = article_content_m::ArticleContent {
                        article_id: article_id,
                        content: post.content.unwrap(),
                        last_time: Some(now),
                    };
                    if new.insert() == 0 {
                        message = "插入文章内容出错".to_string();
                    } else {
                        message = "新增文章成功！！".to_string();
                    }
                }
            }
        }
        Err(e) => {
            message = format!("POST表单认证不通过：{}", e);
        }
    }

    let mut data = Map::new();
    data.insert("jump_url".to_string(), to_json("/article/index"));
    data.insert("message".to_string(), to_json(message));

    let html = to_html_single("hint.html", data);
    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn edit(id: i32, session: crate::session::Session) -> Result<impl Reply, Rejection> {
    let mut data = Map::new();
    let edit = article_model::get_article(id);
    if edit.is_none() {
        log::warn!("查无此数据:article表无ID:{}", id);
        data.insert("jump_url".to_string(), to_json("/article/index"));
        data.insert("message".to_string(), to_json("查无此数据:article表"));
        let html = to_html_single("hint.html", data);
        return Ok(warp::reply::html(html));
    }

    let content = article_content_m::get_article_content(id);

    use crate::models::roles_model::get_all_role;
    let all_roles = get_all_role();

    data.insert("all_roles".to_string(), to_json(all_roles));
    data.insert("edit".to_string(), to_json(edit.unwrap()));
    data.insert("article_content".to_string(), to_json(content));
    let html = view("article/edit.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
}

pub async fn do_edit(
    id: i32,
    form: ArticlePost,
    _session: crate::session::Session,
) -> Result<impl Reply, Rejection> {
    let mut message = String::new();
    match form.validate() {
        Ok(post) => {
            let now = crate::common::now_naive_date_time();
            let newArticle = article_model::NewArticle {
                title: post.title,
                cover: None,
                summary: post.summary,
                seo_title: post.seo_title,
                seo_keywords: post.seo_keywords,
                seo_description: post.seo_description,
                category_id: post.category_id,
                category: None,
                columns_id: post.columns_id,
                available: Some(post.available),
                nav_id: post.nav_id,
                visit: 0,
                collect: 0,
                share: 0,
                user_id: None,
                username: None,
                create: None, //创建时间( Unix 时间戳)
                last_time: Some(now),
            };

            let updated = article_model::modify(id, &newArticle);
            if updated.is_none() {
                // return //更新出错
                message = "文章修改出错".to_string();
            } else {
                let data = article_content_m::ArticleContent {
                    article_id: id,
                    content: post.content.unwrap(),
                    last_time: Some(now),
                };

                if article_content_m::modify(id, &data).is_none() {
                    message = "文章修改失败".to_string();
                } else {
                    message = "文章修改成功".to_string();
                }
            }
            // return //更新成功
        }
        Err(e) => {
            // return 表单认证失败
            message = format!("文章修改POST表单认证不通过：{}", e);
        }
    }

    let mut data = Map::new();
    data.insert("jump_url".to_string(), to_json("/article/index"));
    data.insert("message".to_string(), to_json(message));

    let html = to_html_single("hint.html", data);
    Ok(warp::reply::html(html)) //直接返回html
}
