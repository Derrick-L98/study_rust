
use anyhow::{anyhow, Result};
use sea_orm::entity::prelude::*;
use sea_orm::{QueryFilter, QueryOrder, QuerySelect};
use crate::dataservice::entities::{prelude::*, *};
use crate::client::dbclient::DbClient;
use crate::common::common::OrderDetail;

impl PhoenixAstStockposition {
    pub async fn query_position(detail: &OrderDetail, db: &DbClient) -> Result<PhoenixAstStockposition>{
        let db = db.get_conn();
        match PhoenixAstStockpositionEntity::find()
        .filter(phoenix_ast_stockposition::Column::StockId.eq(detail.stock_id))
        .filter(phoenix_ast_stockposition::Column::UnitId.eq(detail.unit_id))
        // .filter(phoenix_ast_stockposition::Column::Chan.eq(detail.channel_id))
        .one(db)
        .await {
            Ok(model) => {
                if let Some(val) = model {
                    return Ok(val);
                } else {
                    return Err(anyhow!("未找的持仓信息: {}, {}", detail.unit_id, detail.stock_id));
                }
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err));
            }
        };
    }
}
