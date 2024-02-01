use anyhow::{anyhow, Result};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Expr;
// use sea_orm::QueryFilter;
use crate::dataservice::entities::{prelude::*, *};
use crate::client::dbclient::DbClient;
use crate::common::common::OrderDetail;

//交易配置表
impl PhoenixOmsTradeconfig {
    pub fn new() -> Self {
        PhoenixOmsTradeconfig::default()
    }

    pub async fn query_trade_config(detail: &OrderDetail, db: &DbClient) -> Result<PhoenixOmsTradeconfig>{
        let db = db.get_conn();
        match PhoenixOmsTradeconfigEntity::find()
        .filter(phoenix_oms_tradeconfig::Column::ExchangeId.eq(detail.exchange_id))
        .filter(phoenix_oms_tradeconfig::Column::OrderDirection.eq(detail.order_direction))
        .one(db)
        .await {
            Ok(model) => {
                if let Some(val) = model {
                    return Ok(val);
                } else {
                    return Err(anyhow!("未找的交易配置信息: {}, {}", detail.exchange_id, detail.order_direction));
                }
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err));
            }
        };
    }

    /// 判断是否为有效的业务配置数据
    pub async fn isvalid_trade_config(trade_config: &PhoenixOmsTradeconfig) -> Result<()>{
        if trade_config.clear_speed.ne("0") && trade_config.clear_speed.ne("1") && trade_config.clear_speed.ne("2") && trade_config.clear_speed.ne("N") {
            log::error!("清算配置有误[0, T+1, T+2]: {}", trade_config.clear_speed);
        }
        if trade_config.asset_settle_date.ne(&0) && trade_config.asset_settle_date.ne(&1) {
            log::error!("无效的资产处理日期: {}", trade_config.asset_settle_date);
            return Err(anyhow!("无效的资产处理日期: {}", trade_config.asset_settle_date));
        }
        if trade_config.asset_settle_point.ne(&0) && trade_config.asset_settle_point.ne(&1) && trade_config.asset_settle_point.ne(&2) {
            log::error!("无效的资产处理时点: {}", trade_config.asset_settle_point);
            return Err(anyhow!("无效的资产处理时点: {}", trade_config.asset_settle_point));
        }
        if trade_config.market_cash_add_type.ne(&0) && trade_config.market_cash_add_type.ne(&1) && trade_config.market_cash_add_type.ne(&2) {
            log::error!("无效的本市场头寸增加方式: {}", trade_config.market_cash_add_type);
            return Err(anyhow!("无效的本市场头寸增加方式: {}", trade_config.market_cash_add_type));
        }
        //判断是否没有取到对应的配置
        if trade_config.clear_speed.eq("0") && trade_config.asset_settle_point.eq(&0) && trade_config.market_cash_add_type.eq(&0) {
            log::error!("找不到对应的业务配置: {:?}", trade_config);
            return Err(anyhow!("找不到对应的业务配置: {:?}", trade_config));
        }
        Ok(())
    }
}