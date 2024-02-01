
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::dataservice::entities::{
    prelude::PhoenixOrdSuborder, 
    prelude::PhoenixOrdStockorder,
    prelude::PhoenixOrdStockdeal,
    prelude::PhoenixOrdPendSettle,
};
use crate::common::common::OrderDetail;
use crate::protofiles::{RouterMsg, OrderType, OrderMsg, MsgType, MsgContent};

pub async fn convert_to_stockorder(order_detail: &OrderDetail) -> PhoenixOrdStockorder {
    let mut stock_order = PhoenixOrdStockorder::new();
    // stock_order.id
    stock_order.order_no = order_detail.order_id;
    stock_order.sys_date = order_detail.sys_date;
    stock_order.unit_id = order_detail.unit_id;
    stock_order.stock_id = order_detail.stock_id;
    stock_order.stock_code = order_detail.stock_code.to_owned();                    // 接口查询,缓存读取
    stock_order.exchange_id = order_detail.exchange_id;                            // 接口查询,缓存读取
    stock_order.order_direction = order_detail.order_direction;
    stock_order.order_price = order_detail.order_price;
    stock_order.order_amount = order_detail.order_amount;
    stock_order.order_type = order_detail.order_type;
    stock_order.pre_fee = order_detail.pre_fee;       //需要计算
    stock_order.pre_capital = order_detail.capital;   //需要计算
    stock_order.price_type = order_detail.price_type;
    stock_order.deal_amount = order_detail.deal_amount; //0
    stock_order.deal_value = order_detail.deal_value;//0
    stock_order.deal_fee = order_detail.deal_fee;//0
    stock_order.cancel_amount = 0;
    stock_order.order_status = order_detail.order_status; //下单 1:未报
    stock_order.last_deal_time = 0;
    stock_order.relate_order = 0;//order_detail.relate_id;
    stock_order.trade_type = order_detail.trade_type;
    stock_order.order_memo = order_detail.memo.to_owned();
    stock_order.create_time = order_detail.create_time;
    stock_order.modify_time = 0;
    stock_order
}

pub async fn convert_to_suborder(order_detail: &OrderDetail) -> PhoenixOrdSuborder {
    let mut sub_order = PhoenixOrdSuborder::default();
    // sub_order.id = NotSet;
    sub_order.sub_id = order_detail.sub_id;
    sub_order.sys_date = order_detail.sys_date;
    sub_order.order_no = order_detail.order_id;
    sub_order.unit_id = order_detail.unit_id;
    sub_order.stock_code = order_detail.stock_code.to_owned();
    sub_order.channel_id = order_detail.channel_id;   //需要根据实际赋值
    sub_order.channel_type = order_detail.channel_type; //需要根据实际赋值
    sub_order.order_amount = order_detail.order_amount;
    sub_order.order_price = order_detail.order_price;
    sub_order.price_type = order_detail.price_type;
    sub_order.confirm_no = order_detail.confirm_no.to_owned();
    sub_order.order_status = order_detail.order_status; // 未报
    sub_order.deal_value = order_detail.deal_value;
    sub_order.deal_amount = order_detail.deal_amount; //下单时成交量为0
    sub_order.modify_time = 0;
    sub_order.create_time = order_detail.create_time;
    sub_order.cancel_flag = 0;
    sub_order.relate_order = order_detail.relate_id;
    sub_order.remark = order_detail.memo.to_owned();
    sub_order
}


pub async fn convert_to_stockdeal(detail: &OrderDetail) -> PhoenixOrdStockdeal {
    let mut stock_deal = PhoenixOrdStockdeal::default();
    // stock_deal.id = ;
    stock_deal.deal_no = detail.deal_id;
    stock_deal.sys_date = detail.sys_date;
    stock_deal.order_no = detail.order_id;
    stock_deal.exchange_id = detail.exchange_id;
    stock_deal.unit_id = detail.unit_id;
    stock_deal.stock_id = detail.stock_id;
    stock_deal.stock_code = detail.stock_code.to_owned();
    stock_deal.order_direction = detail.order_direction;
    stock_deal.deal_time = detail.last_deal_time as i32;
    stock_deal.deal_amount = detail.deal_amount;
    stock_deal.deal_price = detail.deal_price;
    stock_deal.fee_jy = detail.fee_jy;
    stock_deal.fee_gh = detail.fee_gh;
    stock_deal.fee_yj = detail.fee_yj;
    stock_deal.fee_js = detail.fee_js;
    stock_deal.fee_zg = detail.fee_zg;
    stock_deal.fee_qt = detail.fee_qt;
    stock_deal.fee_js2 = detail.fee_js2;
    stock_deal.fee_jg = detail.fee_jg;
    stock_deal.fee_yh = detail.fee_yh;
    stock_deal.fee_real_yj = detail.fee_real_yj;
    stock_deal.fee_total = detail.fee_total;
    stock_deal.exec_no = detail.exec_id.to_owned();
    stock_deal.channel_type = detail.channel_type;
    stock_deal.channel_id = detail.channel_id;
    stock_deal.refer_profit = detail.profit;
    stock_deal.trade_type = detail.order_type;
    stock_deal.create_time = detail.create_time;
    stock_deal
}

pub async fn convert_to_pendsettle(detail: &OrderDetail) -> PhoenixOrdPendSettle {
    let mut pend_settle = PhoenixOrdPendSettle::new();
    // id: todo!(),
    pend_settle.settle_id = detail.settle_id;
    pend_settle.unit_id = detail.unit_id;
    pend_settle.sys_date = detail.sys_date;
    pend_settle.trade_date = detail.sys_date;
    pend_settle.settle_date = detail.settle_date;
    pend_settle.clear_speed = detail.clear_speed;
    pend_settle.order_no = detail.order_id;
    pend_settle.order_amount = detail.order_amount;
    pend_settle.exchange_id = detail.exchange_id;
    pend_settle.stock_code = detail.stock_code.to_owned();
    pend_settle.stock_id = detail.stock_id;
    pend_settle.order_direction = detail.order_direction;
    pend_settle.settle_amount = detail.deal_amount;
    pend_settle.settle_internal_amount = detail.deal_amount;
    pend_settle.settle_balance =  detail.settle_capital;
    pend_settle.net_balance = detail.deal_value;
    if !Decimal::from(detail.deal_amount).is_zero() {
        pend_settle.deal_avg_price = pend_settle.net_balance / Decimal::from(detail.deal_amount);
    }
    pend_settle.fee_total = detail.fee_total;
    pend_settle.fee_jy = detail.fee_jy;
    pend_settle.fee_yh = detail.fee_yh;
    pend_settle.fee_gh = detail.fee_gh;
    pend_settle.fee_yj = detail.fee_yj;
    pend_settle.fee_js = detail.fee_js;
    pend_settle.fee_zg = detail.fee_zg;
    pend_settle.fee_other = detail.fee_qt;
    pend_settle.fee_js2 = Some(detail.fee_js2);
    pend_settle.fee_jg = Some(detail.fee_jg);
    pend_settle.asset_settle_date = detail.asset_settle_date;
    pend_settle.status = 0;
    pend_settle.currency_no = detail.currency_no.to_owned();
    pend_settle.remark = Some(detail.memo.to_owned());
    pend_settle.currency_rate = detail.rate;
    pend_settle.channel_id = detail.channel_id;

    pend_settle
}

//报盘消息
pub async fn convert_to_orderrouterinfo(sub_order: &PhoenixOrdSuborder, order_detail: &OrderDetail) -> RouterMsg {
    let mut order_msg = OrderMsg::default(); 
    if order_detail.order_flag == OrderType::Place {
        // 下单
        order_msg.order_type       = OrderType::Place as i32;         // 订单类型:见OrderType
        order_msg.order_qty        = sub_order.order_amount;         // 订单数量或撤单数量
        // order_msg.order_id         = sub_order.sub_id;             // 委托id
        order_msg.channel_type     = sub_order.channel_type;         // 通道类型 1 外盘, 2 内盘
        order_msg.channel_id       = sub_order.channel_id as i64;    // 通道id

        order_msg.order_id         = order_detail.order_id;             // 委托id
        order_msg.currency_no      = order_detail.currency_no.to_owned();// 币种
        order_msg.order_price      = order_detail.order_price.to_f64().unwrap_or_default();         // 委托价格
        order_msg.stock_code       = order_detail.stock_code.to_owned();// 证券代码
        order_msg.exchange_id      = order_detail.exchange_id as i64;    // 市场id
        order_msg.order_direction  = order_detail.order_direction;      // 委托方向  1=buy  2=sell
        order_msg.price_type       = order_detail.price_type;           // 价格类型(市价限价)
    } else {
        // 撤单
        order_msg.order_type       = OrderType::Cancel as i32;         // 订单类型:见OrderType
        order_msg.brk_order_id     = sub_order.confirm_no.to_owned();// 委托确认号
        // order_msg.order_id         = sub_order.sub_id;             // 委托id
        order_msg.channel_type     = sub_order.channel_type;         // 通道类型 1 外盘, 2 内盘
        order_msg.channel_id       = sub_order.channel_id as i64;    // 通道id
        order_msg.price_type       = sub_order.price_type;           // 价格类型(市价限价)

        order_msg.order_id         = order_detail.order_id;             // 委托id
        order_msg.cancel_id        = order_detail.cancel_id;        // 撤单id(新生成的撤单委托序号)
        order_msg.order_qty        = order_detail.cancel_amount;         // 撤单数量(需要计算)
        order_msg.stock_code       = order_detail.stock_code.to_owned();// 证券代码
        order_msg.exchange_id      = order_detail.exchange_id as i64;    // 市场id
        order_msg.order_direction  = order_detail.order_direction;      // 委托方向  1=buy  2=sell
    }

    let mut router_msg = RouterMsg::default();
    router_msg.msg_type = MsgType::Order as i32;
    let mut msg_content = MsgContent::default();
    msg_content.order_msg = Some(order_msg);
    router_msg.msg_content = Some(msg_content);
    router_msg.msg_id   = order_detail.msg_id;  //是否使用同一个?
    router_msg.msg_time = order_detail.msg_time;
    router_msg
}