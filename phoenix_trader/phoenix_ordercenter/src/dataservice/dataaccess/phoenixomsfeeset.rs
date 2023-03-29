use anyhow::{anyhow, Result};
use sea_orm::entity::prelude::*;
use sea_orm::{QueryFilter, QueryOrder, QuerySelect};
use crate::dataservice::entities::{prelude::*, *};
use crate::client::dbclient::DbClient;
use crate::common::common::OrderDetail;

//费用配置表
impl PhoenixOmsFeeset {
    pub fn new() -> Self {
        PhoenixOmsFeeset::default()
    }

    pub async fn query_feeset(detail: &OrderDetail, db: &DbClient) ->Result<PhoenixOmsFeeset>{
        let db = db.get_conn();
        match PhoenixOmsFeesetEntity::find()
        .filter(phoenix_oms_feeset::Column::UnitId.is_in(vec![detail.unit_id, -1]))
        .filter(phoenix_oms_feeset::Column::ExchangeId.is_in(vec![detail.exchange_id, 0]))
        .filter(phoenix_oms_feeset::Column::OrderDirection.is_in(vec![detail.order_direction, 0]))
        .filter(phoenix_oms_feeset::Column::FeeType.eq(detail.fee_type.to_owned()))
        .filter(phoenix_oms_feeset::Column::ChannelId.is_in(vec![detail.channel_id, 0]))
        .filter(phoenix_oms_feeset::Column::StockType.is_in(vec![detail.stock_type, 0]))
        .order_by_desc(phoenix_oms_feeset::Column::UnitId)
        .order_by_desc(phoenix_oms_feeset::Column::OrderDirection)
        .order_by_desc(phoenix_oms_feeset::Column::ExchangeId)
        .order_by_desc(phoenix_oms_feeset::Column::ChannelId)
        .order_by_desc(phoenix_oms_feeset::Column::StockType)
        .limit(1)
        .one(db)
        .await {
            Ok(model) => {
                if let Some(val) = model {
                    return Ok(val);
                } else {
                    return Err(anyhow!("未找的费用配置信息"));
                }
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err));
            }
        };
    }
}

