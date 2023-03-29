/*
 Navicat Premium Data Transfer

 Source Server         : uat-stock-data.chinaeast2.cloudapp.chinacloudapi.cn
 Source Server Type    : MySQL
 Source Server Version : 50734
 Source Host           : uat-stock-data.chinaeast2.cloudapp.chinacloudapi.cn:13301
 Source Schema         : phoenix_stock

 Target Server Type    : MySQL
 Target Server Version : 50734
 File Encoding         : 65001

 Date: 20/10/2022 14:43:02
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for phoenix_ord_suborder
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_ord_suborder`;
CREATE TABLE `phoenix_ord_suborder`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `sys_date` int(11) NOT NULL COMMENT '业务日期',
  `order_no` bigint(20) NOT NULL COMMENT '委托号',
  `unit_id` bigint(20) NOT NULL COMMENT 'unitid',
  `channel_id` int(11) NOT NULL COMMENT '通道id',
  `channel_type` int(1) NOT NULL DEFAULT 0 COMMENT '通道类别: 1.外盘, 2:内盘',
  `order_amount` int(11) NOT NULL COMMENT '委托数量',
  `order_price` decimal(16, 4) NOT NULL COMMENT '委托价',
  `price_type` int(11) DEFAULT 0 COMMENT '价格类型',
  `confirm_no` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT '' COMMENT '柜台返回订单号',
  `order_status` int(4) DEFAULT 1 COMMENT '委托状态，同委托表委托状态',
  `deal_value` decimal(16, 4) DEFAULT 0.0000 COMMENT '成交金额',
  `deal_amount` int(11) DEFAULT 0 COMMENT '成交数量',
  `modify_time` bigint(20) NOT NULL DEFAULT 0 COMMENT '成交时间',
  `create_time` bigint(20) NOT NULL DEFAULT 0 COMMENT '时间戳',
  `cancel_flag` int(1) NOT NULL DEFAULT 0 COMMENT '撤单标记：1：待撤',
  `remark` varchar(1024) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT '',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `channel_order`(`order_no`, `channel_id`, `channel_type`) USING BTREE,
  INDEX `confirm_no`(`confirm_no`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci COMMENT = '子订单 - 分单信息表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for phoenix_ord_stockorder
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_ord_stockorder`;
CREATE TABLE `phoenix_ord_stockorder`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `order_no` bigint(20) NOT NULL COMMENT '委托序号',
  `sys_date` int(8) NOT NULL DEFAULT 0,
  `unit_id` bigint(20) NOT NULL COMMENT '用户id',
  `stock_id` bigint(20) NOT NULL COMMENT '证券id',
  `stock_code` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '证券代码',
  `exchange_id` int(11) NOT NULL COMMENT '市场id',
  `order_direction` int(11) NOT NULL COMMENT '1:买，2：卖',
  `order_price` decimal(16, 8) NOT NULL COMMENT '委托价格',
  `order_amount` int(16) NOT NULL COMMENT '委托数量',
  `order_type` int(5) NOT NULL COMMENT '委托类别 1:pc 2:app ',
  `pre_fee` decimal(12, 2) NOT NULL COMMENT '预估交易费用',
  `pre_capital` decimal(16, 4) NOT NULL COMMENT '预买或者预卖金额',
  `price_type` int(11) NOT NULL DEFAULT 0 COMMENT '价格类型1 市价 2 限价',
  `deal_amount` int(11) NOT NULL DEFAULT 0 COMMENT '成交数量',
  `deal_value` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '成交金额',
  `deal_fee` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '真实成交费用',
  `cancel_amount` int(11) NOT NULL DEFAULT 0 COMMENT '撤单数量',
  `order_status` int(4) NOT NULL DEFAULT 0 COMMENT '订单状态 1: 2:',
  `first_deal_time` bigint(20) NOT NULL DEFAULT 0 COMMENT '首次成交时间',
  `relate_order` bigint(20) DEFAULT 0 COMMENT '关联委托,反向单的订单号',
  `trade_type` int(4) NOT NULL DEFAULT 1 COMMENT '交易类型：1:股票买卖， 2:融券交易',
  `order_memo` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT '' COMMENT '订单说明，可以是废单原因...',
  `create_time` bigint(20) NOT NULL COMMENT '订单生成时间',
  `modify_time` bigint(20) NOT NULL COMMENT '修改时间',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `idx_order_no`(`order_no`) USING BTREE,
  INDEX `idx_unit_id`(`unit_id`) USING BTREE,
  INDEX `idx_sys_date`(`sys_date`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci COMMENT = '委托表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for phoenix_ord_stockdeal
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_ord_stockdeal`;
CREATE TABLE `phoenix_ord_stockdeal`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `deal_no` bigint(20) NOT NULL,
  `sys_date` int(11) NOT NULL DEFAULT 0,
  `order_no` bigint(20) NOT NULL DEFAULT 0,
  `exchange_id` int(11) NOT NULL,
  `unit_id` bigint(20) NOT NULL,
  `stock_id` bigint(20) NOT NULL,
  `stock_code` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `order_direction` int(11) NOT NULL,
  `deal_time` int(11) NOT NULL,
  `deal_amount` int(11) NOT NULL,
  `deal_price` decimal(16, 8) NOT NULL,
  `fee_jy` decimal(10, 2) DEFAULT 0.00 COMMENT '交易费',
  `fee_gh` decimal(10, 2) DEFAULT 0.00 COMMENT '过户费',
  `fee_yj` decimal(10, 2) DEFAULT 0.00 COMMENT '佣金',
  `fee_js` decimal(10, 2) DEFAULT 0.00 COMMENT '经手费',
  `fee_zg` decimal(10, 2) DEFAULT 0.00 COMMENT '证管费',
  `fee_qt` decimal(10, 2) DEFAULT 0.00 COMMENT '其他费用',
  `fee_js2` decimal(10, 2) DEFAULT 0.00 COMMENT '结算费',
  `fee_jg` decimal(10, 2) DEFAULT 0.00 COMMENT '交割费',
  `fee_yh` decimal(10, 2) DEFAULT 0.00 COMMENT '印花税',
  `fee_real_yj` decimal(10, 2) DEFAULT 0.00,
  `fee_total` decimal(10, 2) DEFAULT 0.00,
  `exec_no` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT '' COMMENT '柜台返回的成交编号',
  `channel_type` int(11) DEFAULT 1 COMMENT '通道类型',
  `channel_id` int(11) DEFAULT 0 COMMENT '通道id',
  `refer_profit` decimal(16, 4) DEFAULT 0.0000 COMMENT '股票卖出成交时的参考收益,以成交时持仓均价计算, 币种为交易币种',
  `trade_type` int(4) NOT NULL DEFAULT 1 COMMENT '交易类型：1:股票买卖， 2:融券交易',
  `create_time` bigint(20) NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `idx_deal_no`(`deal_no`) USING BTREE,
  INDEX `idx_l_unit_id`(`unit_id`) USING BTREE,
  INDEX `idx_order_no`(`order_no`) USING BTREE,
  INDEX `idx_sys_date`(`sys_date`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci COMMENT = '成交表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for phoenix_ord_settledetail
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_ord_settledetail`;
CREATE TABLE `phoenix_ord_settledetail`(
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '序号',
  `sys_date` int(8) NOT NULL COMMENT '日期',
  `settle_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '交收id->对应交收表',
  `unit_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '用户id',
  `stock_code` varchar(20) NOT NULL DEFAULT '' COMMENT '证券代码',
  `order_direction` int(11) NOT NULL COMMENT '委托方向',
  `order_amount` int(11) NOT NULL COMMENT '委托数量',
  `deal_price` decimal(14, 8) NOT NULL COMMENT '成交均价',
  `currency_rate` decimal(10, 6) NOT NULL COMMENT '交收汇率',
  `real_profit` decimal(10, 6) NOT NULL COMMENT '实际盈亏',
  `settle_amount` int(11) NOT NULL COMMENT '交收数量',
  `settle_internal_amount` int(11) NOT NULL COMMENT '交收内部通道数量，主要用于计算费用',
  `settle_balance` decimal(18, 2) NOT NULL COMMENT '交收金额 实际发生的金额，如何计算？？？（成交价格*交收数量+-费用）*汇率',
  `fee_total` decimal(12, 2) NOT NULL COMMENT '总费用-实际费用',
  `fee_jy` decimal(12, 2) NOT NULL,
  `fee_yh` decimal(12, 2) NOT NULL,
  `fee_gh` decimal(12, 2) NOT NULL,
  `fee_yj` decimal(12, 2) NOT NULL,
  `fee_js` decimal(12, 2) NOT NULL,
  `fee_zg` decimal(12, 2) NOT NULL,
  `fee_other` decimal(12, 2) NOT NULL,
  `fee_js2` decimal(12, 2) NOT NULL,
  `fee_jg` decimal(12, 2) NOT NULL,
  `settle_type` int(11) NOT NULL DEFAULT 0 COMMENT '0:全部 1:资金 2:持仓',
  `currency_no` varchar(3) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '交易币种',
  `remark` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT '',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `idx_settle_detail_type_no`(`settle_id`,`settle_type`) USING BTREE,
  INDEX `idx_settle_detail_id`(`settle_id`) USING BTREE,
  INDEX `idx_sys_settle_date`(`sys_date`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci COMMENT = '交收表明细' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for phoenix_ord_pendsettle
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_ord_pendsettle`;
CREATE TABLE `phoenix_ord_pendsettle`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '序号',
  `settle_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '交收ID',
  `unit_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '用户id',
  `sys_date` int(8) NOT NULL COMMENT '日期',
  `trade_date` int(8) NOT NULL COMMENT '成交日期',
  `settle_date` int(8) NOT NULL COMMENT '交割日期',
  `clear_speed` int(11) NOT NULL COMMENT '交收阶段，结算时减去1',
  `order_no` bigint(20) NOT NULL COMMENT '委托编号',
  `order_amount` int(11) NOT NULL COMMENT '委托数量',
  `deal_avg_price` decimal(14, 8) DEFAULT 0.00000000 COMMENT '成交均价',
  `exchange_id` int(11) NOT NULL COMMENT '市场id',
  `stock_id` bigint(20) NOT NULL COMMENT '证券id',
  `order_direction` int(11) NOT NULL COMMENT '委托方向',
  `settle_amount` int(11) NOT NULL COMMENT '交收数量',
  `settle_internal_amount` int(11) NOT NULL COMMENT '交收内部通道数量，主要用于计算费用',
  `settle_balance` decimal(18, 2) NOT NULL COMMENT '交收金额',
  `net_balance` decimal(18, 2) NOT NULL COMMENT '交收金额(不含费用)',
  `fee_total` decimal(12, 2) NOT NULL COMMENT '总费用',
  `fee_jy` decimal(12, 2) NOT NULL,
  `fee_yh` decimal(12, 2) NOT NULL,
  `fee_gh` decimal(12, 2) NOT NULL,
  `fee_yj` decimal(12, 2) NOT NULL,
  `fee_js` decimal(12, 2) NOT NULL,
  `fee_zg` decimal(12, 2) NOT NULL,
  `fee_other` decimal(12, 2) NOT NULL,
  `fee_js2` decimal(12, 2) DEFAULT 0.00,
  `fee_jg` decimal(12, 2) DEFAULT 0.00,
  `asset_settle_date` int(11) NOT NULL COMMENT '交收资金的日期',
  `p_status` int(11) NOT NULL COMMENT '状态(资金交收) 0:未处理 1:已经理',
  `currency_no` varchar(3) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '交易币种',
  `remark` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT '',
  `currency_rate` decimal(10, 6) NOT NULL COMMENT '成交时汇率',
  `channel_id` int(11) NOT NULL DEFAULT 0 COMMENT '通道id',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `idx_order_no`(`order_no`, `channel_id`) USING BTREE,
  INDEX `unit_id`(`unit_id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci COMMENT = '交收表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for phoenix_oms_assetflow
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_oms_assetflow`;
CREATE TABLE `phoenix_oms_assetflow`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `sys_date` int(11) NOT NULL DEFAULT 0 COMMENT '日期',
  `unit_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '用户id',
  `busin_flag` int(11) NOT NULL DEFAULT 0 COMMENT '业务标志',
  `occur_capital` decimal(16, 2) NOT NULL DEFAULT 0.00 COMMENT '发生金额',
  `post_capital` decimal(16, 2) NOT NULL DEFAULT 0.00 COMMENT '发生后金额',
  `currency_rate` decimal(16, 2) NOT NULL DEFAULT 0.00 COMMENT '当时汇率',
  `stock_code` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '证券代码',
  `stock_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '证券id',
  `remarks` varchar(1024) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '',
  `currency_no` varchar(3) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'HKD' COMMENT '币种',
  `deal_no` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '成交编号/交收序号',
  `datetime` bigint(20) NOT NULL DEFAULT 0 COMMENT '时间戳',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci COMMENT = '资金流水表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for phoenix_ast_unitasset_his
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_ast_unitasset_his`;
CREATE TABLE `phoenix_ast_unitasset_his`  (
  `id` bigint(20) NOT NULL,
  `sys_date` int(11) NOT NULL,
  `unit_id` bigint(20) NOT NULL DEFAULT 0,
  `begin_cash` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '期初本金',
  `current_cash` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '当前本金',
  `frozen_capital` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '冻结资金（提现冻结）',
  `trade_frozen_capital` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '交易临时冻结',
  `cash_in_transit` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '在途资金，交易后首次交收时将未交的资金归结到在途资金',
  `currency_no` varchar(3) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'HKD' COMMENT '币种',
  `credit_multiple` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '信用倍数',
  `enable_cash` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '用于保存收盘时的可用金额',
  `today_total_value` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '当日结算后结存',
  
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `unitid_sysdate_idx`(`unit_id`, `sys_date`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci COMMENT = '资产历史表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for phoenix_ast_unitasset
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_ast_unitasset`;
CREATE TABLE `phoenix_ast_unitasset`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `sys_date` int(11) NOT NULL,
  `unit_id` bigint(20) NOT NULL DEFAULT 0,
  `begin_cash` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '期初本金',
  `current_cash` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '当前本金',
  `frozen_capital` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '冻结资金（提现冻结）',
  `trade_frozen_capital` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '交易临时冻结',
  `cash_in_transit` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '在途资金，交易后首次交收时将未交的资金归结到在途资金',
  `currency_no` varchar(3) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'HKD' COMMENT '币种',
  `credit_multiple` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '信用倍数',
  `enable_cash` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '用于保存收盘时的可用金额',
  `today_total_value` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '当日结算后结存',
  PRIMARY KEY (`id`) USING BTREE,
  INDEX `idx_unit_id`(`unit_id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 2 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci COMMENT = '资产表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for phoenix_ast_stockposition
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_ast_stockposition`;
CREATE TABLE `phoenix_ast_stockposition`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `sys_date` int(11) NOT NULL,
  `unit_id` bigint(20) NOT NULL,
  `stock_code` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '',
  `stock_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '证券id',
  `exchange_id` int(11) NOT NULL COMMENT '市场id',
  `position_flag` int(11) NOT NULL DEFAULT 1 COMMENT '1多 2空',
  `begin_amount` int(11) NOT NULL DEFAULT 0 COMMENT '期初数量',
  `current_amount` int(11) NOT NULL DEFAULT 0 COMMENT '当前数量',
  `frozen_amount` int(11) NOT NULL DEFAULT 0 COMMENT '冻结数量',
  `temp_frozen_amount` int(11) NOT NULL DEFAULT 0 COMMENT '临时冻结数量',
  `buy_amount` int(11) NOT NULL DEFAULT 0 COMMENT '今买数量',
  `sale_amount` int(11) NOT NULL DEFAULT 0 COMMENT '今卖数量',
  `prebuy_amount` int(11) NOT NULL DEFAULT 0 COMMENT '预买数量',
  `presale_amount` int(11) NOT NULL DEFAULT 0 COMMENT '预卖数量',
  `buy_fee` decimal(12, 2) NOT NULL DEFAULT 0.00 COMMENT '买费用(当天结算前)',
  `sale_fee` decimal(12, 2) NOT NULL DEFAULT 0.00 COMMENT '卖费用',
  `real_buy_fee` decimal(12, 2) NOT NULL DEFAULT 0.00 COMMENT '结算汇总费用',
  `real_sale_fee` decimal(12, 2) NOT NULL DEFAULT 0.00 COMMENT '真实费用',
  `current_cost` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '总成本，买：成交数量*买价格 卖：卖价格*数量 +买金额(含费用)-卖金额',
  `avg_price` decimal(16, 8) NOT NULL DEFAULT 0.00000000 COMMENT '成交均价',
  `total_value` decimal(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '总开仓市值',
  `total_value_hkd` decimal(20, 4) NOT NULL DEFAULT 0.0000 COMMENT '港币结算开仓市值',
  `begin_value_hkd` decimal(20, 4) NOT NULL DEFAULT 0.0000 COMMENT '港币结算每日初始开仓市值',
  `today_total_value_hkd` decimal(20, 4) NOT NULL DEFAULT 0.0000 COMMENT '港币结算开仓市值(结算前)',
  `last_price` decimal(16, 4) NOT NULL DEFAULT 0.0000 COMMENT '最新价',
  `buy_in_transit` int(11) NOT NULL DEFAULT 0 COMMENT '在途持仓数量(买)',
  `sale_in_transit` int(11) NOT NULL DEFAULT 0 COMMENT '在途持仓数量(卖)',
  `channel_id` int(11) NOT NULL DEFAULT 0 COMMENT '通道id',
  `stock_type` int(11) NOT NULL DEFAULT 0 COMMENT '股票类别',
  `margin_rate` decimal(10, 4) NOT NULL DEFAULT 0.0000 COMMENT '保证金比例',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci COMMENT = '持仓表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for phoenix_ast_operation_detail
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_ast_operation_detail`;
CREATE TABLE `phoenix_ast_operation_detail`  (
  `id` int(20) NOT NULL AUTO_INCREMENT,
  `sys_date` int(11) NOT NULL,
  `op_businflag` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '',
  `unit_id` bigint(20) NOT NULL DEFAULT 0 COMMENT '账户',
  `occur_capital` decimal(12, 2) NOT NULL DEFAULT 0.00 COMMENT '金额',
  `remark` varchar(1024) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '',
  `ext_id` int(11) NOT NULL DEFAULT 0 COMMENT '对应的业务ID,如果是冻结，则对应冻结的ID',
  `currency_no` varchar(12) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `operator` int(11) NOT NULL DEFAULT 0 COMMENT '操作人员',
  `datetime` bigint(20) NOT NULL,
  PRIMARY KEY (`id`) USING BTREE,
  INDEX `idx_unit_id`(`unit_id`) USING BTREE,
  INDEX `idx_ext_id`(`ext_id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci COMMENT = '资金操作明细(类似日志信息)' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for phoenix_ast_frozendetail_his
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_ast_frozendetail_his`;
CREATE TABLE `phoenix_ast_frozendetail_his`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT  ,
  `did` bigint(20) NOT NULL   ,
  `sys_date` int(11) NOT NULL,
  `unit_id` bigint(20) NOT NULL COMMENT '用户id',
  `deal_capital` decimal(16, 2) NOT NULL DEFAULT 0.00 COMMENT '冻结金额',
  `deal_date` int(11) NOT NULL DEFAULT 0 COMMENT '冻结日期',
  `cancel_date` int(11) NOT NULL DEFAULT 0 COMMENT '解冻日期',
  `status` int(11) NOT NULL DEFAULT 1 COMMENT '状态，1：有效，0：无效',
  `remarks` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '备注',
  `currency_no` varchar(3) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '币种',
  `busin_flag` int(11) NOT NULL DEFAULT 0 COMMENT '业务类型，暂留',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `idx_frozen_detail_his`(`did`, `sys_date`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci COMMENT = '冻结解冻明细历史表' ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for phoenix_ast_frozendetail
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_ast_frozendetail`;
CREATE TABLE `phoenix_ast_frozendetail`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `sys_date` int(11) NOT NULL,
  `unit_id` bigint(20) NOT NULL COMMENT '用户id',
  `deal_capital` decimal(16, 2) NOT NULL DEFAULT 0.00 COMMENT '冻结金额',
  `deal_date` int(11) NOT NULL DEFAULT 0 COMMENT '冻结日期',
  `cancel_date` int(11) NOT NULL DEFAULT 0 COMMENT '解冻日期',
  `status` int(11) NOT NULL DEFAULT 1 COMMENT '状态，1：有效，0：无效',
  `remarks` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '备注',
  `currency_no` varchar(3) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '币种',
  `busin_flag` int(11) NOT NULL DEFAULT 0 COMMENT '业务类型，暂留',
  PRIMARY KEY (`id`) USING BTREE,
  INDEX `idx_unit_id`(`unit_id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 1 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci COMMENT = '冻结解冻明细表' ROW_FORMAT = Dynamic;

-- ---------------------------------------------
-- 实时风控(账户风控)信息表
-- ---------------------------------------------
drop table if exists `phoenix_account_assets`;
CREATE TABLE `phoenix_account_assets` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `p_account_id` int(10) NOT NULL DEFAULT '0' COMMENT '分帐户id',
  `p_current_principal` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '本金:初始值+资金划拨得值;总账户:融资本金',
  `p_credit_cash` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '总账户:融资信用金',
  `p_current_financial` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '当前本金，总账户:当前本金,分帐户:0',
  `p_position_value` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '持仓市值,包含科创版和创业板, 是否科创版,需要从接口获取股票',
  `p_position_value_star` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '(科创版+创业板)持仓市值,单独计算',
  `p_prebuy_capital_star` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '创业板开仓挂单金额汇总',
  `p_financing_borrowed` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '已借金额',
  `p_financing_occupied` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '保证金占用',
  `p_fee_total_hkd` decimal(20,8) NOT NULL COMMENT '汇总的费用',
  `p_real_profit` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '实际盈亏',
  `p_floating_profit` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '浮动盈亏',
  `p_account_type` int(10) NOT NULL DEFAULT '0' COMMENT '账号类型,比如:2:总账户, 1:主账户,0:分帐户',
  `p_date` int(10) NOT NULL DEFAULT '0' COMMENT '所属日期',
  `p_updatetime` bigint(20) NOT NULL DEFAULT '0' COMMENT '更新时间,时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `unique_channel_account_id` (`p_account_id`) USING BTREE COMMENT '唯一得分帐户id'
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT='运营账户资产信息表';

drop table if exists `phoenix_account_assets_his`;
CREATE TABLE `phoenix_account_assets_his` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `p_account_id` int(10) NOT NULL DEFAULT '0' COMMENT '分帐户id',
  `p_current_principal` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '本金:初始值+资金划拨得值;总账户:融资本金',
  `p_credit_cash` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '总账户:融资信用金',
  `p_current_financial` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '当前本金，总账户:当前本金,分帐户:0',
  `p_position_value` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '持仓市值,包含科创版和创业板, 是否科创版,需要从接口获取股票',
  `p_position_value_star` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '(科创版+创业板)持仓市值,单独计算',
  `p_prebuy_capital_star` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '创业板开仓挂单金额汇总',
  `p_financing_borrowed` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '已借金额',
  `p_financing_occupied` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '保证金占用',
  `p_fee_total_hkd` decimal(20,8) NOT NULL COMMENT '汇总的费用',
  `p_real_profit` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '实际盈亏',
  `p_floating_profit` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '浮动盈亏',
  `p_account_type` int(10) NOT NULL DEFAULT '0' COMMENT '账号类型,比如:2:总账户, 1:主账户,0:分帐户',
  `p_date` int(10) NOT NULL DEFAULT '0' COMMENT '所属日期',
  `p_updatetime` bigint(20) NOT NULL DEFAULT '0' COMMENT '更新时间,时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `unique_channel_account_his` (`p_account_id`,`p_date`) USING BTREE COMMENT '通道号唯一'
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT='历史运营账户资产信息表';

drop table if exists `phoenix_stockposition_channel`;
CREATE TABLE `phoenix_stockposition_channel` (
  `id` bigint(12) NOT NULL AUTO_INCREMENT,
  `p_date` int(10) NOT NULL,
  `p_account_unit_id` int(4) NOT NULL DEFAULT '0' COMMENT '通道对应的账户ID',
  `p_stock_id` int(11) NOT NULL DEFAULT '0' COMMENT '股票id',
  `p_stock_code` varchar(20) NOT NULL DEFAULT '' COMMENT '股票代码',
  `p_exchange_id` int(10) NOT NULL COMMENT '交易市场',
  `p_prebuy_amount` int(10) NOT NULL DEFAULT '0' COMMENT '预买数量',
  `p_current_amount` int(11) NOT NULL DEFAULT '0' COMMENT '当前数量',
  `p_frozen_amount` int(11) NOT NULL DEFAULT 0 COMMENT '冻结数量',
  `p_frozen_amount_temp` int(11) NOT NULL DEFAULT 0 COMMENT '临时冻结数量',
  `p_avg_price` decimal(16,8) NOT NULL DEFAULT '0.00000000' COMMENT '持仓均价(持仓成本/数量)',
  `p_avg_price_hkd` decimal(16,8) NOT NULL DEFAULT '0.00000000' COMMENT '持仓均价(持仓成本/数量) HKD',
  `p_total_value` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '总市值(累加(成交的成交价*数量))',
  `p_total_value_hkd` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '总市值(累加(成交的成交价*数量)) HKD',
  `p_last_price` decimal(16,4) NOT NULL DEFAULT '0.0000' COMMENT '最新价',
  `p_fee_hkd` decimal(20,8) NOT NULL COMMENT '汇总的费用',
  `p_channel_id` int(10) NOT NULL DEFAULT '0' COMMENT '通道id',
  `p_qfii_state` int(4) NOT NULL DEFAULT 0 COMMENT 'qfii状态',
  `p_stock_type` int(4) NOT NULL DEFAULT '0' COMMENT '股票类型',
  `p_currency_rate` decimal(28,6) NOT NULL COMMENT '汇率',
  `p_leverage` decimal(10,4) NOT NULL DEFAULT '3.0000' COMMENT '保证金比率(保证金比率一旦发生变化,则需要重新计算),用于计算保证金等数据',
  PRIMARY KEY (`id`),
  UNIQUE KEY `unique_stockcode` (`p_account_unit_id`,`p_stock_id`,`p_channel_id`) USING BTREE,
  KEY `p_account_unit_id` (`p_account_unit_id`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT='分账户持仓信息表';

drop table if exists `phoenix_stockposition_channel_his`;
CREATE TABLE `phoenix_stockposition_channel_his` (
  `id` bigint(12) NOT NULL AUTO_INCREMENT,
  `p_date` int(10) NOT NULL DEFAULT '0' COMMENT '所属日期',
  `p_account_unit_id` int(4) NOT NULL DEFAULT '0' COMMENT '通道对应的账户ID',
  `p_stock_id` int(11) NOT NULL DEFAULT '0' COMMENT '股票id',
  `p_stock_code` varchar(20) NOT NULL DEFAULT '' COMMENT '股票代码',
  `p_exchange_id` int(10) NOT NULL COMMENT '交易市场',
  `p_prebuy_amount` int(10) NOT NULL DEFAULT '0' COMMENT '预买数量',
  `p_current_amount` int(11) NOT NULL DEFAULT '0' COMMENT '当前数量',
  `p_frozen_amount` int(11) NOT NULL DEFAULT 0 COMMENT '冻结数量',
  `p_frozen_amount_temp` int(11) NOT NULL DEFAULT 0 COMMENT '临时冻结数量',
  `p_avg_price` decimal(16,8) NOT NULL DEFAULT '0.00000000' COMMENT '持仓均价(持仓成本/数量)',
  `p_avg_price_hkd` decimal(16,8) NOT NULL DEFAULT '0.00000000' COMMENT '持仓均价(持仓成本/数量) HKD',
  `p_total_value` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '总市值(累加(成交的成交价*数量))',
  `p_total_value_hkd` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '总市值(累加(成交的成交价*数量)) HKD',
  `p_last_price` decimal(16,4) NOT NULL DEFAULT '0.0000' COMMENT '最新价',
  `p_fee_hkd` decimal(20,8) NOT NULL COMMENT '汇总的费用',
  `p_channel_id` int(10) NOT NULL DEFAULT '0' COMMENT '通道id',
  `p_qfii_state` int(4) NOT NULL DEFAULT 0 COMMENT 'qfii状态',
  `p_stock_type` int(4) NOT NULL DEFAULT '0' COMMENT '股票类型',
  `p_currency_rate` decimal(28,6) NOT NULL COMMENT '汇率',
  `p_leverage` decimal(10,4) NOT NULL DEFAULT '3.0000' COMMENT '该品种在该通道的杠杆倍数,用于计算保证金等数据',
  PRIMARY KEY (`id`),
  UNIQUE KEY `unique_stockcode` (`p_account_unit_id`,`p_stock_id`,`p_channel_id`,`p_date`) USING BTREE,
  KEY `p_account_unit_id` (`p_account_unit_id`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT='分账户持仓表-历史记录';

drop table if exists `phoenix_deal_detail`;
CREATE TABLE `phoenix_deal_detail` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `p_date` int(10) NOT NULL DEFAULT '0' COMMENT '所属日期, 格式:20210101,每个日期,只有一条数据,收到结算指令时计算产生',
  `p_order_no` bigint(20) NOT NULL DEFAULT '0' COMMENT '订单号码',
  `p_exchange_id` bigint(20) NOT NULL DEFAULT '0' COMMENT '交易所的ID',
  `p_stock_id` bigint(20) NOT NULL DEFAULT '0' COMMENT '股票ID',
  `p_stock_code` varchar(12) NOT NULL DEFAULT '0' COMMENT '股票代码',
  `p_order_direction` int(4) NOT NULL DEFAULT 1 COMMENT '买卖方向 1:买 2:卖',
  `p_deal_amount` int(10) NOT NULL DEFAULT '0' COMMENT '成交数量',
  `p_prebuy_amount` int(10) NOT NULL DEFAULT '0' COMMENT '预买数量',
  `p_deal_price` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '成交价格',
  `p_fee_total` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '总费用',
  `p_currency_rate` decimal(20,8) NOT NULL DEFAULT '1.00000000' COMMENT '参考汇率,计算浮盈,总市值时使用',
  `p_currency_rate_cj` decimal(20,8) NOT NULL DEFAULT '1.00000000' COMMENT '成交汇率,计算实际盈亏时使用,结算时用结算汇率重算',
  `p_unit_id` bigint(20) NOT NULL DEFAULT '0' COMMENT '用户资产ID',
  `p_channel_type` int(10) NOT NULL DEFAULT '0' COMMENT '通道类型 1:外盘 2:内盘',
  `p_channel_id` bigint(20) NOT NULL DEFAULT '0' COMMENT '通道ID',
  `p_qfii_state` int(4) NOT NULL DEFAULT 0 COMMENT 'qfii状态',
  `p_account_id` bigint(20) NOT NULL DEFAULT '0' COMMENT '通道对应的分帐户ID',
  `p_trade_type` int(10) NOT NULL DEFAULT '0' COMMENT '交易类型 1:普通股票交易, 2:融券交易',
  `p_stock_type` int(10) NOT NULL DEFAULT '0' COMMENT '股票类型',
  `p_leverage` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '品种杠杆比率',
  `p_updatetime` bigint(20) NOT NULL DEFAULT '0' COMMENT '更新时间,时间戳',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT='成交明细信息表';

drop table if exists `phoenix_deal_detail_his`;
CREATE TABLE `phoenix_deal_detail_his` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `p_date` int(10) NOT NULL DEFAULT '0' COMMENT '所属日期, 格式:20210101,每个日期,只有一条数据,收到结算指令时计算产生',
  `p_order_no` bigint(20) NOT NULL DEFAULT '0' COMMENT '订单号码',
  `p_exchange_id` bigint(20) NOT NULL DEFAULT '0' COMMENT '交易所的ID',
  `p_stock_id` bigint(20) NOT NULL DEFAULT '0' COMMENT '股票ID',
  `p_stock_code` varchar(12) NOT NULL DEFAULT '0' COMMENT '股票代码',
  `p_order_direction` int(4) NOT NULL DEFAULT 1 COMMENT '买卖方向 1:买 2:卖',
  `p_deal_amount` int(10) NOT NULL DEFAULT '0' COMMENT '成交数量',
  `p_prebuy_amount` int(10) NOT NULL DEFAULT '0' COMMENT '预买数量',
  `p_deal_price` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '成交价格',
  `p_fee_total` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '总费用',
  `p_currency_rate` decimal(20,8) NOT NULL DEFAULT '1.00000000' COMMENT '参考汇率,计算浮盈,总市值时使用',
  `p_currency_rate_cj` decimal(20,8) NOT NULL DEFAULT '1.00000000' COMMENT '成交汇率,计算实际盈亏时使用,结算时用结算汇率重算',
  `p_unit_id` bigint(20) NOT NULL DEFAULT '0' COMMENT '用户资产ID',
  `p_channel_type` int(10) NOT NULL DEFAULT '0' COMMENT '通道类型 1:外盘 2:内盘',
  `p_channel_id` bigint(20) NOT NULL DEFAULT '0' COMMENT '通道ID',
  `p_qfii_state` int(4) NOT NULL DEFAULT 0 COMMENT 'qfii状态 1: 是qfii，2:不是qfii, DMA',
  `p_account_id` bigint(20) NOT NULL DEFAULT '0' COMMENT '通道对应的分帐户ID',
  `p_trade_type` bigint(20) NOT NULL DEFAULT '0' COMMENT '交易类型 1:普通股票交易, 2:融券交易',
  `p_stock_type` bigint(20) NOT NULL DEFAULT '0' COMMENT '股票类型',
  `p_leverage` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '品种杠杆比率',
  `p_updatetime` bigint(20) NOT NULL DEFAULT '0' COMMENT '更新时间,时间戳',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT='成交明细历史信息表';

-- drop table if exists `phoenix_user_assets`;
-- CREATE TABLE `phoenix_user_assets` (
--   `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
--   `p_unit_id` int(10) NOT NULL DEFAULT '0' COMMENT '用户资产ID',
--   `en_real_cash` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '融资本金',
--   `en_credit_cash` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '融资信用金',
--   `en_risk_capital` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '风控总资产',
--   `st_total_value` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '持仓成本',
--   `st_position_value` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '持仓市值',
--   `st_position_value_star` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '创业板持仓市值',
--   `st_prebuy_value_star` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '创业板开仓挂单金额汇总',
--   `st_position_profit` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '持仓盈亏 ∑(持仓数量*(最新价-均价))',
--   `p_date` int(10) NOT NULL DEFAULT '0' COMMENT '所属日期',
--   `p_updatetime` bigint(20) NOT NULL DEFAULT '0' COMMENT '更新时间,时间戳',
--   PRIMARY KEY (`id`),
--   UNIQUE KEY `unique_trade_account_id` (`p_unit_id`) USING BTREE COMMENT '通道号唯一'
-- ) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT='交易账户资产信息表';

-- drop table if exists `phoenix_user_assets_his`;
-- CREATE TABLE `phoenix_user_assets_his` (
--   `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
--   `p_unit_id` int(10) NOT NULL DEFAULT '0' COMMENT '用户资产ID',
--   `en_real_cash` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '融资本金',
--   `en_credit_cash` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '融资信用金',
--   `en_risk_capital` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '风控总资产',
--   `st_total_value` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '持仓成本',
--   `st_position_value` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '持仓市值',
--   `st_position_value_star` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '创业板持仓市值',
--   `st_prebuy_value_star` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '创业板开仓挂单金额汇总',
--   `st_position_profit` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '持仓盈亏 ∑(持仓数量*(最新价-均价))',
--   `p_date` int(10) NOT NULL DEFAULT '0' COMMENT '所属日期',
--   `p_updatetime` bigint(20) NOT NULL DEFAULT '0' COMMENT '更新时间,时间戳',
--   PRIMARY KEY (`id`),
--   UNIQUE KEY `unique_user_assets_his` (`p_unit_id`,`p_date`) USING BTREE COMMENT '通道号唯一'
-- ) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT='历史交易账户资产信息表';

drop table if exists `phoenix_trans_detail`;
CREATE TABLE `phoenix_trans_detail` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `p_account_target` int(12) NOT NULL DEFAULT '0' COMMENT '操作账户',
  `p_account_source` int(12) NOT NULL DEFAULT '0' COMMENT '对应的账户',
  `p_op_flag` int(4) NOT NULL DEFAULT '0' COMMENT '操作类型 增加:2, 1:减少',
  `p_trans_value` decimal(20,4) NOT NULL DEFAULT '0.0000' COMMENT '发生金额',
  `p_capital_type` int(4) NOT NULL DEFAULT '0' COMMENT '资金类型',
  `p_account_no` int(12) NOT NULL DEFAULT '0' COMMENT '操作人员编号',
  `p_datetime` bigint(20) NOT NULL DEFAULT '0' COMMENT '发生时间',
  `p_remark` varchar(200) NOT NULL DEFAULT '0' COMMENT '备注说明',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT='账户资产转账详细信息表';

-- ---------------------------------- --
-- --------- Reset ------------------ --
-- --------- 分帐户的钱的变化 -------- --
-- --------- 该分帐户持仓的数据变化 --- --
-- ---------------------------------- --
drop table if exists `phoenix_account_reset_detail`;
CREATE TABLE `phoenix_account_reset_detail` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `p_account_unit_id` int(12) NOT NULL DEFAULT '0' COMMENT 'rest账户',
  `p_float_profit_before` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT 'reset前的浮动盈亏',
  `p_current_profit_before` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT 'reset前的实际盈亏',
  `p_float_profit_after` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT 'reset后的浮动盈亏',
  `p_current_profit_after` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT 'reset后的实际盈亏',
  `p_current_principal` decimal(20,8) NOT NULL COMMENT 'reset时的本金',
  `p_account_no` int(12) NOT NULL DEFAULT '0' COMMENT '操作人员编号',
  `p_datetime` bigint(20) NOT NULL DEFAULT '0' COMMENT '发生时间',
  `p_remark` varchar(200) NOT NULL DEFAULT '0' COMMENT '备注说明',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT='分帐户reset信息记录表';

-- ----------------------------
-- Table structure for `phoenix_risk_details`
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_risk_details`;
CREATE TABLE `phoenix_risk_details`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `sys_date` int(11) DEFAULT 0 COMMENT '日期',
  `user_id` int(11) NOT NULL DEFAULT 0 COMMENT '用户Id',
  `current_cash` decimal(16, 4) NOT NULL COMMENT '当前本金',
  `credit_cash` decimal(16, 4) NOT NULL COMMENT '信用金',
  `real_cash` decimal(16, 4) NOT NULL COMMENT '实际金额',
  `warn_line` decimal(16, 4) NOT NULL COMMENT '风控警告线',
  `close_line` decimal(16, 4) NOT NULL COMMENT '平仓线',
  `risk_rate` decimal(16, 5) NOT NULL COMMENT '风险率',
  `credit_multiple` decimal(16, 4) NOT NULL COMMENT '信用倍数',
  `loan_cash` decimal(16, 4) NOT NULL COMMENT '已借金额',
  `total_value` decimal(16, 4) NOT NULL COMMENT '持仓总市值',
  `create_datetime` bigint(20) NOT NULL,
  `total_asset_value` decimal(16, 4) NOT NULL COMMENT '总资产',
  `risk_type` int(11) DEFAULT 0 COMMENT '1:风控强平，2:风控预警  3:管理员强平',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT = '风控记录表';

-- ----------------------------
-- Table structure for `oms_tradeconfig`
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_oms_tradeconfig`;
CREATE TABLE `phoenix_oms_tradeconfig`(
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `exchange_id` int(11) NOT NULL COMMENT '市场id',
  `order_direction` int(11) NOT NULL COMMENT '委托方向',
  `clear_speed` char(1) NOT NULL COMMENT '交收类型:0：T+0, 1: T+1, N: 不处理',
  `market_cash_add_type` int(11) NOT NULL COMMENT '本市场头寸增加方式',
  `deal_cash_frozen_type` int(11) NOT NULL COMMENT '成交时是否进行冻结。0: 未冻结 1: T+0冻结 2: T+1冻结 3: T+2冻结',
  `deal_stock_frozen_type` int(11) NOT NULL COMMENT '成交时是否进行冻结 0: 未冻结 1: T+0冻结 2: T+1冻结 3: T+2冻结',
  `asset_settle_point` int(11) NOT NULL,
  `asset_settle_date` int(11) NOT NULL COMMENT '资产处理日期: 0:交易日, 1: 交收日',

  PRIMARY KEY (`id`),
  UNIQUE index `idx_oms_tradeconfig` (`exchange_id`, `order_direction`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT = '交易配置表';

-- ----------------------------
-- Table structure for `oms_feeset`
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_oms_feeset`;
CREATE TABLE `phoenix_oms_feeset`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `fee_type` varchar(5) NOT NULL DEFAULT '' COMMENT '费用类型',
  `exchange_id` int(11) NOT NULL COMMENT '市场id',
  `capital_ratio` decimal(12, 8) NOT NULL COMMENT '成交金额比例',
  `face_value_ratio` decimal(12, 8) NOT NULL COMMENT '成交面值比例',
  `amount_ratio` decimal(12, 8) NOT NULL COMMENT '成交数量比例',
  `maximum_fee` decimal(12, 4) NOT NULL COMMENT '最高费用',
  `minimum_fee` decimal(12, 4) NOT NULL COMMENT '最低费用',
  `currency_no` varchar(4) DEFAULT '' COMMENT '最高/低收费币种类型: CNY/HKD/USD',
  `modify_date` int(8) NOT NULL COMMENT '维护日期',
  `order_direction` int(11) NOT NULL COMMENT '委托方向 1买入，2，卖出，默认:!',
  `unit_id` bigint(20) NOT NULL DEFAULT -1 COMMENT '-1: 不区分',
  `serfee_type` int(11) NOT NULL COMMENT '技术服务扣减资产: 1-扣除 2-不扣除',
  `close_type` int(11) NOT NULL,
  `stock_type` int(11) NOT NULL DEFAULT 0 COMMENT '证券类别: 0不区分',
  `decimal_type` int(11) NOT NULL COMMENT '小数位处理方式 1:四舍五入  2: 全舍  3: 全入',
  `channel_id` int(11) NOT NULL DEFAULT 0 COMMENT '通道: 0 不区分',
  `plate_id` int(11) NOT NULL DEFAULT 0 COMMENT '板块id 默认0',
  `tid` bigint(20) NOT NULL DEFAULT 0 COMMENT '管理库费用表ID',
    PRIMARY KEY (`id`),
  UNIQUE INDEX `idx_oms_fee_set`  (`fee_type`, `exchange_id`, `unit_id`, `order_direction`, `stock_type`, `channel_id`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT = '费用配置表';

-- ----------------------------
-- Table structure for `sys_dictionary`
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_sys_dictionary`;
CREATE TABLE `phoenix_sys_dictionary`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `dictionary_no` int(11) NOT NULL,
  `lemma_item` varchar(32) NOT NULL,
  `item_name` varchar(32) NOT NULL,
  `allow_edit` char(32) NOT NULL,
  `code_length` int(11) NOT NULL,
  `opposite_no` int(11) NOT NULL,
  `opposite_item` varchar(32) NOT NULL,
    PRIMARY KEY (`id`),
  UNIQUE INDEX `idx_sys_dictionary` (`dictionary_no`, `lemma_item`) USING BTREE
)  ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT = '数字字典表';

-- -------------b---------------
-- Table structure for `sys_system`
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_sys_system`;
CREATE TABLE `phoenix_sys_system`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `system_code` varchar(6) NOT NULL,
  `system_name` varchar(64) NOT NULL,
  `before_preinit_date` int(11) NOT NULL DEFAULT 0 COMMENT '上上个初始化日期',
  `preinit_date` int(11) NOT NULL COMMENT '前一初始化日期',
  `init_date` int(11) NOT NULL COMMENT '当前初始化日期',
  `init_real_time` bigint(20) NOT NULL COMMENT '实际初始化时间',
  PRIMARY KEY (`id`),
  UNIQUE INDEX `idx_sys_system` (`system_code`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT = '系统信息表';

-- ----------------------------
-- Table structure for tbackconfig
-- ----------------------------
DROP TABLE IF EXISTS `phoenix_sys_backconfig`;
CREATE TABLE `phoenix_sys_backconfig`  (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `table_name` varchar(64) NOT NULL DEFAULT '' COMMENT '待归档表名',
  `tag_table_name` varchar(64) NOT NULL DEFAULT '' COMMENT '归档表名',
  `back_flag` int(11) NOT NULL,
  `init_flag` int(11) NOT NULL,
  `deal_flag` int(11) NOT NULL,
  `condition` varchar(1000) NOT NULL,
  `back_date` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `idx_sys_bacconfig` (`table_name`) USING BTREE
)  ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT = '结算归档配置表';


-- 虚拟交易所相关表
drop table if exists `phoenix_exc_orderinfo`;
CREATE TABLE `phoenix_exc_orderinfo` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `order_id` bigint(4) NOT NULL DEFAULT '0' COMMENT '委托id（唯一）',
--   `stock_id` varchar(45) COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '证券key',
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
  -- `relate_id` int(10) NOT NULL COMMENT '关联委托ID',
  `receive_date` int(20) NOT NULL COMMENT '接收日期',
  `ex_status` int(4) NOT NULL DEFAULT '0' COMMENT '状态 0: 未知 1：未处理  2：已处理 3：撤单',
  `created_at` bigint(20) NOT NULL COMMENT '创建时间',
  `updated_at` bigint(20) NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`),
  unique key `unique_exorder_id_idx` (`order_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='收到的订单信息表';

drop table if exists `phoenix_exc_dealdetail`;
CREATE TABLE `phoenix_exc_dealdetail` (
  `id` BIGINT(20) NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `order_id` BIGINT(20) NOT NULL COMMENT '委托id',
  `stock_code` VARCHAR(45) NOT NULL COMMENT '证券代码',
  `deal_amount` bigint(20) NOT NULL COMMENT '成交数量',
  `deal_price` varchar(20) NOT NULL COMMENT '成交价格',
  `deal_direction` int(10)	NOT NULL DEFAULT 0 COMMENT '买卖方向 1：买 2：卖',
  `deal_time` bigint(20) NOT NULL COMMENT '成交时间戳',
  `deal_number` nvarchar(50)	not null default '' comment '成交确认号',
  `created_at` bigint(20) NOT NULL COMMENT '创建时间',
  PRIMARY KEY (`id`)
) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT = '成交明细表';
