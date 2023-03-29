use anyhow::{anyhow, Result};
use sea_orm::entity::prelude::*;
use sea_orm::{entity::*, QueryFilter};
use crate::dataservice::entities::{prelude::*, *};
use crate::client::dbclient::DbClient;
use crate::common::common::OrderDetail;

impl PhoenixOrdPendSettle {
    pub fn new() -> Self {
        PhoenixOrdPendSettle::default()
    }

    pub async fn insert(model: &PhoenixOrdPendSettle, db: &DbClient) -> Result<i64> {
        let db = db.get_conn();
        let info = PhoenixOrdPendSettle::convert_to_activemodel(model);
        match PhoenixOrdPendSettleEntity::insert(info).exec(db).await {
            Ok(v) => {
                log::info!("交收id: {}", v.last_insert_id);
                return Ok(v.last_insert_id)
            }
            Err(err) => {
                log::error!("交收落库失败: {:?}", err);
                return Err(anyhow!("交收落库失败: {:?}", err));
            }
        }
    }

    pub async fn update(model: &PhoenixOrdPendSettle, db: &DbClient) -> Result<()>{
        let conn = db.get_conn();
        // let info = model.clone().into_active_model();
        let info = PhoenixOrdPendSettle::convert_to_activemodel(model);
        match info.update(conn).await {
            Ok(v) => {
                log::info!("更新成功: {:?}", v);
            }
            Err(err) => {
                log::error!("交收更新失败: {}", err);
                return Err(anyhow!("交收更新失败: {}", err));
            }
        }
        Ok(())
    }

    pub async fn save(model: &PhoenixOrdPendSettle, db: &DbClient) -> Result<()> {
        let db = db.get_conn();
        let info = PhoenixOrdPendSettle::convert_to_activemodel(&model);
        match info.save(db).await {
            Ok(v) => {
                if model.id > 0 {
                    log::info!("交收更新: {:?}", v.try_into_model())
                } else {
                    log::info!("交收落库: {:?}", v.try_into_model())
                }
            }
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err));
            }
        }
        Ok(())
    }

    pub async fn select(detail: &OrderDetail, db: &DbClient) -> Result<Option<PhoenixOrdPendSettle>> {
        let db = db.get_conn();
        match PhoenixOrdPendSettleEntity::find()
        .filter(phoenix_ord_pendsettle::Column::OrderNo.eq(detail.order_id))
        .filter(phoenix_ord_pendsettle::Column::ChannelId.eq(detail.channel_id))
        .filter(phoenix_ord_pendsettle::Column::UnitId.eq(detail.unit_id))
        .filter(phoenix_ord_pendsettle::Column::StockId.eq(detail.stock_id))
        // .filter(phoenix_ord_pendsettle::Column::TradeDate.eq(detail.last_deal_time))
        .filter(phoenix_ord_pendsettle::Column::Status.eq(0))//未处理
        .one(db)
        .await {
            Ok(model) => {
                return Ok(model);
                // if let Some(val) = model {
                //     return Ok(val);
                // } else {
                //     return Err(anyhow!("未找交收 {} 信息", order_id));
                // }
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err));
            }
        };
    }

    fn convert_to_activemodel(model: &PhoenixOrdPendSettle) -> phoenix_ord_pendsettle::ActiveModel {
        let mut retinfo = phoenix_ord_pendsettle::ActiveModel {
            id: NotSet,
            settle_id: Set(model.settle_id),
            unit_id: Set(model.unit_id),
            sys_date: Set(model.sys_date),
            trade_date: Set(model.trade_date),
            settle_date: Set(model.settle_date),
            clear_speed: Set(model.clear_speed),
            order_no: Set(model.order_no),
            order_amount: Set(model.order_amount),
            deal_avg_price: Set(model.deal_avg_price),
            exchange_id: Set(model.exchange_id),
            stock_code: Set(model.stock_code.to_owned()),
            stock_id: Set(model.stock_id),
            order_direction: Set(model.order_direction),
            settle_amount: Set(model.settle_amount),
            settle_internal_amount: Set(model.settle_internal_amount),
            settle_balance: Set(model.settle_balance),
            net_balance: Set(model.net_balance),
            fee_total: Set(model.fee_total),
            fee_jy: Set(model.fee_jy),
            fee_yh: Set(model.fee_yh),
            fee_gh: Set(model.fee_gh),
            fee_yj: Set(model.fee_yj),
            fee_js: Set(model.fee_js),
            fee_zg: Set(model.fee_zg),
            fee_other: Set(model.fee_other),
            fee_js2: Set(model.fee_js2),
            fee_jg: Set(model.fee_jg),
            asset_settle_date: Set(model.asset_settle_date),
            status: Set(model.status),
            currency_no: Set(model.currency_no.to_owned()),
            remark: Set(model.remark.to_owned()),
            currency_rate: Set(model.currency_rate),
            channel_id: Set(model.channel_id),
        };

        if model.id > 0 {
            retinfo.id = Set(model.id);
        }
        // log::info!("active model:{:?}", &retinfo);
        retinfo
    }
}