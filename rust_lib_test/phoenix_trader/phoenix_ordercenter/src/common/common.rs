use chrono::{Timelike, Datelike};
use common::{uidservice::UidgenService, constant::OrderStatus, constant::OrderDirection};
use rust_decimal::Decimal;
use rust_decimal::prelude::{ToPrimitive, FromPrimitive};

use crate::protofiles::{OrderType, OrderReq, CancelReq, ExecMsg, ExecType};
use crate::dataservice::entities::prelude::PhoenixOrdSuborder;

#[derive(Debug, Clone, Default)]
pub struct OrderDetail {
        pub msg_id: i64,
        /// 用户相关
        pub unit_id: i64,               // 用户id
        pub operator_no: i64,           // 操作员
        pub trade_mode: i32,            // 1:USER(用户直连) 2:AGENT(代理托管)
        pub agent_account: i64,         // 代理账户
        /// 证券相关
        pub stock_id: i64,              // 证券id
        pub stock_type: i32,            // 类型
        pub exchange_id: i32,           // 市场id
        pub exchange_code: String,      // 市场XSHG
        pub stock_code: String,         // 证券代码
        pub currency_no: String,        // 币种
        pub channel_type: i32,          // 通道类型 1 外盘, 2 内盘
        pub channel_id: i32,            // 通道id
        pub margin_rate: Decimal,          // 品种保证金比例
        /// 订单相关
        pub order_id: i64,              // 委托id
        pub sub_id: i64,                // 子委托id
        pub relate_id: i64,             // 关联订单
        pub cancel_id: i64,             // 撤单id
        pub deal_id: i64,               // 成交号, 自己生成的
        pub exec_id: String,            // 成交号, 交易所返回
        pub settle_id: i64,             // 交收

        pub order_amount: i32,          // 订单数量
        pub deal_amount: i32,           // 成交数量
        pub un_deal_amount: i32,        // 未成交数量
        pub cancel_amount: i32,         // 撤单数量

        pub order_price: Decimal,           // 委托价格
        pub deal_price: Decimal,            // 成交价
        pub avg_price: Decimal,             // 均价
        pub deal_value: Decimal,            // 成交金额

        pub trade_type: i32,                // '交易类型：1:股票买卖， 2:融券交易',
        pub order_status: i32,          // 订单状态
        pub order_direction: i32,       // 委托方向  1=买  2=卖
        pub price_type: i32,            // 价格类型(市价限价) 
        pub order_type: i32,            // 委托类型 1:app下单  2:跟单  3:风控止盈止损平仓单,4:风控总资产预警平仓单 5:pc客户端单 6:结算平仓单 7:管理端强平仓单,8:app清仓,9:pc清仓,10,管理员平仓,11,合约到期日强平
        pub cancel_type: i32,           // 撤单类型 1:app撤单  2:pc撤单  3:风控撤单  4:管理员撤单

        /// 费用
        pub profit: Decimal,                // 盈亏
        pub item_fee: Decimal,              // 分项费用
        pub pre_fee: Decimal,               // 预估交易费用
        // pub pre_capital: Decimal,           // 预买或者预卖金额
        pub deal_fee: Decimal,              // 真实成交费用
        pub rate: Decimal,                  // 汇率
        // pub face_value: Decimal,            // 面值
        pub capital: Decimal,               // 金额
        pub settle_capital: Decimal,        // 结算金额(包含费用)
        // pub pre_buy_capital: Decimal,
        // pub presale_capital: Decimal,

        pub sys_date: i32,              // 业务日期 20191219
        pub deal_time: String,          // 成交时间 2019-12-19 11:21:19
        pub create_time: i64,           // 订单生成时间 112119
        pub modify_time: i64,           // 修改时间 112119
        pub last_deal_time: i64,        // 首次成交时间 112119
        pub msg_time: i64,              // 消息时间 112119
        pub asset_settle_date: i32,     // 交收资金的日期
        pub settle_date: i32,           // 交割日期
        pub clear_speed: i32,           // 清算速度

        //费用相关
        pub fee_type: String,           // 费用类型
        pub fee_jy: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '交易费',
        pub fee_yh: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '印花税',
        pub fee_gh: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '过户费',
        pub fee_yj: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '佣金',
        pub fee_js: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '经手费',
        pub fee_zg: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '证管费',
        pub fee_qt: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '其他费用',
        pub fee_js2: Decimal,               //(10,2) DEFAULT '0.00' COMMENT '结算费',
        pub fee_jg: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '交割费',
        pub fee_fx: Decimal,                //风险金
        pub fee_real_yj: Decimal,           //(10,2) DEFAULT '0.00',
        pub fee_total: Decimal,             //(10,2) DEFAULT '0.00',

        //其他
        pub confirm_no: String,         // 确认号
        pub exec_type: ExecType,        // 回执类型
        pub order_flag: OrderType,      // 下单委托/撤单委托
        pub memo: String,               // 备注
}

impl OrderDetail {
        pub async fn place_order(order: &OrderReq) -> Self{
                let mut order_detail = OrderDetail::default();
                order_detail.unit_id = order.unit_id;
                order_detail.stock_id = order.stock_id;
                order_detail.order_direction = order.order_direction;
                order_detail.order_amount = order.order_qty;
                order_detail.price_type = order.price_type;
                order_detail.order_price = Decimal::from_f64(order.order_price).unwrap_or_default();
                order_detail.operator_no = order.operator_no;
                order_detail.order_type = order.order_type;
                order_detail.trade_mode = order.trade_mode;
                order_detail.agent_account = order.agent_account;
                order_detail.trade_type = 1;

                let naive_time = utility::timeutil::current_naive_time();
                order_detail.msg_time = format!("{:02}{:02}{:02}", naive_time.hour(), naive_time.minute(), naive_time.second()).parse().unwrap_or_default();
                order_detail.create_time = utility::timeutil::current_timestamp();
                order_detail.order_status = OrderStatus::INITED as i32;
                order_detail.order_flag = OrderType::Place; //下单
                order_detail
        } 

        pub async fn cancel_order(cancel_order: &CancelReq) -> Self{
                let mut order_detail = OrderDetail::default();

                order_detail.unit_id        = cancel_order.unit_id;         // 用户id
                order_detail.order_id       = cancel_order.order_id;         // 订单id(撤单用)
                order_detail.operator_no    = cancel_order.operator_no;         // 操作员
                order_detail.cancel_type    = cancel_order.cancel_type;          // 撤单类型 1:app撤单  2:pc撤单  3:风控撤单  4:管理员撤单
                order_detail.trade_mode     = cancel_order.trade_mode;         // 1:USER(用户直连) 2:AGENT(代理托管)
                order_detail.agent_account  = cancel_order.agent_account;         // 代理账户

                order_detail.order_flag = OrderType::Cancel;//撤单
                let naive_time = utility::timeutil::current_naive_time();
                order_detail.msg_time = format!("{:02}{:02}{:02}", naive_time.hour(), naive_time.minute(), naive_time.second()).parse().unwrap_or_default();
                order_detail.create_time = utility::timeutil::current_timestamp();
                order_detail
        }

        pub async fn receipt_msg(exec_msg: &ExecMsg) -> Self {
                let mut detail = OrderDetail::default();
                let build_time = utility::timeutil::build_naive_date_time(&exec_msg.exec_time);
                
                if exec_msg.exec_type == ExecType::Confirm as i32 {
                        detail.exec_type = ExecType::Confirm;
                } else if exec_msg.exec_type == ExecType::Filled as i32{ //成交
                        detail.deal_amount = exec_msg.exec_qty;
                        detail.exec_type = ExecType::Filled;
                } else if exec_msg.exec_type == ExecType::Canceled as i32{
                        detail.cancel_amount = exec_msg.exec_qty;
                        detail.memo =format!("撤单: {}", detail.memo);
                        detail.exec_type = ExecType::Canceled;
                } else if exec_msg.exec_type == ExecType::Rejected as i32 {
                        detail.cancel_amount = exec_msg.exec_qty;
                        detail.memo =format!("废单: {}", detail.memo);
                        detail.exec_type = ExecType::Rejected;
                }
                detail.order_id = exec_msg.order_id;
                detail.deal_price = Decimal::from_f64(exec_msg.exec_price).unwrap_or_default();
                detail.exec_id = exec_msg.exec_id.to_owned();//成交号
                detail.deal_time = exec_msg.exec_time.to_owned();
                detail.order_direction = exec_msg.order_direction;
                detail.confirm_no = exec_msg.brk_order_id.to_owned();
                detail.channel_id = exec_msg.channel_id as i32;
                detail.channel_type = exec_msg.channel_type;
                detail.memo = exec_msg.memo.to_owned();
                detail.last_deal_time = format!("{}{:02}{:02}", build_time.hour(), build_time.minute(), build_time.second()).parse().unwrap_or_default();
                detail.create_time = utility::timeutil::current_timestamp();
                detail
        }


        pub async fn opponent_order(detail: &mut OrderDetail, sub_order: &PhoenixOrdSuborder) -> Self {
                let mut opponent_order_detail = detail.clone();

                opponent_order_detail.order_amount = sub_order.order_amount;
                opponent_order_detail.order_price = sub_order.order_price;
                opponent_order_detail.channel_id = sub_order.channel_id;
                opponent_order_detail.channel_type = sub_order.channel_type;
                opponent_order_detail.order_status = sub_order.order_status;
                opponent_order_detail.price_type = sub_order.price_type;
                opponent_order_detail.relate_id = sub_order.sub_id;
                opponent_order_detail.sub_id = sub_order.relate_order;

                opponent_order_detail.capital = Decimal::from(sub_order.order_amount) * sub_order.order_price;
                if opponent_order_detail.order_direction == OrderDirection::BUY as i32 {
                        opponent_order_detail.order_direction = OrderDirection::SELL as i32;
                } else {
                        opponent_order_detail.order_direction = OrderDirection::BUY as i32;
                }
                opponent_order_detail.order_type = 1;
                opponent_order_detail.operator_no = 1;
                opponent_order_detail
        }
}
