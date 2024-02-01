use anyhow::{anyhow, Result};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::{entity::*, Condition, QueryFilter, QueryOrder, QuerySelect};
use crate::dataservice::entities::{prelude::*, *};
use crate::client::dbclient::DbClient;

impl PhoenixOrdStockdeal {
    pub fn new() -> Self {
        PhoenixOrdStockdeal::default()
    }

    pub async fn insert(model: &PhoenixOrdStockdeal, db: &DbClient) -> Result<i64> {
        let db = db.get_conn();
        let info = PhoenixOrdStockdeal::convert_to_activemodel(model);

        let ret = PhoenixOrdStockdealEntity::insert(info).exec(db).await;
        if ret.as_ref().is_err() {
            log::error!("{:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!("{:?}", ret.as_ref().err().unwrap().to_string()));
        }

        let insert_result = ret.unwrap();
        Ok(insert_result.last_insert_id)
    }

    pub async fn save(model: &PhoenixOrdStockdeal, db: &DbClient) -> Result<()> {
        let conn = db.get_conn();
        let info = PhoenixOrdStockdeal::convert_to_activemodel(model);
        let ret = info.save(conn).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
        }
        if model.id > 0 {
            log::info!("成交更新: {:?}", ret.unwrap().try_into_model().unwrap_or_default());
        } else {
            log::info!("成交落库: {:?}", ret.unwrap().try_into_model().unwrap_or_default());
        }
        
        Ok(())
    }

    pub async fn cheak_order_deal(exec_no: &String, db: &DbClient) -> Result<()>{
        let db = db.get_conn();
        match PhoenixOrdStockdealEntity::find()
        .filter(phoenix_ord_stockdeal::Column::ExecNo.contains(exec_no))
        .one(db)
        .await {
            Ok(model) => {
                if let Some(val) = model {
                    return Err(anyhow!("成交已处理 stockdeal: {:?}", val));
                } else {
                    return Ok(());
                }
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err));
            }
        };
    }

    pub async fn delete() {}
    pub async fn update() {}
    pub async fn select(_db: &DbClient) {
        //  Query::select()
        // .columns([Char::Character, Char::SizeW, Char::SizeH])
        // .from(Char::Table)
        // .and_where(Expr::value(1))
        // .and_where(Expr::value(2.5))
        // .and_where(Expr::value("3"))
        // .to_owned();
    }


    // pub async fn query_order_deal()


    pub fn convert_to_activemodel(info: &PhoenixOrdStockdeal) -> phoenix_ord_stockdeal::ActiveModel {
        let mut retinfo = phoenix_ord_stockdeal::ActiveModel {
            id: NotSet,
            deal_no: Set(info.deal_no),
            sys_date: Set(info.sys_date),
            order_no: Set(info.order_no),
            exchange_id: Set(info.exchange_id),
            unit_id: Set(info.unit_id),
            stock_id: Set(info.stock_id),
            stock_code: Set(info.stock_code.to_owned()),
            order_direction: Set(info.order_direction),
            deal_time: Set(info.deal_time),
            deal_amount: Set(info.deal_amount),
            deal_price: Set(info.deal_price),
            fee_jy: Set(info.fee_jy),
            fee_gh: Set(info.fee_gh),
            fee_yj: Set(info.fee_yj),
            fee_js: Set(info.fee_js),
            fee_zg: Set(info.fee_zg),
            fee_qt: Set(info.fee_qt),
            fee_js2: Set(info.fee_js2),
            fee_jg: Set(info.fee_jg),
            fee_yh: Set(info.fee_yh),
            fee_real_yj: Set(info.fee_real_yj),
            fee_total: Set(info.fee_total),
            exec_no: Set(info.exec_no.to_owned()),
            channel_type: Set(info.channel_type),
            channel_id: Set(info.channel_id),
            refer_profit: Set(info.refer_profit),
            trade_type: Set(info.trade_type),
            create_time: Set(info.create_time),
        };
        
        if info.id > 0 {
            retinfo.id = Set(info.id);
        }
        // log::info!("active model:{:?}", &retinfo);
        retinfo
    }
}
