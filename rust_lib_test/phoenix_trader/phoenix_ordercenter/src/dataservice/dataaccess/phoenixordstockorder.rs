use anyhow::{anyhow, Result};
use common::constant::OrderStatus;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::{entity::*, Condition, QueryFilter, QueryOrder, QuerySelect, ConnectionTrait};
use serde_json::json;
use utility::timeutil::current_timestamp;
use crate::dataservice::entities::{prelude::*, *};
use crate::client::dbclient::DbClient;
use crate::common::common::OrderDetail;

impl PhoenixOrdStockorder {
    pub fn new() -> Self {
        PhoenixOrdStockorder::default()
    }

    pub async fn insert(model: &PhoenixOrdStockorder, db: &DbClient) -> Result<i64> {
        log::info!("订单: {:?}", model);        
        let db = db.get_conn();
        // let info = PhoenixOrdStockorder::convert_to_activemodel(&model);
        let ret = phoenix_ord_stockorder::ActiveModel::from_json(json!(model));
        if ret.as_ref().is_err() {
            return Err(anyhow!("from json error: {}", ret.as_ref().err().unwrap().to_string()));
        }
        let info = ret.unwrap();
        match PhoenixOrdStockorderEntity::insert(info).exec(db).await {
            Ok(v) => {
                log::info!("订单落库成功: {}", model.order_no);
                return Ok(v.last_insert_id)
            }
            Err(err) => {
                log::error!("订单{}落库失败: {:?}", model.order_no, err);
                return Err(anyhow!("订单{}落库失败: {:?}", model.order_no, err));
            }
        }
    }

    pub async fn update(model: &PhoenixOrdStockorder, db: &DbClient) -> Result<()>{
        let conn = db.get_conn();
        // let info = PhoenixOrdStockorder::convert_to_activemodel(model);
        let ret = phoenix_ord_stockorder::ActiveModel::from_json(json!(model));
        if ret.as_ref().is_err() {
            return Err(anyhow!("from json error: {}", ret.as_ref().err().unwrap().to_string()));
        }
        let info = ret.unwrap();
        match PhoenixOrdStockorderEntity::update(info)
        .filter(phoenix_ord_stockorder::Column::OrderNo.eq(model.order_no))
        .exec(conn).await {
            Ok(v) => {
                log::info!("订单更新成功: {:?}", v);
            }
            Err(err) => {
                log::error!("订单{}更新失败: {}", model.order_no, err);
                return Err(anyhow!("订单{}更新失败: {}", model.order_no, err));
            }
        }
        Ok(())
    }

    // pub async fn a<'a, 'async_trait, C>(_i:i64, _t: &'a C) 
    // where 
    //     C: ConnectionTrait,
    //     'a: 'async_trait,
    //     C: 'async_trait,
    // {

    
    // }

    pub async fn save(model: &PhoenixOrdStockorder, db: &DbClient) -> Result<()> {
        let conn = db.get_conn();
        let info = PhoenixOrdStockorder::convert_to_activemodel(model);
        let ret = info.save(conn).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("db失败:{}, {}", model.order_no, ret.as_ref().err().unwrap().to_string()));
        }
        if model.id > 0 {
            log::info!("订单更新成功: {:?}", ret.unwrap().try_into_model().unwrap_or_default());
        } else {
            let model = ret.unwrap().try_into_model().unwrap_or_default();
            log::info!("订单落库成功: {:?}", model);
        }
        Ok(())
    }

    pub async fn query_order(order_id: i64, db: &DbClient) -> Result<PhoenixOrdStockorder>{
        let db = db.get_conn();
        match PhoenixOrdStockorderEntity::find()
        .filter(phoenix_ord_stockorder::Column::OrderNo.eq(order_id))
        .one(db)
        .await {
            Ok(model) => {
                if let Some(val) = model {
                    return Ok(val);
                } else {
                    return Err(anyhow!("未找的订单 {} 信息", order_id));
                }
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err));
            }
        };
    }

    pub async fn query_cancel_order(order_id: i64, db: &DbClient) -> Result<PhoenixOrdStockorder>{
        let db = db.get_conn();
        let invalid: i32 = OrderStatus::INVALID as i32; //5：废单
        let dealed: i32 = OrderStatus::DEALED as i32;   //7：已成
        let canceled: i32 = OrderStatus::CANCELED as i32;//9：已撤
        match PhoenixOrdStockorderEntity::find()
        .filter(phoenix_ord_stockorder::Column::OrderNo.eq(order_id))
        .filter(phoenix_ord_stockorder::Column::OrderStatus.is_not_in(vec![invalid, dealed, canceled]))
        .one(db)
        .await {
            Ok(model) => {
                if let Some(val) = model {
                    return Ok(val);
                } else {
                    return Err(anyhow!("未找的订单 {} 信息", order_id));
                }
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err));
            }
        };
    }

    pub async fn delete() {}

    pub async fn confirm_update(order_id: i64, db: &DbClient) -> Result<u64> {
        //状态为未报, 待报,时更新
        log::info!("主委托确认处理 order_id: {}", order_id);
        let db = db.get_conn();
        let inited: i32 = OrderStatus::INITED as i32; //未报
        let submitted = OrderStatus::SUBMITTED as i32;//已报
        match PhoenixOrdStockorderEntity::update_many()
            .col_expr(phoenix_ord_stockorder::Column::OrderStatus, Expr::value(Value::Int(Some(submitted))))
            .col_expr(phoenix_ord_stockorder::Column::ModifyTime, Expr::value(current_timestamp()))
            .filter(phoenix_ord_stockorder::Column::OrderNo.eq(order_id))
            .filter(phoenix_ord_stockorder::Column::OrderStatus.is_in(vec![inited, 2]))
            .exec(db)
        .await 
        {
            Ok(v) => {
                log::info!("确认更新{}条数据", v.rows_affected);
                return Ok(v.rows_affected);
            },
            Err(err) => {
                log::error!("{} 确认更新失败: {:?}", order_id, err);
                return Err(anyhow!("{} 确认更新失败: {:?}", order_id, err))
            }
        }
    }

    pub async fn deal_update(order_detail: &OrderDetail, db: &DbClient)  -> Result<u64> {
        let db = db.get_conn();
        match PhoenixOrdStockorderEntity::update_many()
            .col_expr(phoenix_ord_stockorder::Column::OrderStatus, Expr::value(Value::Int(Some(order_detail.order_status))))
            .col_expr(phoenix_ord_stockorder::Column::DealAmount, Expr::value(Value::Int(Some(order_detail.deal_amount))))
            .col_expr(phoenix_ord_stockorder::Column::DealValue, Expr::value(Value::Decimal(Some(Box::new(order_detail.deal_value)))))
            .col_expr(phoenix_ord_stockorder::Column::DealFee, Expr::value(Value::Decimal(Some(Box::new(order_detail.deal_fee)))))
            .col_expr(phoenix_ord_stockorder::Column::LastDealTime, Expr::value(Value::BigInt(Some(order_detail.last_deal_time))))
            .col_expr(phoenix_ord_stockorder::Column::ModifyTime, Expr::value(current_timestamp()))
            .filter(phoenix_ord_stockorder::Column::OrderNo.eq(order_detail.order_id))
            .exec(db)
        .await 
        {
            Ok(v) => {
                log::info!("成交更新{}条数据", v.rows_affected);
                return Ok(v.rows_affected);
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err))
            }
        }
    }

    pub async fn cancel_update(detail: &OrderDetail, db: &DbClient) -> Result<u64> {
        let db = db.get_conn();
        match PhoenixOrdStockorderEntity::update_many()
            .col_expr(phoenix_ord_stockorder::Column::OrderStatus, Expr::value(Value::Int(Some(detail.order_status))))
            .col_expr(phoenix_ord_stockorder::Column::CancelAmount, Expr::value(Value::Int(Some(detail.cancel_amount))))
            .col_expr(phoenix_ord_stockorder::Column::OrderMemo, Expr::value(detail.memo.to_owned()))
            .col_expr(phoenix_ord_stockorder::Column::ModifyTime, Expr::value(current_timestamp()))
            .filter(phoenix_ord_stockorder::Column::OrderNo.eq(detail.order_id))
            .exec(db)
        .await 
        {
            Ok(v) => {
                log::info!("撤单更新{}条数据", v.rows_affected);
                return Ok(v.rows_affected);
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err))
            }
        }
    }

    pub async fn opponent_update(detail: &OrderDetail, db: &DbClient) -> Result<u64> {
        let db = db.get_conn();
        match PhoenixOrdStockorderEntity::update_many()
            .col_expr(phoenix_ord_stockorder::Column::RelateOrder, Expr::value(Value::BigInt(Some(detail.relate_id))))
            .col_expr(phoenix_ord_stockorder::Column::OrderMemo, Expr::value(detail.memo.to_owned()))
            .col_expr(phoenix_ord_stockorder::Column::ModifyTime, Expr::value(current_timestamp()))
            .filter(phoenix_ord_stockorder::Column::OrderNo.eq(detail.order_id))
            .exec(db)
        .await 
        {
            Ok(v) => {
                log::info!("撤单更新{}条数据", v.rows_affected);
                return Ok(v.rows_affected);
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err))
            }
        }
    }


    pub async fn cancel_entrust_update(cancel_id: i64, db: &DbClient) -> Result<u64> {
        let db = db.get_conn();
        match PhoenixOrdStockorderEntity::update_many()
            .col_expr(phoenix_ord_stockorder::Column::OrderStatus, Expr::value(Value::Int(Some(OrderStatus::CANCELED as i32))))
            .col_expr(phoenix_ord_stockorder::Column::ModifyTime, Expr::value(current_timestamp()))
            .filter(phoenix_ord_stockorder::Column::OrderNo.eq(cancel_id))
            .exec(db)
        .await 
        {
            Ok(v) => {
                log::info!("更新{}条数据", v.rows_affected);
                return Ok(v.rows_affected);
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err))
            }
        }
    }

    // pub async fn cancel_flag_update(order_id: i64, db: &DbClient) -> Result<u64>{
    //     let db = db.get_conn();
    //     match PhoenixOrdStockorderEntity::update_many()
    //         .col_expr(phoenix_ord_stockorder::Column::C, Expr::value(Value::Int(Some(1))))
    //         .filter(phoenix_ord_stockorder::Column::OrderStatus.is_in(vec![1, 4, 6]))
    //         .filter(phoenix_ord_stockorder::Column::OrderNo.eq(order_id))
    //         .exec(db)
    //     .await 
    //     {
    //         Ok(v) => {
    //             //受影响的行
    //             log::info!("更新{}条数据", v.rows_affected);
    //             return Ok(v.rows_affected);
    //         },
    //         Err(err) => {
    //             log::error!("{:?}", err);
    //             return Err(anyhow!("处理[未报、已报、部成]委托记录{}时，修改委托状态为“待撤”失败! {:?}", order_id, err))
    //         }
    //     }
    // }

    pub async fn select(unit_id: i64, order_id: i64, db: &DbClient) -> Vec<PhoenixOrdStockorder>{
        let db = db.get_conn();
        match PhoenixOrdStockorderEntity::find()
        .filter(phoenix_ord_stockorder::Column::UnitId.eq(unit_id))
        .filter(phoenix_ord_stockorder::Column::OrderNo.eq(order_id))
        // .order_by_asc(phoenix_ord_stockorder::Column::OrderStatus)
        .all(db)
        .await {
            Ok(v) => {
                return v;
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Vec::new();
            }
        };
    }

    pub fn convert_to_activemodel(info: &PhoenixOrdStockorder) -> phoenix_ord_stockorder::ActiveModel {
        let mut retinfo = phoenix_ord_stockorder::ActiveModel {
            id: NotSet,
            order_no: Set(info.order_no),
            sys_date: Set(info.sys_date),
            unit_id: Set(info.unit_id),
            stock_id: Set(info.stock_id),
            stock_code: Set(info.stock_code.to_owned()),
            exchange_id: Set(info.exchange_id),
            order_direction: Set(info.order_direction),
            order_price: Set(info.order_price),
            order_amount: Set(info.order_amount),
            order_type: Set(info.order_type),
            pre_fee: Set(info.pre_fee),
            pre_capital: Set(info.pre_capital),
            price_type: Set(info.price_type),
            deal_amount: Set(info.deal_amount),
            deal_value: Set(info.deal_value),
            deal_fee: Set(info.deal_fee),
            cancel_amount: Set(info.cancel_amount),
            order_status: Set(info.order_status),
            last_deal_time: Set(info.last_deal_time),
            relate_order: Set(info.relate_order),
            trade_type: Set(info.trade_type),
            order_memo: Set(info.order_memo.to_owned()),
            create_time: Set(info.create_time),
            modify_time: Set(info.modify_time),
        };

        if info.id > 0 {
            retinfo.id = Set(info.id);
        }
        // log::info!("active model:{:?}", &retinfo);
        retinfo
    }
}