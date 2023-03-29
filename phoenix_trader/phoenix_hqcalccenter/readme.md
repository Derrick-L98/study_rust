注意：缓存包括内存缓存和落地缓存（KV 存储）

tick 处理:
1）缓存最新的一个点的数据，缓存一段时间的数据？？？？
2）上一个点的数据，发送到 cassandra，如果成交量为 0，则不保存到 cassandra

分时：
缓存：每分钟完整的数据（全部），未计算完的 1 分钟
k 线：完整的 k 线数据（全部），不完整的 k 线数据

-- 只要计时器是准点触发即可
先启动每秒计时器

1）判断是否交易时间

- 1）如果是交易时间，则开始计算当前一分钟的分时，k 线 等 数据
- 2）非交易时间，则不处理
  2）判断当前时间是否准点
- 1）是准点，则触发分钟计时器，并关闭每秒计时器
- 2）非准点，不处理


--- #########################################

收到行情以后，需要处理得事情 
1)接收tick ，判断是否交易时间，如果非交易时间，则丢弃
2)计算 tick(了解计算内容)
2.1)实时发送 tick 到 mq
2.2)保存到 Cassandra

两个缓存
1）缓存所有的最新 tick 数据
2）缓存 1 个小时（60 分钟）的 1分钟k线 数据(缓存数据落地，用什么方案？？？)


1)启动一个3秒钟的定时器，检查tick数据，必要时补发数据(成交量为0,最新价用上一个点的价格)

如何计算分时：
1)tick驱动，如果数据延时,则重新计算分时数据

如何计算1分钟k线
1）tick驱动
2)如果数据延时,则重新计算分时数据
3)缓存60个1分钟k线
用一个变量累计

num++;

if num/5 == 0 //计算5分钟
if num/10 == 0 //10分钟
num/30，
num/60

如何计算5分钟，10分钟，30分钟，60分钟k线？