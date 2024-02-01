use crate::basicdata::marketclosetimecache::{CloseTimeCached, CloseTimeTemp};
use crate::client::AccountRiskClient;
use crate::config::settings::Settings;
use crate::protofiles::account_risk_center_client::AccountRiskCenterClient;
use crate::protofiles::account_risk_center_server::AccountRiskCenterServer;
use crate::protofiles::{
  MarginRatioReq, MarginRatioResp, PhoenixRiskCheckInfo, PhoenixRiskCheckResponse, PhoenixStockPositionRequest, PhoenixStockPositionResponse, PhoenixStockPositions, UserAssetReq, UserAssetResp,
};
use anyhow::Result;
use common::akaclient::AkaClient;
use common::constant;
use common::protofiles::phoenixakacenter::{
  ChannelConfig, ChannelHoldLimit, ChannelHoldLimitReq, ChannelHoldLimitResp, ChannelInfo, ChannelInfoReq, ChannelInfoResp, ExchangeRateReq, ExchangeRateResp, MarketCloseInfo, MarketInfo,
  MarketInfoReq, MarketInfoResp, StockChannelReq, StockChannelResp, StockInfo, StockInfoReq,
};
use core::option::Option;
use futures::FutureExt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use tonic::{self, Status};
use utility::{errors, errors::ErrorCode};
#[derive(Clone)]
pub struct OrderController {
  adminserver: String,
}

impl OrderController {
  pub fn new(settings: &Settings, ttl_long: u64) -> Self {
    OrderController {
      adminserver: settings.system.adminserver.to_string(),
    }
  }

  //买单处理逻辑:
  //1. 检查账户状态是否有效,如果无效则返回
  //2. 检查商品是否有效,无效则返回
  //3. 然后根据用户下单的商品和用户id,判断下单通道。判断规则:
  // 1. 根据用户ID和商品ID从接口获取所有的可用通道
  // 2. 判断用户的通道规则,按照默认通道还是按照用户自定义通道,
  // 3. 如果是用户通道优先,则查找该用户的配置的通道信息,没有则按照默认通用通道。
  //       如果给该用户配置过通道,则以该用户配置的通道为最高优先级,如果全部关闭了(禁止交易）,则不允许下单；
  //       如果配置了多个通道,其中一个关闭了,还有一个没关闭,则认为该用户只允许下该未关闭通道的订单。
  // 4. 如果是默认通用通道,则查找用户的通用通道配置;
  // 5. 如果通道状态已经关闭,则不使用该通道
  pub async fn phoenix_handle_buy_order(
    &mut self,
    order: &mut PhoenixRiskCheckInfo,
    account_info: &UserAssetResp,
    market_info: &MarketInfo,
    aka_client: &AkaClient,
    account_risk_client: &mut AccountRiskClient,
  ) -> Result<PhoenixRiskCheckResponse, Status> {
    log::info!("处理买单:{:?}", &order);
    let mut resp = PhoenixRiskCheckResponse {
      ret_code: errors::get_error_code(ErrorCode::CodeOk).0,
      ret_msg: errors::get_error_code(ErrorCode::CodeOk).1,
      retinfo: Vec::new(),
    };

    /* 3. 根据用户unit_id和商品ID,从接口获取所有的通道信息*/
    let stock_channel_req = StockChannelReq {
      unit_id: order.unit_id,
      stock_id: order.stock_id,
      order_direction: order.order_direction,
    };
    let channel_configs_result = aka_client.query_stock_channel(stock_channel_req).await;
    if channel_configs_result.as_ref().is_err() {
      log::error!(
        "不能根据用户id:{}和品种ID:{}找到相应的通道配置信息:{:?}",
        order.unit_id,
        order.stock_id,
        channel_configs_result.as_ref().unwrap()
      );
      resp.ret_code = errors::get_error_code(ErrorCode::CodeCommidityChannelNotConfigured).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeCommidityChannelNotConfigured).1;
      return Ok(resp);
    }
    let channel_configs = channel_configs_result.unwrap();

    //// 如果没有配置该品种的通道信息,则不能下买单
    if channel_configs.len() <= 0 && order.order_direction == constant::OrderDirection::BUY as i32 {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeCommidityChannelNotConfigured).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeCommidityChannelNotConfigured).1;
      log::error!("不能根据用户id:{}和品种ID:{}找到相应的通道配置信息,不能下买单", order.unit_id, order.stock_id);
      return Ok(resp);
    }

    let mut available_channels: Vec<ChannelConfig>;

    //根据用户ID拆分所有可用通道
    let (user_channels, group_channels): (Vec<ChannelConfig>, Vec<ChannelConfig>) = channel_configs.into_iter().partition(|v| v.unit_id > 0);
    if user_channels.len() > 0 {
      available_channels = user_channels;
    } else {
      available_channels = group_channels;
    }
    available_channels.sort_by(|a, b| a.channel_level.cmp(&b.channel_level));

    let ret = aka_client.query_channel_hold_limit(order.stock_id, 0).await;
    if ret.as_ref().is_err() {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeCommidityChannelNotConfigured).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeCommidityChannelNotConfigured).1;
      log::error!("获取品种通道的最大量错误:{}", ret.as_ref().err().unwrap());
      return Ok(resp);
    }

    //品种通道的最大持仓量信息
    let mut commidity_channel_data = ret.as_ref().unwrap().1.clone();
    let mut total_max_hold = ret.as_ref().unwrap().0;
    let mut new_total: i64 = 0;
    for val in commidity_channel_data.iter() {
      new_total += val.max_holdnum;
    }

    if new_total < total_max_hold {
      total_max_hold = new_total;
    }

    //query current positions
    let pos_req = PhoenixStockPositionRequest {
      stock_id: order.stock_id as i64,
      channel_id: 0 as i64,
    };

    let ret = account_risk_client.query_stock_positions(&pos_req).await;
    if ret.as_ref().is_err() {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeCommidityChannelNotConfigured).0;
      resp.ret_msg = format!("获取品种的当前持仓量错误:{}", ret.as_ref().err().unwrap());
      log::error!("获取品种的当前持仓量错误");
      return Ok(resp);
    }
    //股票得所有持仓数据
    let stock_positions = ret.unwrap().data;
    // log::info!("该股票得持仓数据:{:?}", &stock_positions);

    //根据优先级查找相应的通道
    // let mut matched_channel: Option<&ChannelConfigurations> = None;
    log::info!("买单通道匹配顺序:{:?}", &available_channels);

    // 1) 先判断有没有全部关闭得通道
    let closed_channel: Vec<&ChannelConfig> = available_channels
      // .clone()
      .iter()
      .filter(|c| c.channel_id == constant::VALUE_ALL)
      .collect();

    log::info!("通道ID为0(全部通道)得数量:{}", &closed_channel.len());
    for chn in closed_channel {
      if chn.channel_status == (constant::ChannelStatus::OFF as i32) || chn.channel_status == (constant::ChannelStatus::CLOSED as i32) || chn.channel_status == (constant::ChannelStatus::SELL as i32) {
        //如果是所有通道 都禁止交易
        resp.ret_code = errors::get_error_code(ErrorCode::CodeChannelClosed).0;
        resp.ret_msg = errors::get_error_code(ErrorCode::CodeChannelClosed).1;
        log::error!("该账户【{}】的所有交易通道已经被关闭,禁止交易", account_info.unit_id);
        return Ok(resp);
      } else {
        continue; //如果是所有通道,但是状态又不是禁止交易,则认为无效
      }
    }
    let mut order_results: Vec<PhoenixRiskCheckInfo> = Vec::new();

    // 2) 根据通道拆分订单

    //总下单量
    let mut remain_order_amount = order.order_amount as i64;

    for chn in available_channels.iter() {
      if chn.channel_status == (constant::ChannelStatus::NORMAL as i32) || chn.channel_status == (constant::ChannelStatus::BUY as i32) {
        let ret = aka_client.query_channel_info(chn.channel_id).await;
        if ret.as_ref().is_err() {
          log::error!("不能找到通道ID为{}的通道信息", chn.channel_id);
          continue;
        }
        let chninfo2 = ret.unwrap();
        if chninfo2.channel_id == 0 {
          log::error!("不能找到通道ID为{}的通道信息", chn.channel_id);
          continue;
        }
        if chninfo2.channel_id == -1
          || chninfo2.channel_state == (constant::ChannelStatus::OFF as i32)
          || chninfo2.channel_state == (constant::ChannelStatus::CLOSED as i32)
          || chninfo2.channel_state == (constant::ChannelStatus::SELL as i32)
        {
          log::info!("通道id:{}得通道不存在或者不能下卖单", chninfo2.channel_id);
          continue;
        }
        //这里判断是否休市时间
        log::info!("start to check channel qfstate. channel:{:?}", &chninfo2);
        let close_time_cache = CloseTimeCached::new();
        let is_closed = close_time_cache
          .check_currenttime_is_closetime(aka_client, market_info.market_id, market_info.market_type as i32, chninfo2.qfii_state)
          .await;
        if is_closed {
          //如果是休市时间,则继续下一个通道
          log::info!("channel id: {} in in close time", &chninfo2.channel_id);
          continue;
        }
        //获取品种剩余最大持仓量
        let commidity_available_amount = OrderController::check_commidity_available_amounts(order.stock_id as i64, &stock_positions, total_max_hold);
        if remain_order_amount > commidity_available_amount {
          log::error!("下单量+当前股票全部持仓量超过整体最大持仓量，不能下单");
          resp.ret_code = errors::get_error_code(ErrorCode::CodeChannelClosed).0;
          resp.ret_msg = errors::get_error_code(ErrorCode::CodeChannelClosed).1;
          log::error!("该股票持仓量超过限制,禁止交易");
          return Ok(resp);
        }

        //拆单已经完成
        if remain_order_amount <= 0 {
          break;
        }

        //先生成一个订单,数量为0
        let mut order_detail: PhoenixRiskCheckInfo = PhoenixRiskCheckInfo {
          unit_id: order.unit_id,
          stock_id: order.stock_id,
          order_price: order.order_price,
          order_amount: 0,
          order_direction: order.order_direction,
          order_type: order.order_type,
          order_channel: chninfo2.channel_id,
          market_id: order.market_id,
          trade_mode: order.trade_mode,
          channel_type: chninfo2.channel_type,
          agent_account: order.agent_account,
        };
        //判断该通道是否内盘，是的话，则全部交易到内盘，忽略后面的通道
        // if chninfo2.channel_type == constant::ChannelType::INTERNAL as i32 {
        //   //是内盘
        //   order_detail.order_amount = remain_order_amount as i32;
        //   order_detail.order_channel = chninfo2.channel_id;
        //   order_detail.channel_type = chninfo2.channel_type;
        //   remain_order_amount = 0;
        // } else {
        //判断下单量
        //获取该通道剩余得可用交易量
        let available_amount = OrderController::check_commidity_channel_max_value_exceeded(order.stock_id, chninfo2.channel_id, &commidity_channel_data, &stock_positions);
        //比如可用数量是30000，剩余下单量是1000
        if remain_order_amount < available_amount {
          //该通道可以下完全部
          order_detail.order_amount = remain_order_amount as i32;
          // order_detail.order_channel = chninfo2.channel_id as i32;
          // order_detail.order_channel_type = chninfo2.channel_type;
          remain_order_amount = 0;
        } else {
          order_detail.order_amount = available_amount as i32;
          // order_detail.order_channel = chninfo2.channel_id as i32;
          // order_detail.order_channel_type = chninfo2.channel_type;
          remain_order_amount -= available_amount; //此时total_order_amount变成-29000，表示全部拆分完成
        }

        order_results.push(order_detail);
      }
    }
    // if commidity_channel_data.total_max_hold > 0 {

    if order_results.len() > 0 && remain_order_amount <= 0 {
      log::info!("卖单拆分完成:{:?}", &order_results);
      resp.ret_code = errors::get_error_code(ErrorCode::CodeOk).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeOk).1;
      resp.retinfo = order_results.to_owned();
      return Ok(resp);
    }

    log::error!("找不到能满足订单{:?}下单要求的通道", &order);
    resp.ret_code = errors::get_error_code(ErrorCode::CodeChannelClosed).0;
    resp.ret_msg = errors::get_error_code(ErrorCode::CodeChannelClosed).1;
    return Ok(resp);
  }

  //卖单处理逻辑:
  // 1. 收到用户卖单,根据卖单信息:unit_id和stock_id到持仓表(tstockposition)里查找所有的持仓信息,
  //      可交易量:current_amount-frozen_amount-tmp_frozen_amount
  // 2. 根据用户ID和商品ID获取所有的通道配置信息
  // 3. 根据通道的反向优先级处理订单
  //  // 1. 如果下单量超过正常通道的可下单量,则报废单
  //  // 2. 如果卖单量超过某个通道的持仓量,则生成多笔订单返回给委托服务。
  //  // 3. 只有一笔持仓,则判断该笔持仓的通道状态
  // 注意,需要拆零碎股
  pub async fn phoenix_handle_sell_order(
    &mut self,
    order: &mut PhoenixRiskCheckInfo,
    account_info: &UserAssetResp,
    market_info: &MarketInfo,
    aka_client: &AkaClient,
    goods_info: &StockInfo,
    account_risk_client: &mut AccountRiskClient,
    counter_party: bool,
  ) -> Result<PhoenixRiskCheckResponse, Status> {
    log::info!("处理卖单:{:?}", order);
    let mut resp = PhoenixRiskCheckResponse {
      ret_code: errors::get_error_code(ErrorCode::CodeOk).0,
      ret_msg: errors::get_error_code(ErrorCode::CodeOk).1,
      retinfo: Vec::new(),
    };
    let stock_channel_req = StockChannelReq {
      unit_id: order.unit_id,
      stock_id: order.stock_id,
      order_direction: order.order_direction,
    };
    let channel_configs_result = aka_client.query_stock_channel(stock_channel_req).await;
    if channel_configs_result.as_ref().is_err() {
      log::error!(
        "不能根据用户id:{}和品种ID:{}找到相应的通道配置信息:{:?}",
        order.unit_id,
        order.stock_id,
        channel_configs_result.as_ref().unwrap()
      );
      resp.ret_code = errors::get_error_code(ErrorCode::CodeCommidityChannelNotConfigured).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeCommidityChannelNotConfigured).1;
      return Ok(resp);
    }
    let channel_configs = channel_configs_result.unwrap();

    let mut available_channels: Vec<ChannelConfig> = Vec::new();
    //根据用户ID拆分所有可用通道
    //客户通道按优先级优先，在客户通道里存在的产品组通道过滤掉再排序 ab，bc=abc
    let (mut user_channels, group_channels): (Vec<ChannelConfig>, Vec<ChannelConfig>) = channel_configs.into_iter().partition(|v| v.unit_id > 0);
    if user_channels.len() > 0 {
      available_channels = user_channels;
    } else {
      available_channels = group_channels;
    }
    available_channels.sort_by(|a, b| b.channel_level.cmp(&a.channel_level));
    // let exist_channels: Vec<i64> = available_channels.iter().map(|v| v.channel_id).collect();

    /* 1. 查找用户持仓,如果没有持仓,则直接返回*/
    // let mut user_asset_req = UserAssetReq::default();
    // user_asset_req.unit_id = order.unit_id;
    // let ret = account_risk_client.query_user_asset(&user_asset_req).await;
    // if ret.as_ref().is_err() {
    //   resp.ret_code = errors::get_error_code(ErrorCode::CodeAccountNoStockPosition).0;
    //   resp.ret_msg = errors::get_error_code(ErrorCode::CodeAccountNoStockPosition).1;
    //   return Ok(resp);
    // }
    //所有得持仓数据
    let user_positions = account_info.clone().positions;

    //没有持仓
    if user_positions.len() <= 0 {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeAccountNoStockPosition).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeAccountNoStockPosition).1;
      return Ok(resp);
    }
    //卖单量
    let initial_amount = order.order_amount;
    // log::info!("该股票的最小交易量:{},下单量:{}", goods_info.min_value, initial_amount);

    let mut available_total_amount = 0;
    for stc in user_positions.iter() {
      log::info!("该用户持仓数据,持仓量:{},冻结量:{}", stc.amount, stc.frozen_amount,);
      //计算用户可交易量
      available_total_amount += stc.amount - stc.frozen_amount;
    }
    //没有可交易量
    if available_total_amount <= 0 {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeAccountNoStockPosition).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeAccountNoStockPosition).1;
      return Ok(resp);
    }

    log::info!(
      "开始处理卖单,持仓条数:{},用户可交易量:{},最小交易量:{},下单量:{}",
      &user_positions.len(),
      &available_total_amount,
      goods_info.min_value,
      initial_amount
    );

    //下单量小于最小值,但是持仓量超过最小值,则报错
    if available_total_amount > goods_info.min_value as i64 && initial_amount < goods_info.min_value {
      log::error!("持仓量超过最小值,但是下单量小于最小值,不能下单");
      resp.ret_code = errors::get_error_code(ErrorCode::CodeOrderErrorNumber).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeOrderErrorNumber).1;
      return Ok(resp);
    }

    //报单量超过实际可卖量
    if order.order_amount > available_total_amount as i32 {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeAccountExceedPosition).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeAccountExceedPosition).1;
      return Ok(resp);
    }

    /* 2. 根据用户配置,获取所有的通道信息*/
    log::info!("卖单通道优先级匹配顺序:{:?}", &available_channels);

    /*3. 根据通道规则去拆单*/
    /* 3.1 先检查有没有被关闭 通道ID为0,状态为关闭 */
    for chn in available_channels.iter() {
      if chn.channel_id == constant::VALUE_ALL
        && (chn.channel_status == (constant::ChannelStatus::CLOSED as i32)
          || chn.channel_status == (constant::ChannelStatus::OFF as i32)
          || chn.channel_status == (constant::ChannelStatus::BUY as i32))
      {
        resp.ret_code = errors::get_error_code(ErrorCode::CodeChannelClosed).0;
        resp.ret_msg = errors::get_error_code(ErrorCode::CodeChannelClosed).1;

        return Ok(resp);
      }
    }
    //查询分账户通道
    let mut stock_positions_req = PhoenixStockPositionRequest::default();
    stock_positions_req.stock_id = order.stock_id;
    stock_positions_req.channel_id = constant::VALUE_ALL;
    let ret = account_risk_client.query_stock_positions(&stock_positions_req).await;
    if ret.as_ref().is_err() {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeAccountNoStockPosition).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeAccountNoStockPosition).1;
      return Ok(resp);
    }
    let stpositions = ret.unwrap();
    let mut stock_positions = stpositions.data;
    let user_total_positions = stpositions.user_total_positions;
    log::info!("分账户通道持仓信息:{:?}", &stock_positions);

    /*交易品种 和 通道关系的状态不再考虑, 只考虑通道基本信息的状态 */

    let mut sorted_stock_positions: Vec<PhoenixStockPositions> = Vec::new();
    // let mut other_stock_positions: Vec<StockPosition> = Vec::new();
    // 首先查找移除通道,就是持仓通道不能从通道配置信息中找到的
    let (mut remove_sp, mut exist_sp): (Vec<PhoenixStockPositions>, Vec<PhoenixStockPositions>) =
      stock_positions.into_iter().partition(|sp| available_channels.iter().find(|&c| c.channel_id == sp.channel_id).is_none());

    log::info!("拆分后的订单内容,通道配置移除的订单:{:?},未移除订单:{:?}", &remove_sp, &exist_sp);

    // 从配置的通道已经移除的持仓信息中，除去通道类型内盘（2）的所有持仓信息，优先处理。
    let mut removed_internal_positions: Vec<PhoenixStockPositions> = Vec::new();
    // let mut removed_external_positions: Vec<PhoenixStockPositions> = Vec::new();

    for val in remove_sp.into_iter() {
      let chn_info_ret = aka_client.query_channel_info(val.channel_id).await;
      if chn_info_ret.as_ref().is_err() {
        log::error!("不能找到通道ID为{}的通道信息", val.channel_id);
        continue;
      }
      let chn_info = chn_info_ret.unwrap();
      if chn_info.channel_type == constant::ChannelType::INTERNAL as i32 {
        removed_internal_positions.push(val.to_owned());
      }
    }

    // log::info!("拆分后的订单内容,通道被移除的持仓去掉虚拟通道持仓的剩余持仓:{:?}", &remove_sp);
    //通道已经移除的持仓以及不是虚拟通道的持仓，按照持仓量排序（由大到小）
    // removed_internal_positions.sort_by(|a, b| b.l_current_amount.cmp(&a.l_current_amount)); //理论上只有一条数据

    exist_sp.sort_by(|a, b| {
      let channel_a = available_channels
        .iter()
        .find(|&x| x.channel_id == a.channel_id)
        .map_or(ChannelConfig { ..Default::default() }, |f| f.to_owned());
      let channel_b = available_channels
        .iter()
        .find(|&x| x.channel_id == b.channel_id)
        .map_or(ChannelConfig { ..Default::default() }, |f| f.to_owned());
      channel_b.channel_level.cmp(&channel_a.channel_level)
    });
    removed_internal_positions.sort_by(|a, b| b.current_amount.cmp(&a.current_amount));
    //合并成一个持仓数据（排序后）
    // sorted_stock_positions.extend(removed_external_positions);
    sorted_stock_positions.extend(exist_sp);
    sorted_stock_positions.extend(removed_internal_positions);
    //通道未移除的，则按照通道优先级反向排序
    // available_channels.sort_by(|a, b| a.channel_level.cmp(&b.channel_level));
    // // 查找通道未被移除的订单,通道配置已经根据规定的顺序排序
    // for av_chn in available_channels.iter() {
    //   let sp = exist_sp.iter().find(|p| p.l_channel_id as i64 == av_chn.channel_id);
    //   if sp.is_some() && sorted_stock_positions.iter().find(|ss| ss.l_channel_id == sp.unwrap().l_channel_id).is_none() {
    //     sorted_stock_positions.push(sp.unwrap().clone());
    //   }
    // }

    log::info!("订单处理顺序:{:?}", sorted_stock_positions);
    let mut total_check_result: Vec<PhoenixRiskCheckInfo> = Vec::new();

    let hands_num = goods_info.hands_num.parse::<i32>().unwrap();
    // if goods_info.stock_type == constant::StockType::HSCY as i32 || goods_info.stock_type == constant::StockType::HSKC as i32 {
    //     log::info!("该股票是科创版,不需要处理零碎股,手数设置为1");
    //     hands_num = 1;
    // }
    //计算可用通道的分账户总持仓
    let total_hold_positions: i64 = sorted_stock_positions.iter().filter(|&f| f.channel_id != 7).map(|x| x.current_amount).sum();
    let total_frozen_amount: i64 = sorted_stock_positions.iter().filter(|&f| f.channel_id != 7).map(|x| x.frozen_amount).sum();
    let frozen_amount_temp: i64 = sorted_stock_positions.iter().filter(|&f| f.channel_id != 7).map(|x| x.frozen_amount_temp).sum();
    let stock_total_hold_positions = total_hold_positions - total_frozen_amount - frozen_amount_temp;
    log::info!("分账户所有通道current_amout总持仓:{},冻结总量:{},临时冻结总量:{},可用通道的分账户总持仓:{}", total_hold_positions, total_frozen_amount,frozen_amount_temp,stock_total_hold_positions);

    if stock_total_hold_positions <= 0 {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeAccountNoStockPosition).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeAccountNoStockPosition).1;
      return Ok(resp);
    }
    //报单量超过实际可卖量
    if order.order_amount > stock_total_hold_positions as i32 {
      resp.ret_code = errors::get_error_code(ErrorCode::CodeAccountExceedPosition).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeAccountExceedPosition).1;
      return Ok(resp);
    }
    //如果是虚拟通道，需要取该券用户总持仓和分账户总持仓的差值
    let mut hold_diff = stock_total_hold_positions - user_total_positions;
    if hold_diff < 0 {
      hold_diff = 0;
    }

    for od in sorted_stock_positions.into_iter() {
      //如果订单量小于等于0,则认为订单已经拆分完毕
      if order.order_amount <= 0 {
        break;
      }
      let chn_info_ret = aka_client.query_channel_info(od.channel_id).await;
      if chn_info_ret.as_ref().is_err() {
        log::error!("不能找到通道ID为{}的通道信息", od.channel_id);
        continue;
      }
      let chn_info = chn_info_ret.unwrap();
      log::info!("找到通道的基本信息(通道基本信息):{:?}", &chn_info);

      if chn_info.channel_state == (constant::ChannelStatus::CLOSED as i32)
        || chn_info.channel_state == (constant::ChannelStatus::OFF as i32)
        || chn_info.channel_state == (constant::ChannelStatus::BUY as i32)
      {
        log::info!("通道【{}】已经被关闭", od.channel_id);
        continue;
      }

      //这里判断是否休市时间
      log::info!("start to check channel qfstate. channel:{:?}", &chn_info);
      let close_time_cache = CloseTimeCached::new();
      let is_closed = close_time_cache
        .check_currenttime_is_closetime(aka_client, market_info.market_id, market_info.market_type as i32, od.is_qfii)
        .await;
      if is_closed {
        //如果是休市时间,则继续下一个通道
        log::info!("channel id: {} is in close time", &chn_info.channel_id);
        continue;
      }
      //判断是否qfii交易时间,是的话持仓可用用qfii持仓
      //   let uposition = user_positions.iter().find(|&f|f.stock_id ==od.stock_id);
      //   let qfii_amount = uposition.unwrap().qfii_amount;
      //   if market_info.market_id == constant::MarketCode::XHKG as i64 && od.is_qfii == constant::YesOrNo::YES as i32 && qfii_amount != 0 {
      //     available_total_amount = qfii_amount;
      //   }

      if od.current_amount <= 0 {
        log::info!("l_current_amount: {} is <= 0", od.current_amount);
        continue;
      }

      //如果订单量小于等于0,则认为订单已经拆分完毕
      if order.order_amount <= 0 {
        log::info!("order_amount: {} is <= 0", order.order_amount);
        break;
      }

      let mut channel_type = chn_info.channel_type;
      // if goods_info.stock_type == constant::StockType::HSCY as i32 || goods_info.stock_type == constant::StockType::HSKC as i32 {
      // if goods_info.stock_type == constant::YesOrNo::YES as i32 {
      //如果是科创版,卖单量超过最小限制,则不处理零碎股,如果小于最小限制,则直接内盘

      //如果下单量小于最小报单量,则直接到内盘成交
      if counter_party == false && initial_amount < goods_info.min_value {
        channel_type = 2;
      }

      //判断通道，如果是内盘通道，则全部往内盘发送
      //**********************
      total_check_result.extend(
        self
          .generate_order_check_result(&od, available_total_amount, hold_diff, hands_num, order, channel_type, &goods_info, counter_party)
          .iter()
          .cloned(),
      );
    }
    // let inter_channel = available_channels.iter().find(|ch| ch.channel_type == (constant::ChannelType::INTERNAL as i32));
    // //内盘成交判断
    // if order.order_amount > 0 && inter_channel.is_some() {
    //   for od in sorted_stock_positions.iter() {
    //     let chn_info = self.channels_controller.get_channel_by_id(od.l_channel_id as i64).await.get_or_insert(ChannelInfos::new()).clone();
    //     log::info!("找到通道的基本信息(通道基本信息):{:?}", &chn_info);

    //     //只有哪些移除或者被关闭的通道,才需要到内盘
    //     if chn_info.channel_id == -1
    //       || chn_info.channel_state == (constant::ChannelStatus::CLOSED as i32)
    //       || chn_info.channel_state == (constant::ChannelStatus::OFF as i32)
    //       || chn_info.channel_state == (constant::ChannelStatus::BUY as i32)
    //     {
    //       log::info!("通道【{}】已经被移除或者关闭,需要内盘成交", od.l_channel_id);
    //       //如果订单量小于等于0,则认为订单已经拆分完毕
    //       if order.order_amount <= 0 {
    //         break;
    //       }
    //       total_check_result.extend(
    //         self
    //           .generate_order_check_result(od, hands_num, account_info, order, constant::ChannelType::INTERNAL as i32, false, &goods_info)
    //           .iter()
    //           .cloned(),
    //       );
    //       // continue;
    //     }
    //   }
    // }

    //一个单子都未拆分
    if order.order_amount == initial_amount {
      // log::info!("剩余未拆下单量:{},不能下单", order.order_amount);
      resp.ret_code = errors::get_error_code(ErrorCode::CodeChannelClosed).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeChannelClosed).1;

      return Ok(resp);
    }
    if order.order_amount > 0 {
      let od_ret = PhoenixRiskCheckInfo {
        order_amount: order.order_amount,
        order_channel: 0,
        channel_type: 0,
        ..order.clone()
      };
      total_check_result.push(od_ret);
    }
    // // /* 3.4 再处理通道未变化得持仓*/
    if total_check_result.len() <= 0 {
      //拆单失败
      resp.ret_code = errors::get_error_code(ErrorCode::CodeChannelClosed).0;
      resp.ret_msg = errors::get_error_code(ErrorCode::CodeChannelClosed).1;

      return Ok(resp);
    }

    resp.retinfo = total_check_result;
    log::info!("处理卖单完成,结果:{:?}", &resp);
    Ok(resp)
  }

  fn generate_order_check_result(
    &self,
    sp: &PhoenixStockPositions,
    available_total_amount: i64, //用户持仓可用
    hold_diff: i64,              //用户总持仓和分账户持仓差值
    hands_num: i32,
    order: &mut PhoenixRiskCheckInfo,
    channeltype: i32,
    goods_info: &StockInfo,
    counter_party: bool,
  ) -> Vec<PhoenixRiskCheckInfo> {
    log::info!("开始处理订单:{:?}", sp);
    let mut check_result: Vec<PhoenixRiskCheckInfo> = Vec::new();

    let mut total_lots = 0;
    if channeltype == constant::ChannelType::INTERNAL as i32 {
      total_lots = hold_diff as i32;
    } else {
      total_lots = (sp.current_amount - sp.frozen_amount - sp.frozen_amount_temp) as i32;
    }
    // log::info!("持仓可用的交易量:{}", total_lots);
    log::info!("当前分账户通道持仓可用的交易量:{},未处理下单量:{}", total_lots, order.order_amount);
    let order_lots: i32; //下单量

    if order.order_amount <= total_lots {
      order_lots = order.order_amount;
      order.order_amount = 0;
    } else {
      order_lots = total_lots;
      order.order_amount -= total_lots; //总下单量减去分账户持仓的可用量
    }

    if counter_party {
      //如果是交易对手方,则不需要处理零碎股
      log::info!("处理对手方的订单信息,数量:{}", order_lots);
      let od_ret = PhoenixRiskCheckInfo {
        order_amount: order_lots,
        order_channel: sp.channel_id,
        channel_type: constant::ChannelType::EXTERNAL as i32,
        ..order.clone()
      };
      check_result.push(od_ret);
      return check_result;
    }

    //如果是自撮合通道,则不需要进行零碎股拆分
    if channeltype == (constant::ChannelType::INTERNAL as i32) {
      let od_ret = PhoenixRiskCheckInfo {
        order_amount: order_lots,
        order_channel: sp.channel_id as i64,
        channel_type: channeltype,
        ..order.clone()
      };
      if od_ret.order_amount != 0 {
        check_result.push(od_ret);
        return check_result;
      }
    }
    //   let exist = exist_channels.iter().find(|&&c| sp.channel_id == c);
    //   if exist.is_some() {
    //     //表示配置得通道存在
    //     od_ret.order_amount = order.order_amount + order_lots; //如果是内盘，则全部数量都往该通道发送
    //     order.order_amount = 0; //本次订单已经拆分完毕
    //   }
    //   check_result.push(od_ret);
    //   return check_result;
    // }

    let mut hands_num = hands_num;
    if sp.stock_type == constant::StockType::HSCY as i32 || sp.stock_type == constant::StockType::HSKC as i32 {
      log::info!("该持仓是科创版,不需要处理零碎股,手数设置为1");
      hands_num = 1;
    }

    // let mut odd_lots: i32 = 0;
    let odd_lots = order_lots % hands_num; //零碎股

    let mut channel_type = channeltype;
    if order_lots < goods_info.min_value {
      //如果报单量小于最小值,则进行内盘撮合
      log::info!("该下单量小于最小下单量,需要进行内盘撮合");
      channel_type = constant::ChannelType::INTERNAL as i32;
    }

    if order_lots >= hands_num {
      log::info!("处理整手的订单信息,数量:{}", order_lots - odd_lots);
      let od_ret = PhoenixRiskCheckInfo {
        order_amount: order_lots - odd_lots,
        order_channel: sp.channel_id,
        channel_type: channel_type,
        ..order.clone()
      };
      check_result.push(od_ret);
    }
    if odd_lots > 0 {
      //处理零碎股
      log::info!("处理零碎股,数量  :{}", odd_lots);
      let odd_ret = PhoenixRiskCheckInfo {
        order_amount: odd_lots,
        order_channel: sp.channel_id,
        channel_type: (constant::ChannelType::INTERNAL as i32),
        ..order.clone()
      };

      check_result.push(odd_ret);
    }
    log::info!("生成的订单信息:{:?}", check_result);
    check_result
  }

  fn check_commidity_available_amounts(stockid: i64, positions: &Vec<PhoenixStockPositions>,total_max_hold:i64) -> i64 {
    // let current_total_hold_positions: i64 = positions.iter().map(|x| x.amount).sum();
    // let current_prebuy_total: i64 = positions.iter().map(|x| x.prebuy_amount).sum();

    let (current_total_hold_positions, current_prebuy_total): (i64, i64) = positions
      .iter()
      .filter(|f| f.stock_id == stockid)
      .fold((0, 0), |(acc1, acc2), v| (acc1 + v.current_amount, acc2 + v.prebuy_amount));

    log::info!(
      "整体持仓量: stock id:{},  current amount:{}, prebuy amount:{}, max total allowed number:{}",
      stockid,
      current_total_hold_positions,
      current_prebuy_total,
      total_max_hold
    );
    return total_max_hold - (current_prebuy_total + current_total_hold_positions);
    //当前持仓数量+交易下单量+预买量大于最大允许的持仓量，则验证失败
    // if current_total_hold_positions + ordernumber + current_prebuy_total > commax.total_max_hold {
    //     return true;
    // } else {
    //     return false;
    // }
  }

  fn check_commidity_channel_max_value_exceeded(stockid: i64, channelid: i64, commax: &Vec<ChannelHoldLimit> , positions: &Vec<PhoenixStockPositions>) -> i64 {
    let ret = commax.iter().find(|&x| x.stock_id == stockid && x.channel_id == channelid);
    let commidity_channel_info: ChannelHoldLimit;
    if ret.is_none() {
      commidity_channel_info = ChannelHoldLimit::default();
    } else {
      commidity_channel_info = ret.unwrap().clone();
    }
    let mut cur_amount = 0;
    let mut pre_amount: i64 = 0;
    let pos = positions.iter().find(|&x| x.channel_id == channelid && x.stock_id == stockid);
    if pos.is_some() {
      cur_amount = pos.unwrap().current_amount;
      pre_amount = pos.unwrap().prebuy_amount;
    }
    log::info!(
      "通道持仓量:stock id:{}, channel id:{}, current amount:{}, prebuy amount:{}, max allowed number:{}",
      stockid,
      channelid,
      cur_amount,
      pre_amount,
      commidity_channel_info.max_holdnum
    );
    let available_amount = commidity_channel_info.max_holdnum - (cur_amount + pre_amount);
    if available_amount <= 0 {
      return 0;
    } else {
      return available_amount;
    }
    // //当前持仓数量+交易下单量大于最大允许的持仓量，则验证失败
    // if cur_amount + ordernumber + pre_amount > commidity_channel_info.max_holdnum {
    //   return true;
    // } else {
    //   return false;
    // }
  }
}
