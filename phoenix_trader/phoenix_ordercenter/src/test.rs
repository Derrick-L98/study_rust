#[macro_use]
extern crate anyhow;
extern crate chrono;

mod dataservice;
mod common;
mod server;
mod config;
mod client;
mod protofiles;
use ::common::akaclient::AkaClient;
use ::common::constant::{AssetChangeDirection, AssetChangeType};
use ::common::uidservice::UidgenService;
use messagecenter::notificationclient::NotificationClient;
use messagecenter::protofiles::phoenixnotification::NotificationMessage;
use sea_orm::{TransactionTrait, DatabaseTransaction, DbErr, ActiveModelTrait, Set};
use tokio::sync::mpsc;
use tonic::transport::Channel;
use utility::loggings;
use crate::protofiles::{OrderReq, CancelReq};
// use crate::config::settings::Settings;
use std::{pin::Pin, time::Duration};

use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::{entity::*, Condition, QueryFilter, QueryOrder, QuerySelect};
use crate::dataservice::entities::{prelude::*, *};
use crate::client::DbClient;
use crate::protofiles::order_center_service_client::OrderCenterServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loggings::log_init();

    // mq().await;
// use sea_orm::entity::prelude::*;
// use sea_orm::Uuid;
// let id = Uuid::new_v4();
// log::info!("==================={:#?}", id);

    // let settings = Settings::new().expect("read config error");
    // log::info!("{:#?}", settings);
    let mut id = UidgenService::new(1,1080);
    // for i in 0..50 {
        // test(&id).await; 
    // }

    let uri = String::from("mysql://companytest:ZNvC5NoLJNnO3Wn0@uat-stock-data.chinaeast2.cloudapp.chinacloudapi.cn:13301/phoenix_stock");
    let client = DbClient::new(&uri).await;
    // client.get_conn().await.begin();
    // let mut data = Model::new();
    // data.order_no = 12;
    // data.unit_id = 10;
    // data.order_status = 1;

    // Model::confirm_update(12, &client).await;
    // if let Err(err) = Model::insert(&data, &client).await {
    //     log::info!("{:?}", err);
    //     if let Err(e) = client.get_conn().await.transaction::<_, (), DbErr>(move |x | 
    //         Box::pin(async move { 
    //             if let Err(err) = Model::convert_to_activemodel(&data).save(x).await {
    //                 log::info!("{:?}", err);
    //                 Err(DbErr::Query(RuntimeErr::Internal(
    //                     "Force Rollback!".to_owned(),
    //                 )))
    //             }
    //             Ok(())
    //         }
    //     )).await {
    //         Ok(())
    //     }
    // }

    // if let Err(err) = transaction_begin_commit(&client).await{
    //     log::info!("{:?}", err);
    // } 
    // let _ = db_test(&client).await;

    // for i in 1..100 {
        // println!("{}", i);
        // std::thread::sleep(std::time::Duration::from_secs(1));
        // let mut client = OrderCenterServiceClient::connect("http://127.0.0.1:9090").await?;
        // let mut client = OrderCenterServiceClient::connect("http://40.72.97.246:8405").await?;
        let mut client = OrderCenterServiceClient::connect("http://52.131.220.224:8405").await?;
        let now = std::time::Instant::now();
        // for i in 1..20 {
            place_order(&mut client).await;
        // };
        log::info!("persist completed, elapsed: {:?}", now.elapsed());
    // }

    // change().await;

    Ok(())
}



//10773
async fn place_order(client: &mut OrderCenterServiceClient<Channel>) -> Result<(), Box<dyn std::error::Error>>{
    // let val = OrderReq { 
    //     msg_id: 1, 
    //     unit_id: 201215, 
    //     stock_id: 10773, 
    //     order_direction: 1, 
    //     order_qty:2000, 
    //     price_type: 2, 
    //     order_price: 7.350, 
    //     operator_no: 201215, 
    //     order_type: 5, 
    //     trade_mode: 1, 
    //     agent_account: 0 
    // };
    // let val = OrderReq::default();

        //日照港
    let val = OrderReq { 
        msg_id: 1, 
        unit_id: 201215, 
        stock_id: 10784, 
        order_direction: 1, 
        order_qty: 100,
        price_type: 2, 
        order_price:2.95, 
        operator_no: 201215, 
        order_type: 1, 
        trade_mode: 1, 
        agent_account: 0,
        order_id: 0, 
    };

        //平安银行
    // let val = OrderReq { 
    //     msg_id: 1, 
    //     unit_id: 201215, 
    //     stock_id: 10312, 
    //     order_direction: 1, 
    //     order_qty: 1000,
    //     price_type: 2, 
    //     order_price:13.65, 
    //     operator_no: 201215, 
    //     order_type: 1, 
    //     trade_mode: 1, 
    //     agent_account: 0 
    // };

    // let val = OrderReq { 
    //     msg_id: 1, 
    //     unit_id: 200842,//201215, 
    //     stock_id: 40465, 
    //     order_direction: 2, 
    //     order_qty: 700,
    //     price_type: 2, 
    //     order_price:5.52, 
    //     operator_no: 201215, 
    //     order_type: 1, 
    //     trade_mode: 1, 
    //     agent_account: 0 
    // };

    // //中国移动
    // let val = OrderReq { 
    //     msg_id: 1, 
    //     unit_id: 201215, 
    //     stock_id: 40585,//7877,//40585, 
    //     order_direction: 1, 
    //     order_qty: 1000, 
    //     price_type: 2,//限价 
    //     order_price: 138.0, 
    //     operator_no: 201215, 
    //     order_type: 1, 
    //     trade_mode: 1, 
    //     agent_account: 0 
    // };

    // let val = OrderReq { 
    //     msg_id: 1, 
    //     unit_id: 201215, 
    //     stock_id: 9742,//10372,//10524,//10784,//20488,//20425,//40609,//10652,//10359, 
    //     order_direction: 1, 
    //     order_qty: 700, 
    //     price_type: 2,//限价 
    //     order_price: 132.80, 
    //     operator_no: 201215, 
    //     order_type: 1, 
    //     trade_mode: 1, 
    //     agent_account: 0 
    // };
    

    // let val = OrderReq { 
    //     msg_id: 1, 
    //     unit_id: 201215, 
    //     stock_id: 9803,//11762,//8478,//9742,//10372,//10524,//10784,//20488,//20425,//40609,//10652,//10359, 
    //     order_direction: 1, 
    //     order_qty: 1000, 
    //     price_type: 2,//限价 
    //     order_price: 2.95, 
    //     operator_no: 201215, 
    //     order_type: 1, 
    //     trade_mode: 1, 
    //     agent_account: 0 
    // };

    println!("RESPONSE={:#?}", client.place_order(tonic::Request::new(val.clone())).await?);
    Ok(())
}

async fn cancel_order(client: &mut OrderCenterServiceClient<Channel>) -> Result<(), Box<dyn std::error::Error>>{
    let val = CancelReq {
        msg_id: 1,
        unit_id: 201215,
        order_id: 7039067759644385275,
        operator_no: 201215,
        cancel_type: 1,
        trade_mode: 1,
        agent_account: 1, 
        // msg_id: 1, 
        // unit_id: 201215, 
        // stock_id: 40757, 
        // order_direction: 1, 
        // order_qty: 3000, 
        // price_type: 1, 
        // order_price: 27.55, 
        // operator_no: 201215, 
        // order_type: 1, 
        // trade_mode: 1, 
        // agent_account: 0,

    };

    println!("RESPONSE={:#?}", client.cancel_order(tonic::Request::new(val)).await?);
    Ok(())
}




// async fn grpc_client() ->Result<(), Box<dyn std::error::Error>> {
//     let mut client = OrderCenterServiceClient::connect("http://127.0.0.1:8765").await?;
//     let val = OrderReq { 
//         msg_id: 1, 
//         user_id: 123456, 
//         stock_id: String::from("101010"), 
//         order_side: String::from("1"), 
//         order_qty: 100, 
//         price_type: 1, 
//         order_price: 10.0, 
//         operator_no: 123456, 
//         order_type: 1, 
//         trade_mode: 1, 
//         agent_account: 0 
//     };

//     println!("RESPONSE={:#?}", client.place_order(tonic::Request::new(val)).await?);
// }


pub async fn transaction_begin_commit(client: &DbClient) -> Result<(), DbErr> {
    let ctx =client.get_conn();
    let mut data = PhoenixOrdStockorder::new();
    data.order_no = 12;
    data.unit_id = 10;
    data.order_status = 1;
    let mut active_model = PhoenixOrdStockorder::convert_to_activemodel(&data);
    {
        // Transaction begin in this scope
        let txn = ctx.begin().await?;

        // PhoenixOrdStockorder::a(1, ctx).await;
        // PhoenixOrdStockorder::a(1, &txn).await;
        // PhoenixOrdStockorder::a(1, &txn).await;
        // PhoenixOrdStockorder::a(1, &txn).await;


        match active_model.clone().save(&txn).await {
            Ok(v) => {
                // active_model.id = v.id;
            }
            Err(e) => {
                log::info!("{}", e);
            }
        }
        active_model.order_status = Set(2);
        active_model.clone().update(&txn).await?;
        if let Err(e) = active_model.clone().save(&txn).await {
            log::info!("{}", e);
            txn.rollback().await?
        } else {
            // Commit changes before the end of scope
            txn.commit().await?;
        }
    }
    {
        let txn = ctx.begin().await?;
        match PhoenixOrdStockorderEntity::insert(active_model).exec(&txn).await {
            Ok(v) => {
                log::info!("{:?}", v.last_insert_id);
                txn.commit().await?;
            }
            Err(err) => {
                log::error!("{:?}", err);
                txn.rollback().await?
            }
        }
    }
    

    Ok(())
}

async fn db_test(client: &DbClient) {
    // let conn = client.get_conn();
    let mut model = PhoenixOrdStockorder::new();
    model.order_no = 123456;
    model.sys_date = 1214;

    if let Err(e) = PhoenixOrdStockorder::insert(&model, &client).await {
        log::info!("{}", e);
    }

    model.order_status = 6;
    if let Err(e) = PhoenixOrdStockorder::update(&model, &client).await {
        log::info!("{}", e);
    }
    model.order_status = 9;
    if let Err(e) = PhoenixOrdStockorder::update(&model, &client).await {
        log::info!("{}", e);
    }
    model.order_status = 100;
    if let Err(e) = PhoenixOrdStockorder::update(&model, &client).await {
        log::info!("{}", e);
    }

    model.order_no = 1234567;
    if let Err(e) = PhoenixOrdStockorder::insert(&model, &client).await {
        log::info!("{}", e);
    }

    model.order_status = 6;
    if let Err(e) = PhoenixOrdStockorder::update(&model, &client).await {
        log::info!("{}", e);
    }
}

use crate::protofiles::{PhoenixassetscenterRequest, PhoenixassetscenterRequestInfo, PhoenixassetspostionrequestInfo};
async fn change () {
    let assetscenter_client = client::AssetsCenterClient::new(&"http://40.72.97.246:8403".to_owned()).await;

    let mut assets_req = PhoenixassetscenterRequest::default();
    let mut change_capital = PhoenixassetscenterRequestInfo::default();
    let mut change_postion = PhoenixassetspostionrequestInfo::default();
    assets_req.message_id = 164655225;
    assets_req.operator_id = 201215;
    assets_req.business_flag = AssetChangeType::TradeOrderAsset as i32;
    assets_req.unit_id = 201215;

    // change_capital.flag = 1;//创业板
    change_capital.op_type = AssetChangeDirection::UnFrozenTradeCapital as i32; //105：资产临时解冻
    change_capital.change_amount = 73392.1403;
    assets_req.assets.push(change_capital);

    // change_postion.stock_id = 0;
    // change_postion.op_type = AssetChangeDirection::ReducePosition as i32; // 202：持仓减少
    // change_postion.deal_amount = 0;
    // change_postion.margin_rate = 0.0;
    // change_postion.presale_amount = 0;
    // change_postion.temp_frozen_amount = 0;
    // assets_req.postions.push(change_postion);

    log::info!("place_order_asset_change: {:#?}", &assets_req);
    let ret = assetscenter_client.phoenix_assets_change(&assets_req).await;
    if ret.as_ref().is_err() {
        log::error!("委托资产调整错误: {:?}", ret.as_ref().err().unwrap().to_string());
        //如何处理?
    }
    log::info!("资产变化: {:#?}", ret.unwrap()); 
}


// #![feature(round_char_boundary)]
// let s = "❤️🧡💛💚💙💜";
// assert_eq!(s.len(), 26);
// assert!(!s.is_char_boundary(13));

// let closest = s.ceil_char_boundary(13);
// assert_eq!(closest, 14);
// assert_eq!(&s[..closest], "❤️🧡💛");

pub async fn test(uid: &UidgenService) {
    //这样使用, 每次调用都是一样的结果
    let mut id = uid.to_owned();
    for i in 0..50 {
    log::info!("1===={}", id.get_uid());
    log::info!("2===={}", id.get_uid());
    log::info!("3===={}", id.get_uid());
    log::info!("4===={}", id.get_uid());
    log::info!("5===={}", id.get_uid());
    log::info!("6===={}", id.get_uid());
    }
}



// pub async fn mq() {
//     let (tx_mq, mut rx_mq) = mpsc::channel::<YsHqInfo>(4096);
//     let exchanger = "StockLive";
//     let queue = "";
//     let routingkey = "";
//     let amqpaddr = "";
//     let mut mq_client = NotificationClient::new(&exchanger, &queue, routingkey.to_owned(), &amqpaddr, tx_mq).await;
//     let _ = mq_client.try_connect().await.expect("mq client connection error");
//     // 以下用来测试订阅mq消息
//     let mut interval = tokio::time::interval(Duration::from_secs(3));
//     tokio::spawn(async move {
//       loop {
//         tokio::select! {
//             _ = interval.tick() => {
//               let _ = mq_client.try_consume().await;
//             }
//         }
//       }
//     });

//     tokio::spawn(async move {
//         loop {
//           tokio::select! {
//             mq_task = rx_mq.recv() => {//测试mq消息接收
//                 if let Some(msg) = mq_task {
//                   log::info!("==================={:#?}", msg);
//                 }
//             }
//           }
//         }
//       });
// }