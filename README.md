# admin.xust.net
# 这www.xust.net的后台管理
# 做完书籍，能发布文章，能发布书，(就学Flutter,AI,Rust的無人機開發，嵌入式开发）

AI框架:
Burn应该为最好的选择，开发官网：https://burn.dev/  公司官网：https://tracel.ai/

运行项目前，请先把“env”文件改为“.env”并查看里面的配置对应本机配置

2024.03.21
cargo new admin-xust-net

下面这本书还是要看完的

# Rust電子書籍：
Rust语言圣经(Rust Course) https://course.rs/about-book.html
Discovery：https://jzow.github.io/discovery/
Discovery（Rust探索微控制器）：https://jzow.github.io/discovery/microbit/
嵌入式Rust https://xxchang.github.io/book/

# 书看到：
   https://jzow.github.io/discovery/microbit/05-led-roulette/flash-it.html
# 功能開發計劃

# 应该先抓书：“中华典藏” ：https://www.zhonghuadiancang.com/
做已抓取到书的列表，再做点预览，点修改-发布
章节一章发布
书籍发布……
# 再做后台可用“布丁扫描”上传书。 

# 发布站点。
# 把elapse.date的文章迁过来,


# 正确的Rust机器学习框架：Candle、Burn、DFDX，还是tch-rs？: https://mp.weixin.qq.com/s/NT9QEq226B-w9ErgrG-CIg
Burn应该为最好的选择，开发官网：https://burn.dev/  公司官网：https://tracel.ai/

# 酷美网盘：https://www.kumeiwp.com/  免费音乐

# Rust的無人機開發：https://www.drone-os.com/    GIT:https://github.com/drone-os
# Rust語言開發的電子郵件：Stalwart https://stalw.art/  GIT：https://github.com/stalwartlabs/mail-server


[优先选择：Zorin-OS-17.1-Education ](https://zorin.com/) 

如何训练cloudflare的Workers AI来写文章？

秘塔AI搜索： https://metaso.cn/  （比百度好）
初创AI 
KimiChat  官网：https://kimi.moonshot.cn/

网站模板：https://www.free-css.com/free-css-templates https://templated.co  https://templatemo.com  https://html5up.net/ https://bootstrapmade.com/

单纯只备份数据库的数据，不备份表结构：
下面这条备份语句排除表__diesel_schema_migrations、admins、roles menus的数据备份
pg_dump -h 127.0.0.1 -U postgres -a -O --inserts -d xust -T __diesel_schema_migrations -T admins -T roles -T menus -f xust.bf.2024.5.14.sql
恢复：
先去创建数据库，再执行下行语句恢复数据库
psql -h 107.174.102.197 -U postgres -d xust -f xust.bf.2024.5.14.sql
psql -h 127.0.0.1 -U postgres -d xust -f xust.bf.2024.5.14.sql


while true; do ssh -D 5858 root@107.174.102.197 || true; done
 
查看本地（Ubuntu）的货币格式设置：
在本地 psql 中运行：SHOW lc_monetary;
postgres=# SHOW lc_monetary;

 lc_monetary 

-------------

 en_US.UTF-8

(1 row)



postgres=# 
假设本地返回的是 en_US.UTF-8 或 zh_CN.UTF-8。

备份本机数据：
pg_dump -U postgres -h localhost -p 5432 -d 你的数据库名 -F p -f /tmp/你的数据库名_plain.sql

sed -i "3i SET lc_monetary = 'en_US.UTF-8';" /tmp/你的数据库名_plain.sql


上传后：

su - postgres
psql
CREATE DATABASE 你的数据库名;
\q;

一键执行数据导入:

psql -d 你的数据库名 -f /tmp/你的数据库名_plain.sql

数据库性能黄金收尾（更新统计信息）：
导入完成后，再次进入该数据库：
psql -d 你的数据库名
执行垃圾回收与统计分析，让 PG 18 优化器满血复活：
VACUUM ANALYZE;
\q;


检查命令（全局查找 money 类型）
SELECT 
    table_schema AS 架构名,
    table_name AS 表名,
    column_name AS 列名,
    data_type AS 数据类型
FROM 
    information_schema.columns
WHERE 
    data_type = 'money'
    AND table_schema NOT IN ('pg_catalog', 'information_schema');

   
pg_dump -U postgres -h localhost -p 5432 -d xust -F p -f /tmp/xust_plain.sql
sed -i "3i SET lc_monetary = 'en_US.UTF-8';" /tmp/xust_plain.sql

scp /tmp/xust_plain.sql 远程用户名@服务器IP:/tmp/
服务器：
psql -c "CREATE DATABASE xust;"
psql -d xust -f /tmp/xust_plain.sql
psql -d xust -c "VACUUM ANALYZE;"

pg_dump -U postgres -h localhost -p 5432 -d elapse -F p -f /tmp/elapse_plain.sql
sed -i "3i SET lc_monetary = 'en_US.UTF-8';" /tmp/elapse_plain.sql

服务器：
psql -c "CREATE DATABASE elapse;"
psql -d elapse -f /tmp/elapse_plain.sql
psql -d elapse -c "VACUUM ANALYZE;"

pg_dump -U postgres -h localhost -p 5432 -d admin -F p -f /tmp/admin_plain.sql

服务器：
psql -c "CREATE DATABASE admin;"
psql -d admin -f /tmp/admin_plain.sql
psql -d admin -c "VACUUM ANALYZE;"


pg_dump -U postgres -h localhost -p 5432 -d manage -F p -f /tmp/manage_plain.sql

服务器：
psql -c "CREATE DATABASE manage;"
psql -d manage -f /tmp/manage_plain.sql
psql -d manage -c "VACUUM ANALYZE;"


pg_dump -U postgres -h localhost -p 5432 -d xueyigou -F p -f /tmp/xueyigou_plain.sql
sed -i "3i SET lc_monetary = 'en_US.UTF-8';" /tmp/xueyigou_plain.sql

scp /tmp/xueyigou_plain.sql 远程用户名@服务器IP:/tmp/
服务器：
psql -c "CREATE DATABASE xueyigou;"
psql -d xueyigou -f /tmp/xueyigou_plain.sql
psql -d xueyigou -c "VACUUM ANALYZE;"


把rust的项目做到服务：
vim /etc/systemd/system/www.gwmc.site.service
[Unit]
Description=Warp Site: www.gwmc.site
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/var/www/www-gwmc-site
ExecStart=/var/www/www-gwmc-site/target/release/www-gwmc-site 
Restart=on-failure
RestartSec=5
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target

然后：
# 1. 重载 systemd 配置（告诉系统有新服务）
sudo systemctl daemon-reload

# 2. 启动服务
sudo systemctl start www.gwmc.site

# 3. 设置开机自启（下次重启后自动启动）
sudo systemctl enable www.gwmc.site

# 4. 查看状态（确认是否正常运行）
sudo systemctl status www.gwmc.site

