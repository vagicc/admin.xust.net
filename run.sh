#!/bin/bash
# ================================================================
#  用作重启后“自动启动rust的warp框架下网站”   
#  nohup /var/www/admin.xust.net/target/release/admin-xust-net >> /var/www/admin.xust.net/run.txt &          
#                                                ---临来笑笑生
#  作者：临来笑笑生                              create:2024.06.03
# ================================================================

### BEGIN INIT INFO
# Provides:          LNMP.SH
# Required-Start:    $remote_fs $syslog
# Required-Stop:     $remote_fs $syslog
# Default-Start:     2 3 4 5
# Default-Stop:      0 1 6
# Short-Description: 启动rust的warp框架下网站
# Description:       本人比较懒，就不写了
### END INIT INFO

PATH='/var/www/admin.xust.net'
RUN_SITE='admin-xust-net';

# PWD=/var/www/admin.xust.net
# export PWD

echo -e "\033[35m 现在开始启动站点${RUN_SITE} \033[0m" 

cd ${PATH}
${PATH}/target/release/${RUN_SITE} >> ${PATH}/run.txt