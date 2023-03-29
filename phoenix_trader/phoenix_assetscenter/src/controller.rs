use crate::app::constdata;
use crate::config::settings::Settings;
use crate::dataservice::dbsetup::DbConnection;
use crate::dataservice::entities::prelude::*;
use crate::protofiles::assetscenter::{PhoenixassetsResult, PhoenixassetscenterQueryRequest, PhoenixassetscenterRequest, PhoenixassetscenterRequestInfo, PhoenixassetscenterResponse};
use crate::service::assetssvc::UnitAssetsService;
use crate::service::stockpositionsvc::UnitPositionService;
use anyhow::Result;
use common::akaclient::AkaClient;
use common::constant;
// use common::logclient::LogClient;
use common::redisclient::redisclient::RedisClient;
use common::uidservice::UidgenService;
// use futures::channel::mpsc::Sender;
use messagecenter::notificationclient::NotificationClient;
use messagecenter::protofiles::phoenixnotification::{MessageBody, NotificationAsset, NotificationMessage, NotificationPosition, NotificationType};
// use sea_orm::query;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tonic::{self, Status};

#[derive(Debug)]
pub enum PersistData {
  OperationDetail(Box<Vec<PhoenixAstOperationDetail>>),
  UserPosition(Box<Vec<PhoenixAstStockposition>>),
  UserAssets(Box<Vec<PhoenixAstUnitasset>>),
  AssetsFlow(Box<Vec<PhoenixOmsAssetflow>>),
  PhoenixOmsAssetflow(Box<(PhoenixOmsAssetflow, PhoenixAstOperationDetail)>),
}

#[derive(Debug)]
pub enum NotificationData {
  UserPosition(Box<Vec<NotificationPosition>>),
  UserAssets(Box<NotificationAsset>),
}

#[derive(Clone)]
pub struct ServerController {
  pub redis: Arc<RedisClient>,
  pub dbconn: Arc<DbConnection>,
  pub assetssvc: Arc<UnitAssetsService>,
  pub positionsvc: Arc<UnitPositionService>,
  pub uidgen: Arc<UidgenService>,
  pub akasvc: Arc<AkaClient>,
  pub config: Arc<RwLock<Settings>>,
  pub sysinfo: Arc<PhoenixSysSystem>,
  pub notify: Arc<NotificationClient>,
  pub tx_persist: mpsc::Sender<PersistData>,
  pub tx_notification: mpsc::Sender<NotificationData>,
}

//处理业务逻辑
impl ServerController {
  pub async fn init(&self) {
    //初始化
    //1)从数据库读取所有数据并保存到redis中
    let ret = self.assetssvc.phoenix_query_all_units(&self.dbconn, &self.redis).await;
    if let Err(err) = ret {
      return;
    }

    let assets = ret.unwrap();
    for item in assets {
      _ = self.assetssvc.init(item.unit_id, &self.dbconn, &self.redis);
      _ = self.positionsvc.init(item.unit_id, 0, &self.dbconn, &self.redis);
    }
  }

  //发送持仓数据到消息中心
  pub async fn publish_data(&self, data: &NotificationData) -> Result<()> {
    // 1）组包
    // 2）推送
    match data {
      NotificationData::UserAssets(assets) => {
        //do
        //key: mc.order.持仓UID
        let routing_key = format!("mc.assets.{}", assets.unit_id);
        let assets = assets.as_ref().to_owned();
        let msgbody = MessageBody {
          msg_asset: Some(assets),
          ..Default::default()
        };
        let msg = NotificationMessage {
          msg_type: NotificationType::AssetChanged as i32,
          msg_body: Some(msgbody),
        };
        _ = self.notify.try_publish(routing_key.as_str(), &msg).await;
      }
      NotificationData::UserPosition(positions) => {
        for item in positions.iter() {
          let routing_key = format!("mc.positions.{}", item.position_no);
          let position = item.to_owned();
          let msgbody = MessageBody {
            msg_position: Some(position),
            ..Default::default()
          };
          let msg = NotificationMessage {
            msg_type: NotificationType::PositionChanged as i32,
            msg_body: Some(msgbody),
          };
          _ = self.notify.try_publish(routing_key.as_str(), &msg).await;
        }
      }
    }
    Ok(())
  }

  pub async fn save_dbdata(&self, data: &PersistData) -> Result<()> {
    match data {
      PersistData::UserAssets(assets) => {
        let req = assets.to_vec();
        _ = self.assetssvc.update_assets_model_data(&req, &self.dbconn).await;
      }
      PersistData::AssetsFlow(flows) => {
        let req = flows.to_vec();
        _ = self.assetssvc.save_assets_flow_data(&req, &self.dbconn).await;
      }
      PersistData::OperationDetail(operations) => {
        let req = operations.to_vec();
        _ = self.assetssvc.save_assets_operations_data(&req, &self.dbconn).await;
      }
      PersistData::UserPosition(positions) => {
        let req = positions.to_vec();
        _ = self.positionsvc.update_positions_dbdata(&req, &self.dbconn).await
      }
      PersistData::PhoenixOmsAssetflow(data) => {
        let req = data.as_ref();
        _ = self.positionsvc.save_positions_flow_data(req.to_owned(), &self.dbconn).await
      }
    }
    Ok(())
  }

  //------------grpc interface -------------------
  pub async fn phoenix_assets_change(&self, req: &PhoenixassetscenterRequest) -> Result<PhoenixassetscenterResponse, Status> {
    let mut result = PhoenixassetscenterResponse { ..Default::default() };
    let mut err_msg = "".to_string();
    log::info!("phoenix_assets_change收到请求,{:?}", req);

    //处理资产请求
    if req.assets.len() > 0 {
      let assets_req = req.to_owned();
      //拆分创建资产用户和资产调整的请求

      let (assets_req_create, assets_req_change): (Vec<PhoenixassetscenterRequestInfo>, Vec<PhoenixassetscenterRequestInfo>) =
        assets_req.assets.into_iter().partition(|f| f.op_type == constant::AssetChangeDirection::CreateUnit as i32);

      if assets_req_create.len() > 0 {
        //创建资产用户
        let ret = self.assetssvc.phoenix_assets_crate(&req, &assets_req_create, &self.redis, &self.dbconn, &self.sysinfo).await;
        match ret {
          Ok(v) => {
            if v.unit_id > 0 {
              _ = self.publish_data(&NotificationData::UserAssets(Box::new(v))).await;
            }
          }
          Err(e) => {
            err_msg = e.to_string();
          }
        }
      }
      if assets_req_change.len() > 0 {
        //资产调整
        let ret = self
          .assetssvc
          .phoenix_assets_change(&req, &assets_req_change, &self.redis, &self.dbconn, &self.sysinfo, &self.tx_persist)
          .await;
        match ret {
          Ok(v) => {
            _ = self.publish_data(&NotificationData::UserAssets(Box::new(v))).await;
          }
          Err(e) => {
            let lockkey = format!("{}{}", constdata::LOCK_USER_REDIS_KEY, req.unit_id);
            let _ = self.redis.delele(&lockkey);
            err_msg = e.to_string();
          }
        }
      }
    }

    if req.postions.len() > 0 {
      let ret = self
        .positionsvc
        .phenix_postions_change(&req, &self.redis, &self.dbconn, &self.uidgen, &self.akasvc, &self.sysinfo, &self.tx_persist)
        .await;
      match ret {
        Ok(v) => {
          _ = self.publish_data(&NotificationData::UserPosition(Box::new(v))).await;
        }
        Err(e) => {
          err_msg = e.to_string();
        }
      }
    }
    if !err_msg.is_empty() {
      result.ret_code = 9999;
      result.ret_msg = err_msg;
    } else {
      result.ret_code = 0;
      result.ret_msg = "success".to_string();
    }

    Ok(result)
  }

  pub async fn phoenix_assets_query(&self, req: &PhoenixassetscenterQueryRequest) -> Result<PhoenixassetscenterResponse, Status> {
    log::info!("phoenix_assets_query收到请求,{:?}", req);
    let mut query_flag = true;
    let mut err_msg = "".to_string();
    let mut ret = PhoenixassetscenterResponse::default();
    let mut info_vec = Vec::new();
    let mut unit_id = req.unit_id.to_owned();

    //若是空数组，那查下出所有用户的基础信息
    if unit_id.is_empty() {
      let units_ret = self.assetssvc.phoenix_query_all_units(&self.dbconn, &self.redis).await;
      if let Err(err) = units_ret {
        query_flag = false;
        err_msg = err.to_string();
      } else {
        for i in units_ret.unwrap().iter() {
          unit_id.push(i.to_owned().unit_id);
        }
      }
    }

    for item in unit_id.iter() {
      let mut info = PhoenixassetsResult::default();
      let query_type = req.query_type.to_string();
      if query_type.eq(&constant::QueryAssetsType::Assets.to_string()) || query_type.eq(&constant::QueryAssetsType::Both.to_string()) {
        let assets_result = self.assetssvc.phoenix_query_assets(item.to_owned(), &self.redis, &self.dbconn).await;
        if let Err(err) = assets_result {
          query_flag = false;
          err_msg = err.to_string();
          break;
        }
        info.assets = assets_result.unwrap();
      }
      if query_type.eq(&constant::QueryAssetsType::Positions.to_string()) || query_type.eq(&constant::QueryAssetsType::Both.to_string()) {
        let position_result = self.positionsvc.phoenix_query_positions(item.to_owned(), &self.redis, &self.dbconn).await;
        if let Err(err) = position_result {
          query_flag = false;
          err_msg = err.to_string();
          break;
        }
        info.positionsinfo = position_result.unwrap();
      }
      info.unit_id = item.to_owned();
      info_vec.push(info);
    }

    ret.assetsinfo = info_vec;
    if !query_flag {
      ret.ret_code = 9999;
      ret.ret_msg = err_msg;
    } else {
      ret.ret_code = 0;
      ret.ret_msg = "success".to_string();
    }

    Ok(ret)
  }
}
