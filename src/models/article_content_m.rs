use crate::db::get_connection;
use crate::schema::article_content;
use crate::schema::article_content::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体(Insertable：插入，Queryable：查询，AsChangeset：更新) */
#[derive(
    Debug, Clone, Insertable, AsChangeset, Queryable, PartialEq, Eq, Deserialize, Serialize,
)]
#[table_name = "article_content"]
pub struct ArticleContent {
    pub article_id: i32,
    pub content: String,
    pub last_time: Option<NaiveDateTime>,
}
impl ArticleContent {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        let query = diesel::insert_into(article_content)
            .values(self)
            .returning(article_id);
        log::debug!(
            "article_content表插入数据SQL：{:?}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );
        let result = query.get_result::<i32>(&mut connection);
        match result {
            Ok(insert_id) => {
                log::debug!("article_content插入成功，ID为：{}", insert_id);
                insert_id
            }
            Err(err) => {
                //value too long for type character varying(255) 字段太短，插入内容太长
                log::error!("article_content插入数据失败了：{}", err);
                0
            }
        }
    }
}

/// 通过ID查找文章详情
pub fn get_article_content(articleid: i32) -> Option<ArticleContent> {
    let query = article_content.find(articleid);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_article_content查询SQL：{:?}", sql);

    let mut connection = get_connection();
    let result = query.first::<ArticleContent>(&mut connection);

    match result {
        Ok(data) => Some(data),
        Err(e) => {
            log::debug!("get_article_content查无数据：{}", e);
            return None;
        }
    }
}

pub fn modify(pk: i32, data: &ArticleContent) -> Option<ArticleContent> {
    let query = diesel::update(article_content.find(pk)).set(data);
    log::error!(
        "article_content表更新数据SQL：{:?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let mut conn = get_connection();
    match query.get_result::<ArticleContent>(&mut conn) {
        Ok(result) => Some(result),
        Err(err) => {
            log::error!("article_content表修改数据失败：{}", err);
            None
        }
    }
}
