

drop table if exists `phoenix_exc_orderinfo`;
CREATE TABLE `phoenix_exc_orderinfo` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `order_id` bigint(4) NOT NULL DEFAULT '0' COMMENT '委托id（唯一）',
  `stock_id` varchar(45) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '证券key',
  `stock_code` varchar(45) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '证券代码',
  `exchange_code` varchar(20) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '0' COMMENT '市场代码',
  `order_direction` int(4) NOT NULL DEFAULT '0' COMMENT '委托方向 1:买， 2：卖',
  `order_amount` int(10) NOT NULL DEFAULT '0' COMMENT '委托数量',
  `order_amount_dealed` int(10) NOT NULL default '0' COMMENT '成交量',
  `order_amount_cancel` int(10) NOT NULL default '0' COMMENT '撤单量',
  `price_type` int(4) NOT NULL DEFAULT '0' COMMENT '价格类型',
  `order_price` varchar(20) NOT NULL DEFAULT '0' COMMENT '订单价格',
  `currency_no` varchar(20) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '币种',
  `channel_type` int(4) NOT NULL DEFAULT '0' COMMENT '通道类型 1:实盘,2:内盘',
  `channel_id` int(10) NOT NULL DEFAULT '0' COMMENT '通道ID',
  `confirm_no` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '委托确认号',
  `relate_id` int(10) NOT NULL COMMENT '关联委托ID',
  `receive_date` int(20) NOT NULL COMMENT '接收日期',
  `ex_status` int(4) NOT NULL DEFAULT '0' COMMENT '状态 0: 未知 1：未处理  2：已处理 3：撤单',
  `created_at` bigint(20) NOT NULL COMMENT '创建时间',
  `updated_at` bigint(20) NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`),
  unique key `unique_exorder_id_idx` (`ex_order_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='收到的订单信息表';

drop table if exists `phoenix_exc_dealdetail`;
CREATE TABLE `phoenix_exc_dealdetail` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `order_id` BIGINT(20) NOT NULL COMMENT '委托id',
  `stock_code` VARCHAR(45) NOT NULL COMMENT '证券代码',
  `deal_amount` bigint(20) NOT NULL COMMENT '成交数量',
  `deal_price` varchar(20) NOT NULL COMMENT '成交价格',
  `deal_direction` int(10)	NOT NULL DEFAULT 0 COMMENT '买卖方向 1：买 2：卖',
  `deal_time` bigint(20) NOT NULL COMMENT '成交时间戳',
  `deal_no` nvarchar(50)	not null default '' comment '成交确认号',
  `created_at` bigint(20) NOT NULL COMMENT '创建时间',
  PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT = '成交明细表';
