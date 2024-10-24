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

 
