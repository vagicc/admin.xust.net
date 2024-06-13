-- postgres book表设计
CREATE TABLE books (
    "id" SERIAL PRIMARY KEY,
    "name" CHARACTER VARYING(255) UNIQUE NOT NULL,
    "author" CHARACTER VARYING(180) DEFAULT NULL,
    "publisher" CHARACTER VARYING(255) DEFAULT NULL,
    "front_cover" CHARACTER VARYING(255) DEFAULT NULL,
    "price" MONEY DEFAULT 0.0,
    "category_id" INTEGER DEFAULT NULL,
    "category" CHARACTER VARYING(20) DEFAULT NULL,
    "description" CHARACTER VARYING(1000) DEFAULT NULL,
    "finish" BOOLEAN DEFAULT TRUE,
    "collect" bigint DEFAULT 0,
    "seo_title" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_keywords" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_description" CHARACTER VARYING(1000) DEFAULT NULL,
    "create_id" INTEGER DEFAULT NULL,
    "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);
CREATE INDEX idx_books_name ON books (name);
CREATE INDEX idx_books_author ON books (author);
CREATE INDEX idx_books_create_id ON books (create_id);

COMMENT ON TABLE books IS '书籍表';
COMMENT ON COLUMN books.name IS '书名';
COMMENT ON COLUMN books.author IS '作者';
COMMENT ON COLUMN books.publisher IS '出版社';
COMMENT ON COLUMN books.front_cover IS '书封面图';
COMMENT ON COLUMN books.price IS '原书定价';
COMMENT ON COLUMN books.category_id IS '分类ID';
COMMENT ON COLUMN books.category IS '分类';
COMMENT ON COLUMN books.description IS '简介描述';
COMMENT ON COLUMN books.finish IS '是否已完结';
COMMENT ON COLUMN books.collect IS '收藏次数';
COMMENT ON COLUMN books.seo_title IS 'SEO标题';
COMMENT ON COLUMN books.seo_keywords IS 'SEO关键词';
COMMENT ON COLUMN books.seo_description IS 'SEO描述';
COMMENT ON COLUMN books.create_id IS '创建者ID';

-- 书章节表
CREATE TABLE "book_chapters"(
    "id" SERIAL PRIMARY KEY,
    "book_id" INTEGER DEFAULT NULL,
    "book_name" CHARACTER VARYING(255) DEFAULT NULL,
    "author" CHARACTER VARYING(180) DEFAULT NULL,
    "title" CHARACTER VARYING(255) NOT NULL,
    -- "content" TEXT DEFAULT NULL,
    "visit" bigint NOT NULL DEFAULT 0,
    "previous" INTEGER DEFAULT NULL,
    "next" INTEGER DEFAULT NULL,
    "publish" BOOLEAN DEFAULT TRUE,
    "seo_title" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_keywords" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_description" CHARACTER VARYING(1000) DEFAULT NULL,
    "create_id" INTEGER DEFAULT NULL,
    "create" bigint DEFAULT NULL,
    "last_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);
CREATE INDEX idx_book_chapters_book_id ON book_chapters (book_id);
CREATE INDEX idx_book_chapters_book_name ON book_chapters (book_name);
CREATE INDEX idx_book_chapters_title ON book_chapters (title);
CREATE INDEX idx_book_chapters_publish ON book_chapters (publish);
COMMENT ON TABLE book_chapters IS '书章节内容表';
COMMENT ON COLUMN book_chapters.book_id IS '书籍ID';
COMMENT ON COLUMN book_chapters.book_name IS '书籍名称';
COMMENT ON COLUMN book_chapters.author IS '作者';
COMMENT ON COLUMN book_chapters.title IS '章节标题';
-- COMMENT ON COLUMN book_chapters.content IS '本章内容';
COMMENT ON COLUMN book_chapters.visit IS '阅读次数';
COMMENT ON COLUMN book_chapters.previous IS '上一章（ID）';
COMMENT ON COLUMN book_chapters.next IS '下一章（ID）';
COMMENT ON COLUMN book_chapters.publish IS '是否已发布';
COMMENT ON COLUMN book_chapters.seo_title IS 'SEO标题';
COMMENT ON COLUMN book_chapters.seo_keywords IS 'SEO关键词';
COMMENT ON COLUMN book_chapters.seo_description IS 'SEO描述';
COMMENT ON COLUMN book_chapters.create_id IS '创建者ID';
COMMENT ON COLUMN book_chapters.create IS '创建时间( Unix 时间戳)';
COMMENT ON COLUMN book_chapters.last_time IS '最后修改时间';

-- 书章节正文表
CREATE TABLE "book_chapters_content"(
    "chapter_id" INTEGER NOT NULL,
    "content" TEXT NOT NULL,
    "last_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp(),
    PRIMARY KEY ("chapter_id"),
    FOREIGN KEY ("chapter_id") REFERENCES "book_chapters" ("id")
);
COMMENT ON TABLE book_chapters_content IS '书章节正文表';
COMMENT ON COLUMN book_chapters_content.chapter_id IS '章节ID';
COMMENT ON COLUMN book_chapters_content.content IS '本章内容';
COMMENT ON COLUMN book_chapters_content.last_time IS '最后修改时间';

-- 书分类表
CREATE TABLE book_category(
    "id" SERIAL PRIMARY KEY,
    "category" CHARACTER VARYING(20) UNIQUE NOT NULL,
    "seo_title" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_keywords" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_description" CHARACTER VARYING(1000) DEFAULT NULL,
    "show" BOOLEAN DEFAULT TRUE,
    "order_by" SMALLINT DEFAULT 1,
    "modify_id" INTEGER DEFAULT NULL,
    "modify_time" TIMESTAMP WITHOUT time ZONE DEFAULT NULL,
    "create_id" INTEGER DEFAULT NULL,
    "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);

CREATE INDEX idx_book_category_category ON book_category (category);
CREATE INDEX idx_book_category_order_by ON book_category (order_by);
CREATE INDEX idx_book_category_create_id ON book_category (create_id);

COMMENT ON TABLE book_category IS '书籍分类表';
COMMENT ON COLUMN book_category.id IS '书籍分类ID';
COMMENT ON COLUMN book_category.category IS '书籍分类名';
COMMENT ON COLUMN book_category.seo_title IS 'SEO标题';
COMMENT ON COLUMN book_category.seo_keywords IS 'SEO关键词';
COMMENT ON COLUMN book_category.seo_description IS 'SEO描述';
COMMENT ON COLUMN book_category.show IS '是否显示：默认1显示，0不显示';
COMMENT ON COLUMN book_category.order_by IS '显示先后:小前大后';
COMMENT ON COLUMN book_category.modify_id IS '最后修改者ID';
COMMENT ON COLUMN book_category.modify_time IS '修改时间';
COMMENT ON COLUMN book_category.create_id IS '创建者ID';
COMMENT ON COLUMN book_category.create_time IS '创建时间';


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



