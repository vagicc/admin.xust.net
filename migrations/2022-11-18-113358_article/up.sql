-- 文章表
CREATE TABLE article(
    "id" SERIAL PRIMARY KEY,
    "title" CHARACTER VARYING(180) NOT NULL,
    "cover" CHARACTER VARYING(255) DEFAULT NULL,
    "summary" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_title" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_keywords" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_description" CHARACTER VARYING(255) DEFAULT NULL,
    "category_id" INTEGER DEFAULT NULL,
    "category" CHARACTER VARYING(20) DEFAULT NULL,
    "columns_id" INTEGER NOT NULL DEFAULT 0,
    "available" SMALLINT DEFAULT 0,
    "nav_id" INTEGER DEFAULT NULL,
    "visit" bigint NOT NULL DEFAULT 0,
    "collect" bigint NOT NULL DEFAULT 0,
    "share" bigint NOT NULL DEFAULT 0,
    "user_id" INTEGER DEFAULT NULL,
    "username" CHARACTER VARYING(50) DEFAULT NULL,
    "create" bigint DEFAULT NULL,
    "last_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);
CREATE INDEX idx_article_title ON article (title);
CREATE INDEX idx_article_category_id ON article (category_id);
CREATE INDEX idx_article_available ON article (available);
CREATE INDEX idx_article_user_id ON article (user_id);

COMMENT ON TABLE article IS '文章表';
COMMENT ON COLUMN article.title IS '标题';
COMMENT ON COLUMN article.cover IS '列表封面图';
COMMENT ON COLUMN article.summary IS '文章摘要';
COMMENT ON COLUMN article.seo_title IS 'SEO标题';
COMMENT ON COLUMN article.seo_keywords IS 'SEO关键词';
COMMENT ON COLUMN article.seo_description IS 'SEO描述';
COMMENT ON COLUMN article.category_id IS '文章分类ID';
COMMENT ON COLUMN article.category IS '分类名，对应article_category表';
COMMENT ON COLUMN article.columns_id IS '专栏ID，0不属于任何专栏';
COMMENT ON COLUMN article.available IS '阅读权限：0免费、1登录、2私密';
COMMENT ON COLUMN article.nav_id IS '所属导航栏';
COMMENT ON COLUMN article.visit IS '阅读次数';
COMMENT ON COLUMN article.collect IS '收藏次数';
COMMENT ON COLUMN article.share IS '分享次数';
COMMENT ON COLUMN article.user_id IS '文章发表用户ID';
COMMENT ON COLUMN article.username IS '展示文章发表人';
COMMENT ON COLUMN article.create IS '创建时间( Unix 时间戳)';
COMMENT ON COLUMN article.last_time IS '最后修改时间';

-- 文章内容表
CREATE TABLE article_content(
    "article_id" INTEGER NOT NULL,
    "content" TEXT NOT NULL,
    "last_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp(),
    PRIMARY KEY ("article_id"),
    FOREIGN KEY ("article_id") REFERENCES "article" ("id")
);
COMMENT ON TABLE article_content IS '文章内容表';
COMMENT ON COLUMN article_content.article_id IS '文章ID';
COMMENT ON COLUMN article_content.content IS '文章内容';

-- 文章分类表
CREATE TABLE article_category(
    "id" SERIAL PRIMARY KEY,
    "category" CHARACTER VARYING(20) NOT NULL,
    "seo_title" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_keywords" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_description" CHARACTER VARYING(255) DEFAULT NULL,
    "show" SMALLINT NOT NULL DEFAULT 1,
    "order_by" SMALLINT DEFAULT 1,
    "modify_id" INTEGER DEFAULT NULL,
    "modify_time" TIMESTAMP WITHOUT time ZONE DEFAULT NULL,
    "create_id" INTEGER DEFAULT NULL,
    "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);

CREATE INDEX idx_article_category_category ON article_category (category);
CREATE INDEX idx_article_category_order_by ON article_category (order_by);

COMMENT ON TABLE article_category IS '文章分类表';
COMMENT ON COLUMN article_category.id IS '文章分类ID';
COMMENT ON COLUMN article_category.category IS '文章分类名';
COMMENT ON COLUMN article_category.seo_title IS 'SEO标题';
COMMENT ON COLUMN article_category.seo_keywords IS 'SEO关键词';
COMMENT ON COLUMN article_category.seo_description IS 'SEO描述';
COMMENT ON COLUMN article_category.show IS '是否显示：默认1显示，0不显示';
COMMENT ON COLUMN article_category.order_by IS '显示先后:小前大后';
COMMENT ON COLUMN article_category.modify_id IS '最后修改者ID';
COMMENT ON COLUMN article_category.modify_time IS '修改时间';
COMMENT ON COLUMN article_category.create_id IS '创建者ID';
COMMENT ON COLUMN article_category.create_time IS '创建时间';

-- 文章专栏表
CREATE TABLE "column"(
    "id" SERIAL PRIMARY KEY,
    "title" CHARACTER VARYING(50) NOT NULL,
    "subhead" CHARACTER VARYING(80) NOT NULL,
    "surface_plot" CHARACTER VARYING(255) DEFAULT NULL,
    "author" CHARACTER VARYING(50) DEFAULT NULL,
    "excerpt" TEXT DEFAULT NULL,
    "price" MONEY DEFAULT 0.0,
    "visit" bigint NOT NULL DEFAULT 0,
    "collect" bigint NOT NULL DEFAULT 0,
    "amount" int DEFAULT 0,
    "complete" int NOT NULL DEFAULT 0,
    "seo_title" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_keywords" CHARACTER VARYING(255) DEFAULT NULL,
    "seo_description" CHARACTER VARYING(255) DEFAULT NULL,
    "create_id" INTEGER DEFAULT NULL,
    "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp()
);

CREATE INDEX idx_column_title ON "column" (title);

COMMENT ON TABLE "column" IS '文章专栏表';
COMMENT ON COLUMN "column".id IS '文章专栏ID';
COMMENT ON COLUMN "column".title IS '标题';
COMMENT ON COLUMN "column".subhead IS '副标题';
COMMENT ON COLUMN "column".surface_plot IS '封面图';
COMMENT ON COLUMN "column".author IS '作者';
COMMENT ON COLUMN "column".excerpt IS '简介';
COMMENT ON COLUMN "column".price IS '价格';
COMMENT ON COLUMN "column".visit IS '阅读次数';
COMMENT ON COLUMN "column".collect IS '收藏次数';
COMMENT ON COLUMN "column".amount IS '专栏文章数';
COMMENT ON COLUMN "column".complete IS '已发布文章数';
COMMENT ON COLUMN "column".seo_title IS 'SEO标题';
COMMENT ON COLUMN "column".seo_keywords IS 'SEO关键词';
COMMENT ON COLUMN "column".seo_description IS 'SEO描述';
COMMENT ON COLUMN "column".create_id IS '创建者ID';
COMMENT ON COLUMN "column".create_time IS '创建时间';

