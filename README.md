# admin.xust.net
# 这www.xust.net的后台管理

运行项目前，请先把“env”文件改为“.env”并查看里面的配置对应本机配置

2024.03.21
cargo new admin-xust-net

下面这本书还是要看完的
Rust语言圣经(Rust Course) https://course.rs/about-book.html


# 应该先抓书：“中华典藏” ：https://www.zhonghuadiancang.com/
做已抓取到书的列表，再做点预览，点修改-发布

# 再做后台可用“布丁扫描”上传书。
# 发布站点。
# 把elapse.date的文章迁过来

# 正确的Rust机器学习框架：Candle、Burn、DFDX，还是tch-rs？: https://mp.weixin.qq.com/s/NT9QEq226B-w9ErgrG-CIg
Burn应该为最好的选择，开发官网：https://burn.dev/  公司官网：https://tracel.ai/

# 酷美网盘：https://www.kumeiwp.com/  免费音乐

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
先去创建数据库，再执行下行语句
psql -h 107.174.102.197 -U postgres -d xust -f xust.bf.2024.5.14.sql
psql -h 127.0.0.1 -U postgres -d xust -f xust.bf.2024.5.14.sql



rust出错：
run pkg_config fail: Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags openssl`
  The pkg-config command could not be found.

  Most likely, you need to install a pkg-config package for your OS.
  Try `apt install pkg-config`, or `yum install pkg-config`,
  or `pkg install pkg-config`, or `apk add pkgconfig` depending on your distribution.

  If you've already installed it, ensure the pkg-config command is one of the
  directories in the PATH environment variable.

  If you did not expect this build to link to a pre-installed system library,
  then check documentation of the openssl-sys crate for an option to
  build the library from source, or disable features or dependencies
  that require pkg-config.

  --- stderr
  thread 'main' panicked at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/openssl-sys-0.9.102/build/find_normal.rs:190:5:


  Could not find directory of OpenSSL installation, and this `-sys` crate cannot
  proceed without this knowledge. If OpenSSL is installed and this crate had
  trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
  compilation process.

  Make sure you also have the development packages of openssl installed.
  For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.

  If you're in a situation where you think the directory *should* be found
  automatically, please open a bug at https://github.com/sfackler/rust-openssl
  and include information about your system as well as this message.

  $HOST = x86_64-unknown-linux-gnu
  $TARGET = x86_64-unknown-linux-gnu
  openssl-sys = 0.9.102


  It looks like you're compiling on Linux and also targeting Linux. Currently this
  requires the `pkg-config` utility to find OpenSSL but unfortunately `pkg-config`
  could not be found. If you have OpenSSL installed you can likely fix this by
  installing `pkg-config`.


  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
root@racknerd-e67ec6:/var/www/admin.xust.net# 

