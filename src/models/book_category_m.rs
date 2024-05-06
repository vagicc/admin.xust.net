use crate::db::get_connection;
use crate::schema::book_category;
use crate::schema::book_category::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct BookCategory {
    pub id: i32,
    pub category: String,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub show:Option<bool>,
    pub order_by:Option<i16>,
    pub modify_id: Option<i32>,
    pub modify_time: Option<NaiveDateTime>,
    pub create_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
}
