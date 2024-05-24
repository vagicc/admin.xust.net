use crate::db::get_connection;
use crate::schema::reptile_zhdc_books;
use crate::schema::reptile_zhdc_books::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct ReptileZhdcBooks {
    pub id: i32,
    pub name: String,
    pub author: Option<String>,
    pub publishing: Option<String>,
    pub front_cover: Option<String>,
    pub front_cover_download: Option<bool>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub finish: Option<bool>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub reptile_url: String,
    pub is_published: Option<bool>,
    pub create_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = reptile_zhdc_books)]
pub struct NewReptileZhdcBooks {
    pub name: String,
    pub author: Option<String>,
    pub publishing: Option<String>,
    pub front_cover: Option<String>,
    pub front_cover_download: Option<bool>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub finish: Option<bool>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub reptile_url: String,
    pub is_published: Option<bool>,
    pub create_time: Option<NaiveDateTime>,
}
impl NewReptileZhdcBooks {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        let query = diesel::insert_into(reptile_zhdc_books)
            .values(self)
            .returning(id);
        log::debug!(
            "reptile_zhdc_books表插入数据SQL：{:?}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );
        let result = query.get_result::<i32>(&mut connection);
        match result {
            Ok(insert_id) => {
                log::debug!("插入成功，ID为：{}", insert_id);
                insert_id
            }
            Err(err) => {
                //value too long for type character varying(255) 字段太短，插入内容太长
                log::error!("插入数据失败了：{}", err);
                0
            }
        }
    }
}

pub fn whether_link_exists(url: String) -> bool {
    let query = reptile_zhdc_books.filter(reptile_url.eq(url));
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("whether_link_exists查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.first::<ReptileZhdcBooks>(&mut connection);
    match result {
        Ok(row) => {
            log::debug!("reptile_zhdc_books存在数据");
            true
        }
        Err(err) => {
            log::debug!("reptile_zhdc_books查无数据：{}", err);
            false
        }
    }
}

/// 取得列表数据
/// page: Option<u32>  第几页
/// per: Option<u32>   每页多少条数据,默认为50
/// 返回（总条数：i64,数据数组，分页html)
pub fn list_page(
    page: Option<u32>,
    per: Option<u32>,
    whe: Option<crate::handlers::zhdc_handler::GetQuery>,
) -> (i64, Vec<ReptileZhdcBooks>, String) {
    let mut limit: i64 = 50; //每页取几条数据
    let mut offset: i64 = 0; //从第0条开始

    if !per.is_none() {
        limit = per.unwrap() as i64;
    }

    if !page.is_none() && page.unwrap() > 1 {
        offset = ((page.unwrap() as i64) - 1) * limit;
    }

    let mut name_where = String::new();
    let mut publish_where = false;

    if let Some(get_data) = whe {
        name_where = get_data.book_name;
        publish_where = get_data.is_published;
    }

    let mut query = reptile_zhdc_books.filter(is_published.eq(publish_where));
    if !name_where.is_empty() {
        query.filter(name.like(name_where)); //这个条件好像不生效
    }

    let query_count = query.count();
    log::error!(
        "reptile_zhdc_books分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count: i64 = query_count
        .get_result(&mut conn)
        .expect("reptile_zhdc_books分页数量查询出错"); //查询总条数

    let mut pages = String::new();
    let data_null: Vec<ReptileZhdcBooks> = Vec::new();
    if count <= 0 {
        return (count, data_null, pages);
    }

    let query = query
        .order_by(id.desc())
        .limit(limit) //取10条数据
        .offset(offset); //从第0条开始;
    log::error!(
        "reptile_zhdc_books分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let list = query
        .get_results::<ReptileZhdcBooks>(&mut conn)
        .unwrap_or(data_null);

    // let page = page.unwrap_or(1);
    pages = crate::pager::default_full("reptile/list", count, page.unwrap_or(1), limit as u32);
    (count, list, pages)
}
