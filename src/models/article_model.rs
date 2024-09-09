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

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = article)]
pub struct NewArticle {
    pub title: String,           //标题
    pub cover: Option<String>,   //列表封面图
    pub summary: Option<String>, //文章摘要
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub category_id: Option<i32>, //文章分类ID
    pub category: Option<String>, //分类名，对应article_category表
    pub columns_id: i32,          //专栏ID，0不属于任何专栏
    pub available: Option<i16>,   //阅读权限：0免费、1登录、2私密
    pub nav_id: Option<i32>,      //所属导航栏
    pub visit: i64,               //阅读次数
    pub collect: i64,             //收藏次数
    pub share: i64,               //分享次数
    pub user_id: Option<i32>,
    pub username: Option<String>,
    pub create: Option<i64>, //创建时间( Unix 时间戳)
    pub last_time: Option<NaiveDateTime>,
}
impl NewArticle {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        let query = diesel::insert_into(article).values(self).returning(id);
        log::debug!(
            "article表插入数据SQL：{:?}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );
        let result = query.get_result::<i32>(&mut connection);
        match result {
            Ok(insert_id) => {
                log::debug!("article插入成功，ID为：{}", insert_id);
                insert_id
            }
            Err(err) => {
                //value too long for type character varying(255) 字段太短，插入内容太长
                log::error!("article插入数据失败了：{}", err);
                0
            }
        }
    }
}

//删除一条记录
pub fn delete(pky: i32) -> usize {
    let query = diesel::delete(article.find(pky));
    log::debug!(
        "article表删除SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );
    let mut conn = get_connection();
    let deleted_rows = query.execute(&mut conn);
    // crate::common::type_v(deleted_rows);
    //变量值：Ok(1)  =>类型： core::result::Result<usize, diesel::result::Error>  删除成功1条数据
    //变量值：Ok(0)  =>类型： core::result::Result<usize, diesel::result::Error>  删除成功0条数据

    match deleted_rows {
        Ok(row) => row,
        Err(e) => {
            log::error!("article表删除数据失败：{}", e);
            0
        }
    }
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
    pub category_id: Option<i32>,       //分类ID
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
        // columns_id专栏ID
        if let Some(columnid) = params.columns_id {
            query=query.filter(columns_id.eq(columnid));
            query_count=query_count.filter(columns_id.eq(columnid));
        }

        if let Some(c_id) = params.columns_id {
            query=query.filter(category_id.eq(c_id));
            query_count=query_count.filter(category_id.eq(c_id));
        }

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

pub fn modify(pk: i32, data: &NewArticle) -> Option<Article> {
    let query = diesel::update(article.find(pk)).set(data);
    log::debug!(
        "article表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut conn = get_connection();
    match query.get_result::<Article>(&mut conn) {
        Ok(result) => Some(result),
        Err(err) => {
            log::error!("article表修改数据失败：{}", err);
            None
        }
    }
}
