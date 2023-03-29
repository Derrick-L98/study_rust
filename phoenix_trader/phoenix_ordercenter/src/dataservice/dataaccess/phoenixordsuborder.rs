use anyhow::{anyhow, Result};
use common::constant::OrderStatus;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::{entity::*, Condition, QueryFilter, QueryOrder};
use serde_json::json;
use utility::timeutil::current_timestamp;
use crate::dataservice::entities::{prelude::*, *};
use crate::client::dbclient::DbClient;
use crate::common::common::OrderDetail;

impl PhoenixOrdSuborder {
    pub fn new() -> Self {
        PhoenixOrdSuborder::default()
    }
    // save 和 insert 区别：
    // save: 如果主键为NotSet，则插入模型，否则更新。仅当实体具有自动递增主键时才有效。
    pub async fn insert(model: &PhoenixOrdSuborder, db: &DbClient) -> Result<PhoenixOrdSuborder> {
        let db = db.get_conn();
        let info = PhoenixOrdSuborder::convert_to_activemodel(model);
        match info.insert(db).await {
            Ok(v) =>{
                log::info!("子订单落库成功: {:?}", v);
                return Ok(v);
            }
            Err(err) => {
                log::error!("子订单落库失败: {:?}", err);
                return Err(anyhow!("子订单落库失败: {:?}", err));
            }
        }
    }

    pub async fn delete() {}
    pub async fn update() {}

    pub async fn insert_many(info: &Vec<PhoenixOrdSuborder>, db: &DbClient) -> Result<()> {
        log::info!("子订单: {:?}", info); 
        if info.len() <= 0 {
            return Err(anyhow!("empty data"));
        }
        let db = db.get_conn();
        let model: Vec<phoenix_ord_suborder::ActiveModel> = info.iter().map(|val| PhoenixOrdSuborder::convert_to_activemodel(val)).collect();
        let ret = PhoenixOrdSuborderEntity::insert_many(model).exec(db).await;
        if ret.as_ref().is_err() {
            log::error!("子订单落库失败: {:?}", ret.as_ref().err().unwrap().to_string());
            return Err(anyhow!("子订单落库失败: {:?}", ret.as_ref().err().unwrap().to_string()));
        }
        log::info!("子订单落库成功: {:?}", ret.unwrap());
        Ok(())
    }

    pub async fn update_many(info: &Vec<PhoenixOrdSuborder>, db: &DbClient) -> Result<()> {
        if info.len() <= 0 {
            return Err(anyhow!("empty data"));
        }
        let db = db.get_conn();
        for data in info.iter() {
            let ret = phoenix_ord_suborder::ActiveModel::from_json(json!(data));
            if ret.as_ref().is_err() {
                return Err(anyhow!("from json error: {}", ret.as_ref().err().unwrap().to_string()));
            }
            let model = ret.unwrap();
            let ret = PhoenixOrdSuborderEntity::update(model).exec(db).await;
            if ret.as_ref().is_err() {
                log::error!("子订单更新失败: {:?}", ret.as_ref().err().unwrap().to_string());
                return Err(anyhow!("子订单更新失败: {:?}", ret.as_ref().err().unwrap().to_string()));
            }
            log::info!("子订单更新成功: {:?}", ret.unwrap());
        }
        Ok(())
    }

    pub async fn save_many(infos: &Vec<PhoenixOrdSuborder>, db: &DbClient) -> Result<()>{
        if infos.len() <= 0 {
            return Err(anyhow!("empty data"));
        }
        let db = db.get_conn();
        let (update_values, insert_values): (Vec<PhoenixOrdSuborder>, Vec<PhoenixOrdSuborder>) = infos.clone().into_iter().partition(|v| v.id > 0);

        // do insert
        if insert_values.len() > 0 {
            let vals: Vec<phoenix_ord_suborder::ActiveModel> = insert_values.iter().map(|v| PhoenixOrdSuborder::convert_to_activemodel(v)).collect();
            let ret = PhoenixOrdSuborderEntity::insert_many(vals).exec(db).await;
            if ret.as_ref().is_err() {
                return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
            }
            log::info!("子订单落库成功: {}", ret.unwrap().last_insert_id);
        }

        //do update
        if update_values.len() > 0 {
            let vals: Vec<phoenix_ord_suborder::ActiveModel> = update_values.iter().map(|v| PhoenixOrdSuborder::convert_to_activemodel(v)).collect();
            for val in vals {
                let ret = val.update(db).await;
                if ret.as_ref().is_err() {
                    return Err(anyhow!(ret.as_ref().err().unwrap().to_string()));
                }
                log::info!("子订单更新成功: {:?}", ret.unwrap());
            }
        }
        Ok(())
    }
    
    //id 为主委托id(查询所有子委托)
    pub async fn query_all_sub_order(order_id: i64, db: &DbClient) -> Result<Vec<PhoenixOrdSuborder>>{
        let db = db.get_conn();
        match PhoenixOrdSuborderEntity::find()
            // .filter(phoenix_ord_suborder::Column::UnitId.eq(unit_id))
            .filter(phoenix_ord_suborder::Column::OrderNo.eq(order_id))
            // .order_by_asc(phoenix_ord_suborder::Column::OrderNo)
            .all(db)
        .await {
            Ok(v) => {
                return Ok(v);
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err));
            }
        }
    }

    pub async fn query_sub_order_by_subid(sub_id: i64, db: &DbClient) -> Result<PhoenixOrdSuborder>{
        let db = db.get_conn();
        match PhoenixOrdSuborderEntity::find()
            .filter(phoenix_ord_suborder::Column::SubId.eq(sub_id))
            .one(db)
        .await {
            Ok(model) => {
                if let Some(val) = model {
                    return Ok(val);
                } else {
                    return Err(anyhow!("未找的子订单 {} 信息", sub_id));
                }
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err));
            }
        }
    }

    pub async fn query_sub_order(order_detail: &OrderDetail, db: &DbClient) -> Result<PhoenixOrdSuborder>{
        let db = db.get_conn();
        match PhoenixOrdSuborderEntity::find()
            .filter(phoenix_ord_suborder::Column::OrderNo.eq(order_detail.order_id))
            .filter(phoenix_ord_suborder::Column::ChannelId.eq(order_detail.channel_id))
            .filter(phoenix_ord_suborder::Column::ChannelType.eq(order_detail.channel_type))
            .one(db)
            // .order_by_asc(phoenix_ord_suborder::Column::OrderNo)
        .await {
            Ok(model) => {
                if let Some(val) = model {
                    return Ok(val);
                } else {
                    return Err(anyhow!("未找的订单 {} 信息, channel_id: {}, channel_type: {}", order_detail.order_id, order_detail.channel_id, order_detail.channel_type));
                }
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err));
            }
        }
    }

    pub async fn confirm_update(order_detail: &OrderDetail, db: &DbClient) -> Result<u64> {
        log::info!("子委托确认处理 order_id: {}, channel_id: {}, channel_type: {}", order_detail.order_id, order_detail.channel_id, order_detail.channel_type);
        let db = db.get_conn();
        let inited: i32 = OrderStatus::INITED as i32; //未报
        let submitted = OrderStatus::SUBMITTED as i32;//已报
        // .col_expr(phoenix_ord_suborder::Column::ConfirmNo, Expr::value(Value::String(Some(Box::new(confirm_no.to_owned())))))
        match PhoenixOrdSuborderEntity::update_many()
            .col_expr(phoenix_ord_suborder::Column::OrderStatus, Expr::value(Value::Int(Some(submitted))))
            .col_expr(phoenix_ord_suborder::Column::ConfirmNo, Expr::value(order_detail.confirm_no.to_owned()))
            .col_expr(phoenix_ord_suborder::Column::ModifyTime, Expr::value(current_timestamp()))
            .filter(phoenix_ord_suborder::Column::OrderNo.eq(order_detail.order_id))
            .filter(phoenix_ord_suborder::Column::ChannelId.eq(order_detail.channel_id))
            .filter(phoenix_ord_suborder::Column::ChannelType.eq(order_detail.channel_type))
            .filter(phoenix_ord_suborder::Column::OrderStatus.is_in(vec![inited, 2]))
            .exec(db)
        .await 
        {
            Ok(v) => {
                log::info!("确认更新{}条数据", v.rows_affected);
                return Ok(v.rows_affected);
            },
            Err(err) => {
                log::error!("{} 确认更新失败: {:?}", order_detail.order_id, err);
                return Err(anyhow!("{} 确认更新失败: {:?}", order_detail.order_id, err))
            }
        }
    }

    pub async fn deal_update(order_detail: &OrderDetail, db: &DbClient)  -> Result<u64> {
        let db = db.get_conn();
        match PhoenixOrdSuborderEntity::update_many()
            .col_expr(phoenix_ord_suborder::Column::OrderStatus, Expr::value(Value::Int(Some(order_detail.order_status))))
            .col_expr(phoenix_ord_suborder::Column::DealAmount, Expr::value(Value::Int(Some(order_detail.deal_amount))))
            .col_expr(phoenix_ord_suborder::Column::ConfirmNo, Expr::value(order_detail.confirm_no.to_owned()))
            .col_expr(phoenix_ord_suborder::Column::DealValue, Expr::value(Value::Decimal(Some(Box::new(order_detail.deal_value)))))
            .col_expr(phoenix_ord_suborder::Column::ModifyTime, Expr::value(current_timestamp()))
            .filter(phoenix_ord_suborder::Column::OrderNo.eq(order_detail.order_id))
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
        match PhoenixOrdSuborderEntity::update_many()
            .col_expr(phoenix_ord_suborder::Column::OrderStatus, Expr::value(Value::Int(Some(detail.order_status))))
            .col_expr(phoenix_ord_suborder::Column::ConfirmNo, Expr::value(detail.confirm_no.to_owned()))
            .col_expr(phoenix_ord_suborder::Column::CancelFlag, Expr::value(0))
            .col_expr(phoenix_ord_suborder::Column::Remark, Expr::value(detail.memo.to_owned()))
            .col_expr(phoenix_ord_suborder::Column::ModifyTime, Expr::value(current_timestamp()))
            .filter(phoenix_ord_suborder::Column::OrderNo.eq(detail.order_id))
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
        match PhoenixOrdSuborderEntity::update_many()
            .col_expr(phoenix_ord_suborder::Column::RelateOrder, Expr::value(Value::BigInt(Some(detail.relate_id))))
            .col_expr(phoenix_ord_suborder::Column::ConfirmNo, Expr::value(detail.confirm_no.to_owned()))
            .col_expr(phoenix_ord_suborder::Column::ModifyTime, Expr::value(current_timestamp()))
            .filter(phoenix_ord_suborder::Column::SubId.eq(detail.sub_id))
            .exec(db)
        .await 
        {
            Ok(v) => {
                log::info!("关联委托更新{}条数据", v.rows_affected);
                return Ok(v.rows_affected);
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!(err))
            }
        }
    }

    pub async fn cancel_flag_update(order_id: i64, db: &DbClient) -> Result<u64>{
        let db = db.get_conn();
        match PhoenixOrdSuborderEntity::update_many()
            .col_expr(phoenix_ord_suborder::Column::CancelFlag, Expr::value(Value::Int(Some(1))))
            .filter(phoenix_ord_suborder::Column::OrderStatus.is_in(vec![OrderStatus::INITED as i32, OrderStatus::SUBMITTED as i32, OrderStatus::PARTIALDEALED as i32]))
            .filter(phoenix_ord_suborder::Column::OrderNo.eq(order_id))
            .exec(db)
        .await 
        {
            Ok(v) => {
                //受影响的行
                log::info!("更新{}条数据cancel_flag", v.rows_affected);
                return Ok(v.rows_affected);
            },
            Err(err) => {
                log::error!("{:?}", err);
                return Err(anyhow!("处理[未报、已报、部成]委托记录{}时，修改委托状态为“待撤”失败! {:?}", order_id, err))
            }
        }
    }

    pub fn convert_to_activemodel(info: &PhoenixOrdSuborder) -> phoenix_ord_suborder::ActiveModel {
        let mut retinfo = phoenix_ord_suborder::ActiveModel {
            id: NotSet,
            sub_id: Set(info.sub_id),
            sys_date: Set(info.sys_date),
            order_no: Set(info.order_no),
            unit_id: Set(info.unit_id),
            stock_code: Set(info.stock_code.to_owned()),
            channel_id: Set(info.channel_id),
            channel_type: Set(info.channel_type),
            order_amount: Set(info.order_amount),
            order_price: Set(info.order_price),
            price_type: Set(info.price_type),
            confirm_no: Set(info.confirm_no.to_owned()),
            order_status: Set(info.order_status),
            deal_value: Set(info.deal_value),
            deal_amount: Set(info.deal_amount),
            modify_time: Set(info.modify_time),
            create_time: Set(info.create_time),
            relate_order: Set(info.relate_order),
            cancel_flag: Set(info.cancel_flag),
            remark: Set(info.remark.to_owned()),
        };

        if info.id > 0 {
            retinfo.id = Set(info.id);
        }
        // log::info!("active model:{:?}", &retinfo);
        retinfo
    }
}

