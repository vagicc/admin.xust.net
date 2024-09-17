use crate::db::get_connection;
use crate::schema::article_category;
use crate::schema::article_category::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Queryable, Serialize)]
pub struct ArticleCategory {
    pub id: i32,
    pub category: String,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub show: i16, //是否显示：默认1显示，0不显示
    pub order_by: Option<i16>,
    pub modify_id: Option<i32>,
    pub modify_time: Option<NaiveDateTime>,
    pub create_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = article_category)]
pub struct NewArticleCategory {
    pub category: String,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub show: i16, //是否显示：默认1显示，0不显示
    pub order_by: Option<i16>,
    pub modify_id: Option<i32>,
    pub modify_time: Option<NaiveDateTime>,
    pub create_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
}
impl NewArticleCategory {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        let query = diesel::insert_into(article_category)
            .values(self)
            .returning(id);
        log::debug!(
            "article_category表插入数据SQL：{:?}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );
        let result = query.get_result::<i32>(&mut connection);
        match result {
            Ok(insert_id) => {
                log::debug!("article_category插入成功，ID为：{}", insert_id);
                insert_id
            }
            Err(err) => {
                //value too long for type character varying(255) 字段太短，插入内容太长
                log::error!("article_category插入数据失败了：{}", err);
                0
            }
        }
    }
}

// GET查询条件
#[derive(Debug, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct GetQuery {
    pub category: Option<String>, //分类名
    pub show: Option<i16>,        //是否显示：默认1显示，0不显示
}

/// 取得列表数据
/// page: Option<u32>  第几页
/// per: Option<u32>   每页多少条数据,默认为50
/// 返回（总条数：i64,数据数组，分页html)
pub fn list_page(
    page: Option<u32>,
    per: Option<u32>,
    whe: Option<GetQuery>,
) -> (i64, Vec<ArticleCategory>, String) {
    let mut limit: i64 = 50; //每页取几条数据
    let mut offset: i64 = 0; //从第0条开始

    if !per.is_none() {
        limit = per.unwrap() as i64;
    }

    if !page.is_none() && page.unwrap() > 1 {
        offset = ((page.unwrap() as i64) - 1) * limit;
    }

    let mut query = article_category.into_boxed();
    let mut query_count = article_category.into_boxed();
    //可变的查询条件以上面结合下面的写法
    if let Some(params) = whe {
        if let Some(category_like) = params.category.filter(|t| !t.is_empty()) {
            let category_like = format!("%{}%", category_like);
            query = query.filter(category.like(category_like.clone()));
            query_count = query_count.filter(category.like(category_like));
        }

        //是否显示：默认1显示，0不显示
        if let Some(a) = params.show.filter(|a| *a < 2) {
            query = query.filter(show.eq(a));
            query_count = query_count.filter(show.eq(a));
        }
    }

    let query_count = query_count.count();
    log::debug!(
        "article_category分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count: i64 = query_count
        .get_result(&mut conn)
        .expect("article_category分页数量查询出错"); //查询总条数

    let mut pages = String::new();
    let data_null: Vec<ArticleCategory> = Vec::new();
    if count <= 0 {
        return (count, data_null, pages);
    }

    let query = query
        .order_by(id.desc())
        .limit(limit) //取10条数据
        .offset(offset); //从第0条开始;
    log::debug!(
        "article_category分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let list = query
        .get_results::<ArticleCategory>(&mut conn)
        .unwrap_or(data_null);

    pages = crate::pager::default_full(
        "article-category/index",
        count,
        page.unwrap_or(1),
        limit as u32,
    );

    (count, list, pages)
}

/// 通过ID查找文章详情
pub fn get_article_category(pky: i32) -> Option<ArticleCategory> {
    let query = article_category.find(pky);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_article_category查询SQL：{:?}", sql);

    let mut connection = get_connection();
    let result = query.first::<ArticleCategory>(&mut connection);

    match result {
        Ok(data) => Some(data),
        Err(e) => {
            log::debug!("get_article_category查无数据：{}", e);
            return None;
        }
    }
}

//删除一条记录
pub fn delete(pky: i32) -> usize {
    let query = diesel::delete(article_category.find(pky));
    log::debug!(
        "article_category表删除SQL：{:?}",
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
            log::error!("article_category表删除数据失败：{}", e);
            0
        }
    }
}

pub fn modify(pk: i32, data: &NewArticleCategory) -> Option<ArticleCategory> {
    let query = diesel::update(article_category.find(pk)).set(data);
    log::debug!(
        "article_category表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut conn = get_connection();
    match query.get_result::<ArticleCategory>(&mut conn) {
        Ok(result) => Some(result),
        Err(err) => {
            log::error!("article_category表修改数据失败：{}", err);
            None
        }
    }
}

/* 表查询插入结构体(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Queryable, Serialize)]
pub struct ArticleCategoryName {
    pub id: i32,
    pub category: String,
}

pub fn all_article_category() -> Option<Vec<ArticleCategoryName>> {
    let query = article_category.select((id, category)).order_by(id.desc());

    log::warn!(
        "all_article_category表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );
    let mut conn = get_connection();
    let result = query.get_results::<ArticleCategoryName>(&mut conn);
    match result {
        Ok(d) => Some(d),
        Err(e) => {
            log::error!("article_category表查找所有分類失敗：{}", e);
            None
        }
    }
}

pub fn all_article_category23() -> Option<Vec<(i32, String)>> {
    let query = article_category.select((id, category)).order_by(id.desc());
    log::debug!(
        "all_article_category表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );
    let mut conn = get_connection();
    let result = query.get_results::<(i32, String)>(&mut conn);
    match result {
        Ok(d) => Some(d),
        Err(e) => {
            log::error!("article_category表查找所有分類失敗：{}", e);
            None
        }
    }
}