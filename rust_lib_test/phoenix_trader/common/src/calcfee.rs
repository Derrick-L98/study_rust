use std::str::FromStr;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use anyhow::{Result, anyhow};
use crate::{
    redisclient::redisclient::RedisClient,
    protofiles::phoenixakacenter::FeeSetting, 
    constant::{OrderDirection, RateType, get_rate_key}, akaclient::AkaClient, 
};

#[derive(Debug, Clone, Default)]
pub struct FeeDetail {
    pub amount: i32,                // 委托数量/成交数量
    pub price: Decimal,             // 委托价格/成交价
    pub currency_no: String,        // 币种
    pub order_direction: i32,       // 下单方向
    pub rate: Decimal,              // 汇率
    // 以下是计算结果
    pub fee_jy: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '交易费',
    pub fee_yh: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '印花税',
    pub fee_gh: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '过户费',
    pub fee_yj: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '佣金',
    pub fee_js: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '经手费',
    pub fee_zg: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '证管费',
    pub fee_qt: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '其他费用',
    pub fee_js2: Decimal,               //(10,2) DEFAULT '0.00' COMMENT '结算费',
    pub fee_jg: Decimal,                //(10,2) DEFAULT '0.00' COMMENT '交割费',
    // pub fee_fx: Decimal,                //风险金
    // pub fee_real_yj: Decimal,           //(10,2) DEFAULT '0.00',
    pub fee_total: Decimal,             //(10,2) DEFAULT '0.00',
}

impl FeeDetail {
  pub async fn new() -> Self {
    FeeDetail::default()
  }

  // fee_settings: 基础服务查询
  pub async fn calc_fee_info_v2(
    currency_type: &str, amount: i32, price: f64, 
    order_direction: i32, exchange_id: i64, unit_id: i64, 
    channel_id: i64, stock_type: i32, akaclient: &AkaClient
  ) -> Result<FeeDetail> {
    //查汇率
    let ret = akaclient.query_exchange_rate(currency_type).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!("{}", ret.as_ref().unwrap_err().to_string()));
    }
    let rate_info = ret.unwrap();
    //查stockinfo
    let ret = akaclient.query_fee_setting("".to_string(), exchange_id, order_direction, unit_id, channel_id, stock_type).await;
    if ret.as_ref().is_err() {
      return Err(anyhow!("{}", ret.as_ref().unwrap_err().to_string()));
    }
    let fee_settings = ret.unwrap();
    let mut fee_detail: FeeDetail = FeeDetail { ..Default::default() };
    for fee in fee_settings.iter() {
      log::info!("fee_type: {}, amount: {}, price: {}, fee_ratio: {}", fee.fee_type, amount, price, fee.fee_ratio);
      let mut item_fee = Decimal::from_f64(fee.fee_ratio * amount as f64 * price).unwrap_or_default();
      log::info!("fee: {}", item_fee);

      if fee.minimum_fee.gt(&0.001) || fee.maximum_fee.gt(&0.001) {
        // 当前证券的交易币种 和 当前费用设置的币种 不一致, 需要将费用设置的最高/低费用转成当前交易币种.
        let mut rate = dec!(1.0);
        // let rate = rate_info.buy_rate;
        log::info!("current currency: {}, fee currency: {}", currency_type, fee.currency_type().as_str_name());
        if !currency_type.eq_ignore_ascii_case(&fee.currency_type().as_str_name()) {
          rate = Decimal::from_f64(rate_info.buy_rate).unwrap_or_default();
        }
        // if !currency_type.contains(&fee.currency_type().as_str_name()) {
        // if order_direction == OrderDirection::BUY as i32 {
        //   // format!("{}2{}", fee.currency_type().as_str_name(), fee_detail.currency_no)
        //   rate = rate_info.buy_rate;
        // } else {
        //   // format!("{}2{}", fee_detail.currency_no, fee.currency_type().as_str_name())
        //   rate = rate_info.sell_rate;
        // };
        // let ret = FeeDetail::get_rate(&str, redis_client).await;
        // if ret.as_ref().is_err() {
        //   log::error!("{}", ret.as_ref().err().unwrap().to_string());
        //   return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
        // }
        //在minimum_fee 与 maximum_fee 之间取值
        // let mut rate = fee_detail.rate;
        //if order_direction == OrderDirection::SELL as i32 {
        rate = dec!(1.0) / rate;
        //}
        // fee_detail.rate = rate;
        // }
        log::info!("=== 最终汇率 ===:{}", rate);
        if fee.minimum_fee > 0.001 && item_fee < Decimal::from_f64(fee.minimum_fee).unwrap_or_default() * rate {
          item_fee = Decimal::from_f64(fee.minimum_fee).unwrap_or_default() * rate;
        }
        if fee.maximum_fee > 0.001 && item_fee > Decimal::from_f64(fee.maximum_fee).unwrap_or_default() * rate {
          item_fee = Decimal::from_f64(fee.maximum_fee).unwrap_or_default() * rate;
        }
      }
      // 费用明细
      match &fee.fee_type as &str {
        "1" => {
          fee_detail.fee_jy = item_fee; //交易费
        }
        "2" => {
          fee_detail.fee_yh = item_fee; //印花税
        }
        "3" => {
          fee_detail.fee_gh = item_fee; //过户费
        }
        "4" => {
          fee_detail.fee_yj = item_fee; //佣金
        }
        "5" => {
          fee_detail.fee_js = item_fee; //经手费
        }
        "6" => {
          fee_detail.fee_zg = item_fee; //证管费
        }
        "7" => {
          fee_detail.fee_qt = item_fee; //其他费用
        }
        // "9" => {
        //     fee_detail.fee_fx = item_fee; //风险金
        // },
        "a" => {
          fee_detail.fee_js2 = item_fee; //结算费
        }
        "b" => {
          fee_detail.fee_jg = item_fee; //交割费
        }
        // "c" => {
        //     fee_detail.fee_jy = item_fee; //实际佣金
        // },
        // "d" => {
        //     fee_detail.fee_jy = item_fee; //券商佣金
        // },
        _ => {}
      }
    }
    fee_detail.fee_total = fee_detail.fee_jy + fee_detail.fee_yh + fee_detail.fee_gh + fee_detail.fee_yj + fee_detail.fee_js + fee_detail.fee_zg + fee_detail.fee_qt + fee_detail.fee_js2 + fee_detail.fee_jg;
    log::info!("fee_total: {}", fee_detail.fee_total);
    Ok(fee_detail)
  }


  // fee_settings: 基础服务查询
  pub async fn calc_fee_info(fee_detail: &mut FeeDetail, fee_settings: &Vec<FeeSetting>, redis_client: &RedisClient) -> Result<()> {
    log::info!("amount: {}, price: {}", fee_detail.amount, fee_detail.price);
    for fee in fee_settings.iter() {
      // log::info!("fee_type: {}, amount: {}, price: {}, fee_ratio: {}", fee.fee_type, fee_detail.amount, fee_detail.price, fee.fee_ratio);
      let mut item_fee = Decimal::from_f64(fee.fee_ratio * fee_detail.amount as f64 ).unwrap_or_default() * fee_detail.price;
      log::info!("item_fee: {}", item_fee);
      if fee.minimum_fee.gt(&0.001) || fee.maximum_fee.gt(&0.001) {
        let mut rate = dec!(1.0);
        // 当前证券的交易币种 和 当前费用设置的币种 不一致, 需要将费用设置的最高/低费用转成当前交易币种.
        log::info!("current currency: {}, fee currency: {}", fee_detail.currency_no, fee.currency_type().as_str_name());
        if !fee_detail.currency_no.contains(&fee.currency_type().as_str_name()) {
          let str = if fee_detail.order_direction == OrderDirection::BUY as i32 {
            format!("{}2{}", fee.currency_type().as_str_name(), fee_detail.currency_no)
          } else {
            format!("{}2{}", fee_detail.currency_no, fee.currency_type().as_str_name())
          };
          let ret = FeeDetail::get_rate(&str, redis_client).await;
          if ret.as_ref().is_err() {
            log::error!("{}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
          }
          rate = ret.unwrap();
          //在minimum_fee 与 maximum_fee 之间取值
          // let mut rate = fee_detail.rate;
          if fee_detail.order_direction == OrderDirection::SELL as i32 {
            rate = dec!(1.0) / rate;
          }
          // fee_detail.rate = rate;
        }
        if fee.minimum_fee > 0.001 && item_fee < Decimal::from_f64(fee.minimum_fee).unwrap_or_default() * rate {
          item_fee = Decimal::from_f64(fee.minimum_fee).unwrap_or_default() * rate;
        }
        if fee.maximum_fee > 0.001 && item_fee > Decimal::from_f64(fee.maximum_fee).unwrap_or_default() * rate {
          item_fee = Decimal::from_f64(fee.maximum_fee).unwrap_or_default() * rate;
        }
      }
      // 费用明细
      match &fee.fee_type as &str {
        "1" => {
            fee_detail.fee_jy = item_fee; //交易费
        },
        "2" => {
            fee_detail.fee_yh = item_fee; //印花税
        },
        "3" => {
            fee_detail.fee_gh = item_fee; //过户费
        },
        "4" => {
            fee_detail.fee_yj = item_fee; //佣金
        },
        "5" => {
            fee_detail.fee_js = item_fee; //经手费
        },
        "6" => {
            fee_detail.fee_zg = item_fee; //证管费
        },
        "7" => {
            fee_detail.fee_qt = item_fee; //其他费用
        },
        // "9" => {
        //     fee_detail.fee_fx = item_fee; //风险金
        // },
        "a" => {
            fee_detail.fee_js2 = item_fee; //结算费
        },
        "b" => {
            fee_detail.fee_jg = item_fee; //交割费
        },
        // "c" => {
        //     fee_detail.fee_jy = item_fee; //实际佣金
        // },
        // "d" => {
        //     fee_detail.fee_jy = item_fee; //券商佣金
        // },
        _ => {},
      }
    }
    fee_detail.fee_total = fee_detail.fee_jy + fee_detail.fee_yh + fee_detail.fee_gh + fee_detail.fee_yj + fee_detail.fee_js + 
    fee_detail.fee_zg + fee_detail.fee_qt + fee_detail.fee_js2 + fee_detail.fee_jg;
    log::info!("fee_total: {}", fee_detail.fee_total);
    Ok(())
  }

  pub async fn get_rate(currency_direction: &String, redis_client: &RedisClient) -> Result<Decimal> {
    log::info!("currency_direction: {}", currency_direction);
    let ret = RateType::from_str(currency_direction);
    if ret.as_ref().is_err() {
      log::error!("获取汇率失败: {}", ret.as_ref().err().unwrap().to_string());
      return Err(anyhow!("获取汇率失败: {:?}", ret.as_ref().err().unwrap().to_string()));
    }
    let rate_type = ret.unwrap();
    let rate_key = get_rate_key(&rate_type);
    let rate_str = redis_client.get_value_by_get(rate_key).await;
    //取第一个数据
    // 1.3|1670465193
    log::info!("rate string: {}", rate_str);
    let rate_v: Vec<&str> = rate_str.split('|').collect();
    if !rate_v.is_empty() {
      if rate_key == "RATE_CNY_HKD_STOCK_BUY" || rate_key == "RATE_USD_HKD_STOCK_BUY" {
        let rate = dec!(1.0) / Decimal::from_str(rate_v[0]).unwrap_or_default();
        log::info!("rate: {}", rate);
        return Ok(rate)
      }
      log::info!("汇率: {}", rate_v[0]);
      return Ok(Decimal::from_str(rate_v[0]).unwrap_or_default());
    }
    Ok(dec!(1.0))
  }
}
