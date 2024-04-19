use crate::template::to_html_single;
use handlebars::to_json;
use serde_json::value::Map;
use warp::{Rejection, Reply};

/* 响应：/demo/redirect*/
pub async fn redirect_index() -> std::result::Result<impl Reply, Rejection> {
    log::debug!("[调试信息]访问了“/demo/redirect”");

    let html = "重定向示例".to_string();

    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回状态码
}

// 输出POST表单
pub async fn show_repeat_html() -> std::result::Result<impl Reply, Rejection> {
    // let html = "首页";
    // Ok(warp::reply::html(html)) //直接返回html

    let mut data = Map::new();
    data.insert(
        "base_url".to_string(),
        to_json(crate::common::get_env("BASE_URL")),
    );

    Ok(warp::reply::html(to_html_single("demo_repeat.html", data)))
}

/* 响应：/demo/redirect/post_repeat */
pub async fn repeat(post: u32) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("[调试信息]访问了“/demo/redirect”");

    let html = "重定向示例".to_string();

    if post == 0 {
        log::warn!("无post数据，输出表单");
        //无post数据，输出表单
        return Err(warp::reject::not_found()); //错误的返回状态码
    }

    //处理完post数据，跳转到列表页
    let k = warp::redirect::see_other(warp::http::Uri::from_static("/demo/redirect/v"));

    Ok(k)
    // Ok(warp::reply::html(html)) //直接返回html
    // Err(warp::reject::not_found())   //错误的返回
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RepeatForm {
    pub username: String,
}
impl RepeatForm {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.username.is_empty() {
            return Err("用户名不能为空");
        }
        Ok(self.clone())
    }
}

pub async fn repeat_post(form: RepeatForm) -> Result<impl Reply, Rejection> {
    match form.validate() {
        Ok(post) => {
            println!("post:{:#?}", post);
        }
        Err(msg) => {
            println!("出错：{}", msg);
        }
    }
    //处理完post数据，跳转到列表页
    Ok(warp::redirect::see_other(warp::http::Uri::from_static(
        "/demo/redirect/v",
    )))
}
