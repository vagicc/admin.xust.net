
CREATE TABLE reptile_zhdc_books (
    "id" SERIAL PRIMARY KEY,
    "name" CHARACTER VARYING(255) UNIQUE NOT NULL,
    "author" CHARACTER VARYING(180) DEFAULT NULL,
    "publishing" CHARACTER VARYING(255) DEFAULT NULL,
    "front_cover" CHARACTER VARYING(255) DEFAULT NULL,
    "front_cover_download" boolean DEFAULT FALSE,
    "category" CHARACTER VARYING(20) DEFAULT NULL,
    "description" CHARACTER VARYING(1800) DEFAULT NULL,
    "finish" BOOLEAN DEFAULT TRUE,
    "seo_title" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_keywords" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_description" CHARACTER VARYING(1000) DEFAULT NULL,
    "reptile_url" CHARACTER VARYING(255) UNIQUE NOT NULL,
    "is_published" BOOLEAN DEFAULT FALSE,
    "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);
CREATE INDEX idx_reptile_zhdc_books_name ON reptile_zhdc_books (name);
CREATE INDEX idx_reptile_zhdc_books_author ON reptile_zhdc_books (author);
CREATE INDEX idx_reptile_zhdc_books_reptile_url ON reptile_zhdc_books (reptile_url);
CREATE INDEX idx_reptile_zhdc_books_is_published ON reptile_zhdc_books (is_published);
COMMENT ON TABLE reptile_zhdc_books IS '爬虫抓取中华典藏网书籍表';
COMMENT ON COLUMN reptile_zhdc_books.name IS '书名';
COMMENT ON COLUMN reptile_zhdc_books.author IS '作者';
COMMENT ON COLUMN reptile_zhdc_books.publishing IS '出版社';
COMMENT ON COLUMN reptile_zhdc_books.front_cover IS '书封面图';
COMMENT ON COLUMN reptile_zhdc_books.front_cover_download IS '书封面图是否已下载';
COMMENT ON COLUMN reptile_zhdc_books.category IS '分类';
COMMENT ON COLUMN reptile_zhdc_books.description IS '简介描述';
COMMENT ON COLUMN reptile_zhdc_books.finish IS '书是否完本';
COMMENT ON COLUMN reptile_zhdc_books.seo_title IS 'SEO标题';
COMMENT ON COLUMN reptile_zhdc_books.seo_keywords IS 'SEO关键词';
COMMENT ON COLUMN reptile_zhdc_books.seo_description IS 'SEO描述';
COMMENT ON COLUMN reptile_zhdc_books.reptile_url IS '抓取原始URL:唯一，可用来判断是否已抓取过';
COMMENT ON COLUMN reptile_zhdc_books.is_published IS '是否已发布';

CREATE TABLE "reptile_zhdc_chapters"(
    "id" SERIAL PRIMARY KEY,
    "zhdc_books_id" INTEGER NOT NULL,
    "book_name" CHARACTER VARYING(255) DEFAULT NULL,
    "title" CHARACTER VARYING(255) NOT NULL,
    "content" TEXT DEFAULT NULL,
    "publish" BOOLEAN DEFAULT FALSE,
    "seo_title" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_keywords" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_description" CHARACTER VARYING(1000) DEFAULT NULL,
    "reptile_url" CHARACTER VARYING(255) UNIQUE NOT NULL,
    "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);
CREATE INDEX idx_reptile_zhdc_chapters_zhdc_books_id ON reptile_zhdc_chapters (zhdc_books_id);
CREATE INDEX idx_reptile_zhdc_chapters_book_name ON reptile_zhdc_chapters (book_name);
CREATE INDEX idx_reptile_zhdc_chapters_title ON reptile_zhdc_chapters (title);
CREATE INDEX idx_reptile_zhdc_chapters_reptile_url ON reptile_zhdc_chapters (reptile_url);
CREATE INDEX idx_reptile_zhdc_chapters_publish ON reptile_zhdc_chapters (publish);
COMMENT ON TABLE reptile_zhdc_chapters IS '爬虫抓取中华典藏网书章节内容表';
COMMENT ON COLUMN reptile_zhdc_chapters.zhdc_books_id IS 'reptile_zhdc_books表ID';
COMMENT ON COLUMN reptile_zhdc_chapters.book_name IS '书籍名称';
COMMENT ON COLUMN reptile_zhdc_chapters.title IS '章节标题';
COMMENT ON COLUMN reptile_zhdc_chapters.content IS '本章内容';
COMMENT ON COLUMN reptile_zhdc_chapters.publish IS '是否已发布';
COMMENT ON COLUMN reptile_zhdc_chapters.seo_title IS 'SEO标题';
COMMENT ON COLUMN reptile_zhdc_chapters.seo_keywords IS 'SEO关键词';
COMMENT ON COLUMN reptile_zhdc_chapters.seo_description IS 'SEO描述';
COMMENT ON COLUMN reptile_zhdc_chapters.reptile_url IS '抓取原始URL:唯一，可用来判断是否已抓取过';
COMMENT ON COLUMN reptile_zhdc_chapters.create_time IS '创建修改时间';