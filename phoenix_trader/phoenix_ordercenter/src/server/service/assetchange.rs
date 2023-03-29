use anyhow::Result;
use common::constant::{OrderDirection, AssetChangeType, AssetChangeDirection, StockType};
use rust_decimal::prelude::ToPrimitive;
// use rust_decimal_macros::dec;

use crate::common::common::OrderDetail;
use crate::client::AssetsCenterClient;
use crate::protofiles::{
    PhoenixassetscenterRequest, 
    PhoenixassetscenterRequestInfo, 
    PhoenixassetspostionrequestInfo, 
    PhoenixassetscenterQueryRequest, 
    Phoenixassetspostioninfo
};


// 1、买挂单  203：预买数量。 105：临时冻结（order传参）
// 2、买成交  102：费用减少，201：持仓增加（order传参），106：临时解冻 ,        预买减少，A股冻结：资产服务自己处理，
// 3、买撤单  203：预买数量减少  ，106：临时解冻
// 4、卖挂单  203：预卖数量增加，                                               持仓临时冻结增加，资产服务自己处理，
// 5、卖成交  101：实盈，102：费用，202：持仓减少                               预卖减少，持仓临时冻结减少
// 6、卖撤单  203：预卖减少  


//下单
pub async fn place_order_asset_change(detail: &OrderDetail, assetscenter_client: &AssetsCenterClient) -> Result<()> {
    let mut assets_req = PhoenixassetscenterRequest::default();
    let mut change_capital = PhoenixassetscenterRequestInfo::default();
    let mut change_postion = PhoenixassetspostionrequestInfo::default();
    assets_req.message_id = detail.msg_id;
    assets_req.operator_id = detail.operator_no;
    assets_req.business_flag = AssetChangeType::TradeOrderAsset as i32;
    assets_req.unit_id = detail.unit_id;
    change_postion.stock_id = detail.stock_id;

    if detail.order_direction == common::constant::OrderDirection::BUY as i32 {
        change_capital.op_type = AssetChangeDirection::FrozenTradeCapital as i32; //105：资产临时冻结
        if detail.stock_type == StockType::HSCY as i32 {//创业板
            change_capital.flag = 1;
        }
        change_capital.change_amount = detail.capital.to_f64().unwrap_or_default();//包含汇率
        change_capital.memo = "委托买下单,资产临时冻结".to_owned();
        assets_req.assets.push(change_capital);

        change_postion.op_type = AssetChangeDirection::FrozenPosition as i32; // 203
        change_postion.deal_amount = detail.deal_amount;//0
        change_postion.margin_rate = detail.margin_rate.to_f64().unwrap_or_default();
        change_postion.prebuy_amount = detail.order_amount;
        assets_req.postions.push(change_postion);
    } else {
        change_postion.op_type = AssetChangeDirection::FrozenPosition as i32; // 203：持仓冻结
        change_postion.presale_amount = detail.order_amount;
        assets_req.postions.push(change_postion);
    }
    log::info!("place_order_asset_change: {:#?}", &assets_req);
    let ret = assetscenter_client.phoenix_assets_change(&assets_req).await;
    if ret.as_ref().is_err() {
        log::error!("委托资产调整错误: {:?}", ret.as_ref().err().unwrap().to_string());
        return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
    }
    log::info!("订单{}交易资产变化: {:#?}", detail.order_id, ret.unwrap()); 
    Ok(())
}

//成交
pub async fn deal_order_asset_change(detail: &OrderDetail, assetscenter_client: &AssetsCenterClient) -> Result<()> {
    let mut assets_req = PhoenixassetscenterRequest::default();
    let mut change_capital = PhoenixassetscenterRequestInfo::default();
    let mut change_postion = PhoenixassetspostionrequestInfo::default();
    assets_req.message_id = detail.msg_id;
    assets_req.operator_id = detail.operator_no;
    assets_req.business_flag = AssetChangeType::TradeOrderAsset as i32;//交易产生的资产变动
    assets_req.unit_id = detail.unit_id;
    change_postion.stock_id = detail.stock_id;
    if detail.order_direction == OrderDirection::BUY as i32{
        log::info!("持仓增加, 资金减少");
        change_capital.op_type = AssetChangeDirection::ReduceCapital as i32; // 102：资金减少
        change_capital.change_amount = (detail.fee_total* detail.rate).to_f64().unwrap_or_default();//减少费用
        change_capital.memo = "买成交,扣减费用".to_owned();
        assets_req.assets.push(change_capital.to_owned());

        change_capital.op_type = AssetChangeDirection::UnFrozenTradeCapital as i32; //106：资产临时解冻
        if detail.stock_type == StockType::HSCY as i32 {
            change_capital.flag = 1;
        }
        change_capital.change_amount = detail.capital.to_f64().unwrap_or_default();
        change_capital.memo = "买成交,资产临时解冻".to_owned();
        assets_req.assets.push(change_capital.to_owned());

        change_postion.op_type = AssetChangeDirection::AddPosition as i32; // 201：持仓增加
        change_postion.deal_amount = detail.deal_amount;
        change_postion.margin_rate = detail.margin_rate.to_f64().unwrap_or_default();
        change_postion.fee_value = detail.fee_total.to_f64().unwrap_or_default();
        change_postion.deal_price = detail.deal_price.to_f64().unwrap_or_default();
        change_postion.deal_no = detail.deal_id;
        // change_postion.prebuy_amount = -detail.deal_amount;
        assets_req.postions.push(change_postion.clone());
    } else {
        log::info!("持仓减少, 资金增加");
        change_capital.op_type = AssetChangeDirection::AddCapital as i32; // 101: 资金增加
        change_capital.change_amount = (detail.profit * detail.rate).to_f64().unwrap_or_default();//增加 盈亏
        if detail.profit.to_f64().unwrap_or_default() < 0.0 {//亏损->减钱
            change_capital.op_type = AssetChangeDirection::ReduceCapital as i32; // 102: 资金减少
            change_capital.change_amount = (detail.profit * detail.rate).to_f64().unwrap_or_default().abs();//增加 盈亏
        }
        change_capital.memo = "卖成交,盈亏(加减?)".to_owned();
        assets_req.assets.push(change_capital.to_owned());

        change_capital.op_type = AssetChangeDirection::ReduceCapital as i32; // 101: 资金减少（费用）
        change_capital.change_amount = (detail.fee_total * detail.rate).to_f64().unwrap_or_default();
        change_capital.memo = "卖成交,扣减费用".to_owned();
        assets_req.assets.push(change_capital.to_owned());

        change_postion.op_type = AssetChangeDirection::ReducePosition as i32; // 202：持仓减少
        change_postion.deal_amount = detail.deal_amount;
        change_postion.margin_rate = detail.margin_rate.to_f64().unwrap_or_default();
        change_postion.fee_value = detail.fee_total.to_f64().unwrap_or_default();
        change_postion.deal_price = detail.deal_price.to_f64().unwrap_or_default();
        change_postion.deal_no = detail.deal_id;
        // change_postion.presale_amount = -detail.deal_amount;
        assets_req.postions.push(change_postion.clone());
    }
    log::info!("deal_order_asset_change: {:#?}", &assets_req);
    let ret = assetscenter_client.phoenix_assets_change(&assets_req).await;
    if ret.as_ref().is_err() {
        log::error!("成交资产调整错误: {:?}", ret.as_ref().err().unwrap().to_string());
        return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
    }
    log::info!("订单{}成交资产变化: {:#?}", detail.order_id, ret.unwrap()); 

    Ok(())
}

//撤单
pub async fn cancel_order_asset_change(detail: &OrderDetail, assetscenter_client: &AssetsCenterClient) -> Result<()>{
    let mut assets_req = PhoenixassetscenterRequest::default();
    let mut change_capital = PhoenixassetscenterRequestInfo::default();
    let mut change_postion = PhoenixassetspostionrequestInfo::default();
    assets_req.message_id = detail.msg_id;
    assets_req.operator_id = detail.operator_no;
    assets_req.business_flag = AssetChangeType::TradeOrderAsset as i32;//交易产生的资产变动
    assets_req.unit_id = detail.unit_id;
    change_postion.stock_id = detail.stock_id;

    if detail.order_direction == OrderDirection::BUY as i32 {
        change_capital.change_amount = detail.capital.to_f64().unwrap_or_default();
        change_capital.op_type = AssetChangeDirection::UnFrozenTradeCapital as i32; //106：资产临时解冻
        if detail.stock_type == StockType::HSCY as i32 {
            change_capital.flag = 1;
        }
        change_capital.memo = "买撤单,资产临时解冻".to_owned();
        assets_req.assets.push(change_capital.to_owned());

        change_postion.op_type = AssetChangeDirection::FrozenPosition as i32; // 203
        change_postion.margin_rate = detail.margin_rate.to_f64().unwrap_or_default();
        change_postion.prebuy_amount = -detail.cancel_amount;
        assets_req.postions.push(change_postion.clone());
    } else {
        change_postion.op_type = AssetChangeDirection::FrozenPosition as i32; // 203
        change_postion.margin_rate = detail.margin_rate.to_f64().unwrap_or_default();
        change_postion.presale_amount = -detail.cancel_amount;
        assets_req.postions.push(change_postion.clone());
    }
    log::info!("cancel_order_asset_change: {:#?}", &assets_req);
    let ret = assetscenter_client.phoenix_assets_change(&assets_req).await;
    if ret.as_ref().is_err() {
        log::error!("撤单资产调整错误: {:?}", ret.as_ref().err().unwrap().to_string());
        return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }
    log::info!("订单{}撤单资产变化: {:#?}", detail.order_id, ret.unwrap()); 
    Ok(())
}

pub async fn query_postion_info(unit_id: i64, stock_id: i64, assetscenter_client: &AssetsCenterClient) -> Result<Phoenixassetspostioninfo>{
    let mut req = PhoenixassetscenterQueryRequest::default();
    req.unit_id.push(unit_id);
    req.query_type = 2;
    log::info!("query_spostion_info: {:#?}", &req);
    let ret = assetscenter_client.phoenix_assets_query(&req).await;
    if ret.as_ref().is_err() {
        log::error!("查询用户{}持仓{}信息出错!: {:?}", unit_id, stock_id, ret.as_ref().err().unwrap().to_string());
        return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
    }

    let query_info = ret.unwrap();
    let ret = query_info.assetsinfo.iter().find(|x| x.unit_id == unit_id);
    if ret.is_none() {
        log::error!("查询用户{}持仓{}信息出错!: {:?}", unit_id, stock_id, query_info.assetsinfo);
        return Err(anyhow!("查询用户{}持仓{}信息出错!: {:?}", unit_id, stock_id, query_info.assetsinfo));
    }

    let info = ret.unwrap();
    let ret = info.positionsinfo.iter().find(|x| x.stock_id == stock_id);
    if ret.is_none() {
        log::error!("查询用户{}持仓{}信息出错!: {:?}", unit_id, stock_id, info.positionsinfo);
        return Err(anyhow!("查询用户{}持仓{}信息出错!: {:?}", unit_id, stock_id, info.positionsinfo));
    }
    log::info!("持仓: {:?}", ret.unwrap());
    Ok(ret.unwrap().to_owned())
}
