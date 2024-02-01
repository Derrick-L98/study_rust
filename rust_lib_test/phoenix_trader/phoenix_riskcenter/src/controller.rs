use crate::basicdata::marketclosetimecache::CloseTimeCached;
use crate::basicdata::mintradeunit::Mintradeunit;
// use super::{dto::*, service::ordercontroller::OrderController};
use crate::client::{AccountRiskClient, HqCenterClient};
use crate::config::settings::Settings;
use crate::protofiles::{LastPriceMsgReq, MarginRatioReq, MarginRatioResp, PhoenixStockPositionRequest, PhoenixStockPositionResponse, UserAssetReq, UserAssetResp};
use crate::protofiles::{PhoenixRiskCheckInfo, PhoenixRiskCheckResponse};
use crate::protofiles::{PhoenixRiskRequest, PhoenixRiskResponse};
use crate::service::ordercontroller::OrderController;
use anyhow::{anyhow, Result};
use common::akaclient::AkaClient;
use common::constant;
use common::protofiles::phoenixakacenter::{
  ChannelConfig, ChannelHoldLimit, ChannelHoldLimitReq, ChannelHoldLimitResp, ChannelInfo, ChannelInfoReq, ChannelInfoResp, ExchangeRateReq, ExchangeRateResp, MarketCloseInfo, MarketInfo,
  MarketInfoReq, MarketInfoResp, StockChannelReq, StockChannelResp, StockInfo, StockInfoReq,
};
use rust_decimal::{prelude::*, Decimal};
use tonic::{self, Status};
use utility::{errors, errors::ErrorCode};
// #[derive(Clone)]
pub struct RiskCenterController {
  pub order_controller: OrderController,
  pub close_time_cache: CloseTimeCached,
  pub settings: Settings,
  pub account_risk_client: AccountRiskClient,
  pub akacenterconn: AkaClient,
  pub hq_center_client: HqCenterClient,
}
impl RiskCenterController {
  pub async fn phoenix_risk_check(&mut self, order: &mut PhoenixRiskCheckInfo) -> Result<PhoenixRiskCheckResponse, Status> {
    log::info!("收到订单信息:{:?}", order);
    let ret: Result<PhoenixRiskCheckResponse, Status>;
    let mut resp = PhoenixRiskCheckResponse {
      ret_code: errors::get_error_code(ErrorCode::CodeOk).0,
      ret_msg: errors::get_error_code(ErrorCode::CodeOk).1,
      retinfo: Vec::new(),
    };
    //根据市场ID，获取市场信息
    let marketinfo_ret = AkaClient::query_market_info(&self.akacenterconn, order.market_id).await;
    if marketinfo_ret.as_ref().is_err() {
      log::error!("获取市场信息信息error:{:?}", marketinfo_ret.as_ref().unwrap());
      resp.ret_code = errors::get_error_code(ErrorCode::CodeSystemErrRequest).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeSystemErrRequest).1;
      return Ok(resp);
    }
    let marketinfo = marketinfo_ret.unwrap();

    if marketinfo.market_id != order.market_id {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeSystemErrRequest).0;
      resp.ret_msg = format!("{},市场ID{}不存在", errors::get_error_code(ErrorCode::CodeSystemErrRequest).1, order.market_id);
      // log::error!("没有找到【{}】的商品信息", order.stock_id);
      return Ok(resp);
    }
    if marketinfo.date_type == 0 {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeMarketClosed).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeMarketClosed).1;
      log::error!("当前市场非交易日{}", order.market_id);
      return Ok(resp);
    }

    // order: &mut PhoenixRiskCheckInfo
    /* 1. 如果用户状态无效，则返回*/
    //帐号状态 '0:待配置, 1: 正常交易 ，2:禁止开仓， 3:禁止交易(只读)，4:账号冻结(禁用)'
    let mut user_asset_req = UserAssetReq::default();
    user_asset_req.unit_id = order.unit_id;
    let account_info = match self.account_risk_client.query_user_asset(&user_asset_req).await {
      Ok(account) => {
        if account.trade_state == (constant::AccountStatus::AccountNotReady as i32)
          || account.trade_state == (constant::AccountStatus::AccountFrozed as i32)
          || account.trade_state == (constant::AccountStatus::AccountOrderClosed as i32)
        {
          resp.ret_code = errors::get_error_code(ErrorCode::CodeAccountNottradable).0;
          resp.ret_msg = errors::get_error_code(ErrorCode::CodeAccountNottradable).1;
          log::error!("账户【{}】禁止交易，不能下单", order.unit_id);
          return Ok(resp);
        }
        account
      }
      Err(err) => {
        log::error!("query_user_asset error: {}", err);
        resp.ret_code = errors::get_error_code(ErrorCode::CodeAccountNotexist).0;
        resp.ret_msg = errors::get_error_code(ErrorCode::CodeAccountNotexist).1;
        log::error!("没有找到编号为【{}】的账号信息", order.unit_id);
        return Ok(resp);
      }
    };

    log::info!("接口返回的账户信息:{:?}", &account_info);
    let agentret = AkaClient::query_account_info(&self.akacenterconn, order.unit_id).await;
    if agentret.as_ref().is_err() {
      log::error!("查找托管账户信息错误:{:?}", agentret.as_ref().unwrap());
      resp.ret_code = errors::get_error_code(ErrorCode::CodeAgentAccountError).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeAgentAccountError).1;
      return Ok(resp);
    }
    let agentaccount = agentret.unwrap();

    //验证托管账户是否正确
    if order.trade_mode == constant::TradeMode::AGENT as i32 {
      if order.agent_account != agentaccount.agent_account {
        resp.ret_code = errors::get_error_code(ErrorCode::CodeAgentAccountError).0;
        resp.ret_msg = errors::get_error_code(ErrorCode::CodeAgentAccountError).1;
        log::error!("代理账户不匹配,权限不足不能下单:{}", agentaccount.unit_id);
        return Ok(resp);
      }
    }
    //判断风险率和预警线
    if account_info.risk_rate >= agentaccount.warning_line {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeAccountRiskRateHigh).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeAccountRiskRateHigh).1;
      log::error!("账户风险率已达到预警线{}", account_info.risk_rate);
      return Ok(resp);
    }
    //验证账户创业板限制
    let currentorder: f64 = order.order_amount as f64 * order.order_price;
    if (account_info.gem_position_value + currentorder) > (account_info.total_asset * agentaccount.gem_limit) {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeGemPositionError).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeGemPositionError).1;
      log::error!("本次下单已达创业板上限{}", agentaccount.unit_id);
      return Ok(resp);
    }
    //查询保证金比例
    let mut marginRatioReq = MarginRatioReq::default();
    marginRatioReq.unit_id = order.unit_id;
    marginRatioReq.stock_id = order.stock_id;
    let marginRatioret = self.account_risk_client.query_margin_ratio(&marginRatioReq).await;
    let margin = marginRatioret.unwrap();
    if (currentorder * margin.margin_ratio) > (account_info.real_cash - account_info.real_margin) {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeRealCashError).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeRealCashError).1;
      log::error!("可用资金不足{}", currentorder);
      return Ok(resp);
    }

    //验证unitid是否是交易对手方账号
    let mut counter_party = false;
    let accounttype = common::protofiles::phoenixakacenter::SpecialAccountType::Counterparty as i32;
    let counter_partys_ret = AkaClient::query_special_account(&self.akacenterconn, accounttype).await;
    if counter_partys_ret.as_ref().is_err() {
      log::error!("查找交易对手方信息错误:{:?}", counter_partys_ret.as_ref().unwrap());
      resp.ret_code = errors::get_error_code(ErrorCode::CodeSystemErrRequest).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeSystemErrRequest).1;
      return Ok(resp);
    }
    let counter_partys = counter_partys_ret.as_ref().unwrap();
    let counterparty = counter_partys.iter().find(|&f| f.unid_id == order.unit_id);
    if counterparty.is_some() {
      counter_party = true;
    }
    /* 2. 如果该商品禁止交易，则返回错误*/
    let mut commidity_req = StockInfoReq::default();
    commidity_req.stock_id = order.stock_id;
    let goods_info_ret = AkaClient::query_stock_info(&self.akacenterconn, order.stock_id).await;
    if goods_info_ret.as_ref().is_err() {
      log::error!("没有找到商品信息:{:?}", goods_info_ret.as_ref().unwrap());
      resp.ret_code = errors::get_error_code(ErrorCode::CodeCommidityNotExist).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeCommidityNotExist).1;
      return Ok(resp);
    }
    let goods_info = goods_info_ret.as_ref().unwrap();
    if goods_info.trade_state == constant::CommidityStatus::Closed as i32 {
      log::error!("商品【{}】禁止交易", order.stock_id);
      resp.ret_code = errors::get_error_code(ErrorCode::CodeCommidityChannelNotConfigured).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeCommidityChannelNotConfigured).1;
      return Ok(resp);
    }
    if goods_info.stock_id == 0 {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeCommidityNotExist).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeCommidityNotExist).1;
      log::error!("没有找到【{}】的商品信息", order.stock_id);
      return Ok(resp); //不能找到该商品
    }

    log::info!("接口返回的商品信息:{:?}", &goods_info);
    //取最新价判断下单加最小变动单位
    let mut pricereq = LastPriceMsgReq::default();
    pricereq.stock_code = goods_info.stock_code.to_owned();
    pricereq.exchange_id = order.market_id as i32;
    let lastprice = self.hq_center_client.get_last_price(&pricereq).await;
    if lastprice.as_ref().is_err() {
      log::error!("获取最新价错误:{}", lastprice.as_ref().err().unwrap());
    }
    let priceinfo = lastprice.unwrap().data;
    let price = priceinfo.unwrap();
    let is_tradeunit = Mintradeunit::IsCorrespondMinTradeUnit(price.last_price, order.order_price, goods_info.stock_type);
    if !is_tradeunit {
      //下单价最小变动单位错误
      resp.ret_code = errors::get_error_code(ErrorCode::CodeMinTradeUnitError).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeMinTradeUnitError).1;
      log::error!("下单价最小变动单位错误");
      return Ok(resp);
    }

    let min_number = match goods_info.min_value {
      0 => {
        if goods_info.stock_type == constant::StockType::HSKC as i32 || goods_info.stock_type == constant::StockType::HSCY as i32 {
          200
        } else if goods_info.stock_type == constant::StockType::GANGGU as i32 {
          //港股
          100
        } else if goods_info.stock_type == constant::StockType::MEIGU as i32 {
          1
        } else {
          100
        }
      }
      _ => goods_info.min_value,
    };

    let max_number = match goods_info.max_value {
      0 => {
        if goods_info.stock_type == constant::StockType::HSKC as i32 || goods_info.stock_type == constant::StockType::HSCY as i32 {
          100000
        } else if goods_info.stock_type == constant::StockType::GANGGU as i32 {
          2000000
        } else if goods_info.stock_type == constant::StockType::MEIGU as i32 {
          10000000
        } else {
          300000
        }
      }
      _ => goods_info.max_value,
    };

    let max_money: Decimal; // = Decimal::from(0);

    if goods_info.max_single_money == 0.0 {
      if goods_info.stock_type == constant::StockType::HSKC as i32 || goods_info.stock_type == constant::StockType::HSCY as i32 {
        max_money = Decimal::from(2000000 as i64);
      } else if goods_info.stock_type == constant::StockType::GANGGU as i32 {
        max_money = Decimal::from(20000000 as i64);
      } else if goods_info.stock_type == constant::StockType::MEIGU as i32 {
        max_money = Decimal::from(10000000 as i64);
      } else {
        max_money = Decimal::from(3000000 as i64);
      }
    } else {
      max_money = Decimal::from(goods_info.max_single_money as i64);
    }

    if let Err(e) = &goods_info.hands_num.parse::<i32>() {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeCommidityNotExist).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeCommidityNotExist).1;
      log::error!("商品信息【{}】数据错误，每手数量不是有效得整数:{:?}", order.stock_id, &e);
      return Ok(resp); //不能找到该商品
    }

    //非管理员平仓，需要判断最大量和最大金额，不管买卖
    if order.order_type != constant::OrderType::ADMINCLEAR2 as i32 && order.order_type != constant::OrderType::ADMINCLEAR as i32 {
      if order.order_amount > max_number {
        resp.ret_code = errors::get_error_code(ErrorCode::CodeOrderExceedMaxNumber).0;
        resp.ret_msg = format!("{},最大数量是:{}", errors::get_error_code(ErrorCode::CodeOrderExceedMaxNumber).1, max_number);
        // log::error!("没有找到【{}】的商品信息", order.stock_id);
        return Ok(resp); //不能找到该商品
      }
      let rate_info = AkaClient::query_exchange_rate(&self.akacenterconn, goods_info.trade_currency.to_string().as_str()).await;
      if rate_info.as_ref().is_err() {
        log::error!("查找汇率错误:{:?}", rate_info.as_ref().unwrap());
        resp.ret_code = errors::get_error_code(ErrorCode::CodeOrderExceedMaxMoney).0;
        resp.ret_msg = errors::get_error_code(ErrorCode::CodeOrderExceedMaxMoney).1;
        return Ok(resp);
      }
      let rateinfo = rate_info.unwrap();
      let mut rate = 0.0;
      if order.order_direction == constant::OrderDirection::BUY as i32 {
        rate = rateinfo.buy_rate;
      } else if order.order_direction == constant::OrderDirection::SELL as i32 {
        rate = rateinfo.sell_rate;
      }
      let rate = Decimal::from(0 as i64);
      let total_money = rate * Decimal::from_f64(order.order_price).unwrap_or_default() * Decimal::from_i32(order.order_amount).unwrap_or_default();
      if total_money > max_money {
        resp.ret_code = errors::get_error_code(ErrorCode::CodeOrderExceedMaxMoney).0;
        resp.ret_msg = format!("{},最大金额是:{}", errors::get_error_code(ErrorCode::CodeOrderExceedMaxMoney).1, max_money);
        // log::error!("没有找到【{}】的商品信息", order.stock_id);
        return Ok(resp); //不能找到该商品
      }
    }

    if order.order_direction == constant::OrderDirection::BUY as i32 {
      if order.order_amount < min_number {
        resp.ret_code = errors::get_error_code(ErrorCode::CodeOrderExceedMinNumber).0;
        resp.ret_msg = format!("{},最小数量是:{}", errors::get_error_code(ErrorCode::CodeOrderExceedMinNumber).1, min_number);
        return Ok(resp); //不能找到该商品
      }

      //处理买单
      if account_info.trade_state == (constant::AccountStatus::AccountBuyClosed as i32) {
        resp.ret_code = errors::get_error_code(ErrorCode::CodeAccountBuyClosed).0;
        resp.ret_msg = errors::get_error_code(ErrorCode::CodeAccountBuyClosed).1;
        log::error!("账户【{}】禁止开仓", order.unit_id);
        return Ok(resp);
      }

      if goods_info.trade_state == constant::CommidityStatus::SellOnly as i32 {
        log::error!("商品[{}]:[{}]只能平仓,不能下买单", order.stock_id, &goods_info.stock_name);
        resp.ret_code = errors::get_error_code(ErrorCode::CodeCommidityChannelNotConfigured).0;
        resp.ret_msg = errors::get_error_code(ErrorCode::CodeCommidityChannelNotConfigured).1;
        return Ok(resp);
      }
      ret = self
        .order_controller
        .phoenix_handle_buy_order(order, &account_info, &marketinfo, &self.akacenterconn, &mut self.account_risk_client)
        .await;
    } else {
      //处理卖单
      ret = self
        .order_controller
        .phoenix_handle_sell_order(order, &account_info, &marketinfo, &self.akacenterconn, &goods_info, &mut self.account_risk_client, counter_party)
        .await;
    }

    log::info!("====处理获取通道的请求完成,结果:{:?}", &ret);

    ret
  }

  pub async fn phoenix_risk_test(&mut self, _req: PhoenixRiskRequest) -> Result<PhoenixRiskResponse, Status> {
    let result = PhoenixRiskResponse {
      ret_code: 1111,
      ret_msg: String::from("aaa"),
    };
    log::info!("收到一个下单请求");
    std::thread::sleep(std::time::Duration::from_secs(2));

    Ok(result)
  }
}
