use anyhow::Result;
use rust_decimal::prelude::*;
use common::{
    akaclient::AkaClient,
    redisclient::redisclient::RedisClient,
    constant::OrderDirection,
    protofiles::{TradeDateInfo, Currency, TradeDateReq}, logclient::LogClient,
};

use crate::client::AccountRiskClient;

use crate::common::common::OrderDetail;

pub async fn get_settle_date(detail: &mut OrderDetail, redis_client: &RedisClient) {
    if detail.clear_speed == 0 {
        return 
    }
    let mut index = detail.clear_speed;
    if detail.exchange_id == 103 {
        //当下一交易日为半日市, 清算速度+1
        let key = format!("holiday_market_{}", detail.exchange_id);
        let date = redis_client.get_values_by_lindex(&key, 1).await;
        log::info!("半日市: {}", date);
        if date.as_bytes()[9] != 49 {
            index += 1;
        }
        // if date.pop() != Some('1') {
        //     index += 1;
        // }
    }
    log::info!("index: {}", index);
    if detail.exchange_id != 0 {
        let key = format!("holiday_market_{}", detail.exchange_id);
        let date_ret = redis_client.get_values_by_lindex(&key, index).await;
        let date: Vec<&str> = date_ret.split(',').collect();
        if !date.is_empty() {
            detail.settle_date = date[0].parse().unwrap_or_default();
        }
    }
}


// pub async fn get_settle_day(s: i32, offset: i32, exchange_id: i64, mo: i32, redis_client: &RedisClient) -> i32{
//     let key = format!("holiday_market_{}", exchange_id);
//     if s + offset >= 17 {
//         log::error!("[{}]超过当前可取记录", s + offset);
//         return -1;
//     }
//     let date_ret = redis_client.get_values_by_lindex(&key, s + offset).await;
//     let date: Vec<&str> = date_ret.split(',').collect();
//     if date.is_empty() {
//         log::error!("获取交收日期出错");
//         return -1;
//     }

//     if date_ret.len() >= 10 {
//         if mo == 1 {
//             if date_ret.as_bytes()[9] != 49 {
//                 log::info!("下个日期是半日市:{}, {}", offset, date_ret);
//                 let dt = get_settle_day(s, offset + 1, exchange_id, 1, redis_client).await;
//                 return dt;
//             }
//         }
//     }
//     date[0].parse().unwrap_or_default()
// }

pub async fn get_stock_info(detail: &mut OrderDetail, aka_client: &AkaClient, account_risk_client: &AccountRiskClient) -> Result<()>{
    let ret = aka_client.query_stock_info(detail.stock_id).await;
    if ret.as_ref().is_err() {
        log::error!("{}", ret.as_ref().err().unwrap().to_string());
        return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let stock_info = ret.unwrap();
    detail.exchange_id = stock_info.market_id as i32;
    detail.stock_type = stock_info.stock_type;
    detail.exchange_code =  stock_info.market_code.to_owned();
    detail.stock_code = stock_info.stock_code.to_owned();
    detail.margin_rate = Decimal::from_f64(stock_info.margin_rate).unwrap_or_default();
    let ret = account_risk_client.query_margin_ratio(detail.unit_id, detail.stock_id).await;
    if ret.as_ref().is_err() {
        log::error!("{}", ret.as_ref().err().unwrap().to_string());
        LogClient::get().push_error(format!("{}", ret.as_ref().err().unwrap().to_string()).as_str()).await;
        return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    let margin_rate = Decimal::from_f64(ret.unwrap().margin_ratio).unwrap_or_default();
    log::info!("保证金比例: stock: {}, unit: {}", detail.margin_rate, margin_rate);
    if detail.margin_rate < margin_rate {
        detail.margin_rate = margin_rate;
    }
    if stock_info.trade_currency() == Currency::Cny {
        detail.currency_no = "CNY".to_owned();
    } else if stock_info.trade_currency() == Currency::Usd {
        detail.currency_no = "USD".to_owned();
    } else {
        detail.currency_no = "HKD".to_owned();
    }

    let ret = aka_client.query_exchange_rate(&detail.currency_no).await;
    if ret.as_ref().is_err() {
        log::error!("{}", ret.as_ref().err().unwrap().to_string());
        return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }

    // let exchange_rate = ret.unwrap(); 
    if detail.order_direction == OrderDirection::BUY as i32 {
        // 人民币 ==> 港币
        detail.rate = Decimal::from_f64(ret.unwrap().buy_rate).unwrap_or_default();
    } else {
        detail.rate = Decimal::from_f64(ret.unwrap().sell_rate).unwrap_or_default();
    }
    log::info!("stock info currency: {}, order direction: {}, rate: {}", detail.currency_no, detail.order_direction, detail.rate);
    Ok(())
}

pub async fn query_trade_date(market_id: i64, query_date: i32, query_type: i32, date_offset: i32, aka_client: &AkaClient) ->Result<TradeDateInfo> {
    let ret = aka_client.query_trade_date(market_id, query_date, query_type, date_offset).await;
    if ret.as_ref().is_err() {
        log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
        return Err(anyhow!("查询交易日出错"));
    }
    let trade_date_info = ret.unwrap();
    if trade_date_info.is_empty() {
        log::error!("交易日为空");
        return Err(anyhow!("查询交易日出错"));
    }
    // let s: Vec<TradeDateInfo> = trade_date_info.iter().filter(|x| x.market_id == 1).map(|x| x.clone()).collect();
    // trade_date_info.iter_mut().map(|x| {x.current_date = 1;x.date_type = 2});
    log::info!("日期: {:?}", trade_date_info);
    Ok(trade_date_info[0].clone())
}