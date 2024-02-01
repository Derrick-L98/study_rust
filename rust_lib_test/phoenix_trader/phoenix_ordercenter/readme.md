# 订单中心

数据库操作:
使用如下命令生成表实体:
<!-- sea-orm-cli generate entity -u mysql://root:password@localhost:3306/bakeries_db -o src/entities -->

sea-orm-cli generate entity -u mysql://companytest:ZNvC5NoLJNnO3Wn0@uat-stock-data.chinaeast2.cloudapp.chinacloudapi.cn:13301/phoenix_stock -o src/sql


-u / --database-url：数据库 URL（默认值：在 ENV 中指定DATABASE_URL）
-s / --database-schema：数据库架构（默认值：DATABASE_SCHEMA ENV 中指定）
对于MySQL和SQLite，这个参数被忽略了。
对于PostgreSQL，此参数是可选的，默认值为“public”
-o / --output-dir：实体文件输出目录（默认：当前目录）
-v / --verbose：打印调试消息
-l / --lib：生成索引文件代替lib.rsmod.rs
--include-hidden-tables：从隐藏表生成实体文件（默认情况下，名称以下划线开头的表将被隐藏并忽略）
--ignore-tables：跳过为指定表生成实体文件（默认：seaql_migrations)
--compact-format：生成紧凑格式的实体文件（默认值：true）
--expanded-format：生成扩展格式的实体文件
--with-serde：自动派生 serde 序列化/反序列化实体 （、） 的特征（默认：noneserializedeserializebothnone)
--date-time-crate：用于生成实体的日期时间箱 （， ） （默认值：chronotimechrono)
--max-connections：连接池中要初始化的最大数据库连接数（默认值：1)


sea-orm-cli generate entity -u mysql://companytest:ZNvC5NoLJNnO3Wn0@uat-stock-data.chinaeast2.cloudapp.chinacloudapi.cn:13301/phoenix_stock --include-hidden-tables phoenix_oms_feeset -o src/dataservice

snowflake库


数据库相关库：
    MySql: sea-orm
    Cassandra: scylla

sudo systemctl restart phoenix_streamcenter
sudo systemctl restart phoenix_streamcenterHk.service
sudo systemctl restart phoenix_tickcenter
sudo systemctl restart phoenix_tickcenterHk.service
sudo systemctl restart phoenix_hqcalccenter
sudo systemctl restart phoenix_hqcalccenterHk.service
sudo systemctl restart phoenix_hqcenter.service


1、买挂单  203：预买数量。 105：临时冻结（order传参）
2、买成交  102：费用减少，201：持仓增加（order传参），106：临时解冻 ,        预买减少，A股冻结：资产服务自己处理，
3、买撤单  203：预买数量减少  ，106：临时解冻
4、卖挂单  203：预卖数量增加，                                               持仓临时冻结增加，资产服务自己处理，
5、卖成交  101：实盈，102：费用，202：持仓减少                               预卖减少，持仓临时冻结减少
6、卖撤单  203：预卖减少  



sudo systemctl stop SvrStockFtBp.service 
sudo systemctl stop SvrStockYrBp.service 
sudo systemctl stop SvrStockYhBp.service 
sudo systemctl stop SvrStockFIX.service 
sudo systemctl stop SvrStockBpSer.service 