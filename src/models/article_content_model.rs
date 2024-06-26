use crate::db::get_connection;
use crate::schema::article_content;
use crate::schema::article_content::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体合二为一(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Insertable, Queryable, PartialEq, Eq, Deserialize, Serialize)]
#[table_name = "article_content"]
pub struct ArticleContent {
    pub article_id: i32,
    pub content: String,
    pub last_time: Option<NaiveDateTime>,
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
