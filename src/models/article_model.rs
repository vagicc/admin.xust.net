use std::fmt::format;

use crate::db::get_connection;
use crate::schema::article;
use crate::schema::article::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Queryable, Serialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub cover: Option<String>,
    pub summary: Option<String>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub category_id: Option<i32>,
    pub category: Option<String>,
    pub columns_id: i32,
    pub available: Option<i16>,
    pub nav_id: Option<i32>,
    pub visit: i64,
    pub collect: i64,
    pub share: i64,
    pub user_id: Option<i32>,
    pub username: Option<String>,
    pub create: Option<i64>,
    pub last_time: Option<NaiveDateTime>,
}

/// 通过ID查找文章详情
pub fn get_article(article_id: i32) -> Option<Article> {
    let query = article.find(article_id);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_article查询SQL：{:?}", sql);

    let mut connection = get_connection();
    let result = query.first::<Article>(&mut connection);

    match result {
        Ok(data) => Some(data),
        Err(e) => {
            log::debug!("get_article查无数据：{}", e);
            return None;
        }
    }
}

/// 首页查询最新limit条
pub fn get_new(limit: i64) -> Vec<Article> {
    let query = article.order_by(last_time.desc()).limit(limit);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("article查询最新数据SQL：{:?}", sql);

    let mut connection = get_connection();
    let list = query
        .get_results::<Article>(&mut connection)
        .unwrap_or_else(|_op| {
            let temp: Vec<Article> = Vec::new();
            temp
        });
    list
}

/// 取得最亲列表数据(前端使用)
/// page: Option<u32>  第几页
/// per: Option<u32>   每页多少条数据,默认为50
pub fn article_list(page: Option<u32>, per: Option<u32>) -> (i64, Vec<Article>, String) {
    let mut limit: i64 = 50; //每页取几条数据
    let mut offset: i64 = 0; //从第0条开始

    if !per.is_none() {
        limit = per.unwrap() as i64;
        //u32是无符号整数,也就是大于0
        // if limit < 1 {
        //     limit = 1;
        // }
    }

    if !page.is_none() && page.unwrap() > 1 {
        offset = ((page.unwrap() as i64) - 1) * limit;
        //u32是无符号整数,也就是大于0
        // if offset < 0 {
        //     offset = 0;
        // }
    }

    let query_count = article.count();
    log::debug!(
        "article分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count: i64 = query_count
        .get_result(&mut conn)
        .expect("article分页数量查询出错"); //查询总条数

    let mut pages = String::new();
    let data_null: Vec<Article> = Vec::new();
    if count <= 0 {
        return (count, data_null, pages);
    }

    let query = article.order_by(id.desc()).limit(limit).offset(offset);
    log::debug!(
        "article_list分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );
    let list = query.get_results::<Article>(&mut conn).unwrap_or(data_null);

    pages = crate::pager::default_full("article/index", count, page.unwrap_or(1), limit as u32);
    (count, list, pages)
}

// GET查询条件
#[derive(Debug, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct GetQuery {
    pub title: Option<String>,   //标题
    pub columns_id: Option<i32>, //专栏ID，0不属于任何专栏
    pub c_id: Option<i32>,       //分类ID
    pub available: Option<i16>,  //阅读权限：0免费、1登录、2私密
}

/// 取得列表数据
/// page: Option<u32>  第几页
/// per: Option<u32>   每页多少条数据,默认为50
/// 返回（总条数：i64,数据数组，分页html)
pub fn list_page(
    page: Option<u32>,
    per: Option<u32>,
    whe: Option<GetQuery>,
) -> (i64, Vec<Article>, String) {
    let mut limit: i64 = 50; //每页取几条数据
    let mut offset: i64 = 0; //从第0条开始

    if !per.is_none() {
        limit = per.unwrap() as i64;
    }

    if !page.is_none() && page.unwrap() > 1 {
        offset = ((page.unwrap() as i64) - 1) * limit;
    }

    let mut query = article.into_boxed();
    let mut query_count = article.into_boxed();
    //可变的查询条件以上面结合下面的写法
    if let Some(params) = whe {
        if let Some(title_like) = params.title.filter(|t| !t.is_empty()) {
            let title_like = format!("%{}%", title_like);
            query = query.filter(title.like(title_like.clone()));
            query_count = query_count.filter(title.like(title_like));
        }
        // columns_id专栏ID,c_id分类ID未处理

        //阅读权限：0免费、1登录、2私密
        if let Some(a) = params.available.filter(|a| *a < 3) {
            query = query.filter(available.eq(a));
            query_count = query_count.filter(available.eq(a));
        }
    }

    let query_count = query_count.count();
    log::error!(
        "article分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count: i64 = query_count
        .get_result(&mut conn)
        .expect("Article分页数量查询出错"); //查询总条数

    let mut pages = String::new();
    let data_null: Vec<Article> = Vec::new();
    if count <= 0 {
        return (count, data_null, pages);
    }

    let query = query
        .order_by(id.desc())
        .limit(limit) //取10条数据
        .offset(offset); //从第0条开始;
    log::error!(
        "article分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let list = query.get_results::<Article>(&mut conn).unwrap_or(data_null);

    pages = crate::pager::default_full("article/index", count, page.unwrap_or(1), limit as u32);

    (count, list, pages)
}
