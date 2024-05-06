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

/* 响应： new_html*/
pub async fn new_html(session: Session) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("[调试信息]访问了“/demo/redirect”");

    let mut data = Map::new();
    // let html = to_html_single("reptile_new.html", data);
    let html = view("reptile/new.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回状态码
}

//https://www.zhonghuadiancang.com/xuanxuewushu/18616/
//https://www.zhonghuadiancang.com/xuanxuewushu/18783/
//https://www.zhonghuadiancang.com/xueshuzaji/18404/
//https://www.zhonghuadiancang.com/xueshuzaji/18289/

#[derive(Debug, Clone, serde::Deserialize)]
pub struct NewPost {
    pub url: String, //要抓的目录URL
}
impl NewPost {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.url.is_empty() {
            return Err("url不能为空");
        }
        Ok(self.clone())
    }
}

//处理抓取“中华典藏”
pub async fn zhonghuadiancang(
    form: NewPost,
    session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("post:{:#?}", form);
    match form.validate() {
        Ok(post) => {
            let url = post.url.as_str();
            let result = crate::http::http_request(url).await;
            let response = result.unwrap();
            // println!("response: {:?}", response);
            let html = response.html.as_str();
            // println!("抓取到的html=========={}", html);
            println!("到这===");
            let data = crate::parse::zhonghuadiancang_select(html).await;
        }
        Err(e) => {}
    }
    let mut html = "后台命令去抓取法拍车".to_string();
    Ok(warp::reply::html(html)) //直接返回html
}
