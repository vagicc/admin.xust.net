use crate::session::Session;
use crate::template::to_html_single;
use crate::template::view;
use handlebars::{to_json, Handlebars};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Map;
use warp::{Rejection, Reply};

/* 响应： /check/link */
pub async fn new_html(session: Session) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("访问了：“/check/link”");

    let mut data = Map::new();
    // let html = to_html_single("reptile_new.html", data);
    let html = view("check/link.html", data, session);

    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回状态码
}

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

pub async fn check_all_url(
    form: NewPost,
    _session: Session,
) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("post:{:#?}", form);
    match form.validate() {
        Ok(post) => {
            let url = post.url;
            let links = get_all_url(url).await;
            if links.is_none() {
                println!("为何为空：{:#?}", links);
            }
            let mut links = links.unwrap(); //当前层URL

            // let next = next_level_url(&links, 1).await;
            let next = next_level_url(&mut links, 1).await;
            

            println!("第一层URL");
            // for link in links {
            //     let url = link.as_str().to_string();
            //     // println!("{}", url);
            // }
            println!("第=====二=============层URL");
            // for link in next {
            //     let url = link.as_str().to_string();
            //     println!("{}", url);
            // }
        }
        Err(e) => {}
    }
    let mut html = "检查URL所有的死链".to_string();
    Ok(warp::reply::html(html)) //直接返回html
}

//改写next_level_url，
//改写思想：
//添加一个全局变量来保存url
//改写原因：是因为所有URL会有重复问题，
//改写后缺点： static静态变量 应该会比较占运行内存的，是否应该换glob
//或者使用可变 &mut引用，会更多好。
//下面代码，可以当使用Mutex的锁简单应用
lazy_static::lazy_static! {
    static ref MY_URLS: std::sync::Mutex<std::collections::HashSet<url::Url>> = std::sync::Mutex::new(std::collections::HashSet::new());
}
fn add_url(url: url::Url) {
    let mut urls = MY_URLS.lock().unwrap();
    urls.insert(url);
}
fn check_url(url: url::Url) {
    let urls = MY_URLS.lock().unwrap();
    if urls.contains(&url) {
        println!("Url exists in the HashSet");
    } else {
        println!("Url does not exist in the HashSet");
    }
}

/* 
深度递归会出错：
thread 'tokio-runtime-worker' has overflowed its stack
fatal runtime error: stack overflow
已放弃 (核心已转储)
翻译：
线程“tokio-runtime-worker”已溢出其堆栈
致命的运行时错误：堆栈溢出
就是溢出了，
除了优化，就是增加堆栈大小：
Rust允许你通过环境变量RUST_MIN_STACK设置每个线程的最小堆栈大小。
比如，你可以在运行你的程序前设置RUST_MIN_STACK=16777216，这将会把堆栈大小设置为16MB。
但是需要注意的是，这个环境变量对主线程不起作用。
刚372
RUST_MIN_STACK=8388608 cargo run >> run.txt
上面这条命令临时解决，来自于
 */
async fn next_level_url(links: &mut std::collections::HashSet<url::Url>, depth: usize) {
    //HashSet 是一种集合，它的特性就是所有的元素都是唯一的，如果你尝试向 HashSet 中添加一个已经存在的元素，它不会有任何效果。因此，HashSet 已经为你保证了所有的 URL 都是唯一的，你不需要做任何更改。
    //这个说法还没去证明

    //这个设置抓取的深度
    if depth > 58 {
        // return temp_links;
    }

    for link in links.clone().iter() {
        /// let url = Url::parse("ftp://rms@example.com")?;
        /// assert_eq!(url.host_str(), Some("example.com"));
        let kk = link.host_str();
        if kk != Some("www.59fayi.com") {
            // println!("别的域名:{:#?}，不做抓取", kk);
            continue;
        }
        // println!("域名：{:#?}",kk);
        let url = link.as_str().to_string();
        if !url.contains("/m/") {
            //contains 包含的意思
            // println!("不是移动端");
            continue;
        }

        // if !temp_links.contains(&link) {}
        println!("<li><a href=\"{}\">{0}</a></li>", url);

        // 取得当前URL下的所有链接
        if let Some(temp) = get_all_url(url).await {
            // temp_links.insert(link.clone()); //当前层
            // temp_links.extend(temp.clone());

            //取得差值，避免重复抓url
            let mut difference: std::collections::HashSet<url::Url> =
                temp.difference(&links).cloned().collect();
            // links.extend(difference.clone()); //把差集放进去

            // 异步递归
            // let future = Box::pin(next_level_url(&temp, depth + 1));
            // let future = Box::pin(next_level_url(&mut difference, depth + 1));
            // let result = future.await;
            // links.extend(result);
            // next_level_url(&mut difference, depth + 1).await;
            Box::pin(next_level_url(&mut difference, depth + 1)).await;
            links.extend(difference); //把差集放进去
        }
    }
}

//异步递归取得下一层这后的所有链接
//这里是异步递归, 这个做 /m/ 的筛选，放到 法议网的  网站地图
//问题，做url唯一性
async fn _old_next_level_url(
    links: &std::collections::HashSet<url::Url>,
    depth: usize,
) -> std::collections::HashSet<url::Url> {
    //HashSet 是一种集合，它的特性就是所有的元素都是唯一的，如果你尝试向 HashSet 中添加一个已经存在的元素，它不会有任何效果。因此，HashSet 已经为你保证了所有的 URL 都是唯一的，你不需要做任何更改。
    //这个说法还没去证明
    let mut temp_links: std::collections::HashSet<url::Url> = std::collections::HashSet::new();
    if depth > 58 {
        return temp_links;
    }
    for link in links {
        /// let url = Url::parse("ftp://rms@example.com")?;
        /// assert_eq!(url.host_str(), Some("example.com"));
        let kk = link.host_str();
        if kk != Some("www.59fayi.com") {
            // println!("别的域名:{:#?}，不做抓取", kk);
            continue;
        }
        // println!("域名：{:#?}",kk);
        let url = link.as_str().to_string();
        if !url.contains("/m/") {
            //contains 包含的意思
            // println!("不是移动端");
            continue;
        }
        // println!("{}", url);

        if !temp_links.contains(&link) {
            println!("<li><a href=\"{}\">{0}</a></li>", url);

            temp_links.insert(link.clone()); //当前层

            if let Some(temp) = get_all_url(url).await {
                temp_links.extend(temp.clone());

                //取得差值，避免重复抓url
                let difference: std::collections::HashSet<url::Url> =
                    temp.difference(&links).cloned().collect();

                // 异步递归
                // let future = Box::pin(next_level_url(&temp, depth + 1));
                let future = Box::pin(_old_next_level_url(&difference, depth + 1));
                let result = future.await;
                temp_links.extend(result);
            }
        }
    }
    temp_links
}

//取得当前页的所有URL，这里还可以做检查是否只返回当前域名下的链接
async fn get_all_url(url: String) -> Option<std::collections::HashSet<url::Url>> {
    let mut links: std::collections::HashSet<url::Url> = std::collections::HashSet::new();
    let res = reqwest::get(&url).await;
    match res {
        Ok(res) => {
            let res = res.text().await.expect("msg");
            let document = select::document::Document::from(res.as_str());
            let temp_url = url::Url::parse(&url).unwrap();
            let base_url = get_base_url(&temp_url, &document);
            // println!("取得的基URL：{:#?}", base_url);
            let base_parser = url::Url::options().base_url(Some(&base_url));
            //取得当前页的URL
            let links: std::collections::HashSet<url::Url> = document
                .find(select::predicate::Name("a"))
                .filter_map(|n| n.attr("href"))
                .filter_map(|link| base_parser.parse(link).ok())
                .collect();
            // println!("当前页所有的URL：{:#?}", links);
            Some(links)
        }
        Err(e) => None,
    }
}

fn get_base_url(url: &url::Url, doc: &select::document::Document) -> url::Url {
    let base_tag_href = doc
        .find(select::predicate::Name("base"))
        .filter_map(|n| n.attr("href"))
        .nth(0);

    let base_url = base_tag_href
        .map_or_else(
            || url::Url::parse(&url[..url::Position::BeforePath]),
            url::Url::parse,
        )
        .expect("取得base_url出错");
    base_url
}
