use super::querydto;
// use crate::protofiles::phoenixassetsdata::PhoenixAccountAssetsInfo;
use crate::dataservice::{dataaccess::*, dbsetup::DbConnection, entities::prelude::*};
use crate::{
  app::constdata::HttpCode,
  protofiles::phoenixaccountriskcenter::PhoenixAccountAssetsInfo,
  server::service::commonservice::CommonService,
  webserver::httpresponse::{DataBody, ResponseBody},
};
use axum::extract::{Extension, Json};
use chrono::NaiveDateTime;
use common::constant;
use serde_json::Value;
use std::sync::Arc;
// use utility::constant::{self, HttpCode};
use utility::timeutil::current_date;

/// 查找运营账户的历史资产信息
/// method: post
/// parameters:
/// ```
/// {
///     "account_id":0//0表示全部，否则是单个账号
///     "start_date":20220214, //必需
///     "end_date": 20220315,//必需
/// }
/// ```
/// return array
pub async fn query_account_assets_his(Extension(db): Extension<Arc<DbConnection>>, Json(payload): Json<serde_json::Value>) -> Json<Value> {
  log::info!("query account assets history, payload is: {:?}", &payload);
  let query_dto = serde_json::from_value::<querydto::QueryDto>(payload);
  if query_dto.is_err() {
    log::error!("decode account dto error: {:?}", query_dto.err().unwrap());
    let res = ResponseBody::new(HttpCode::Error as i32, "can't decode query_dto", "");
    return Json(serde_json::json!(res));
  }
  let query_dto = query_dto.unwrap();
  if query_dto.start_date <= 0 {
    let res = ResponseBody::new(HttpCode::Error as i32, "start date must greater than 0", "");
    return Json(serde_json::json!(res));
  }
  if query_dto.end_date <= 0 {
    let res = ResponseBody::new(HttpCode::Error as i32, "end date must greater than 0", "");
    return Json(serde_json::json!(res));
  }

  if query_dto.end_date < query_dto.start_date {
    let res = ResponseBody::new(HttpCode::Error as i32, "end date must greater than start date", "");
    return Json(serde_json::json!(res));
  }

  let ret = PhoenixAccountAssetsHis::query_many_in_duration(query_dto.account_id, query_dto.start_date, query_dto.end_date, &db).await;
  if ret.is_err() {
    log::error!("query account assets error: {}", ret.as_ref().err().unwrap().to_string());
    let res = ResponseBody::new(HttpCode::Error as i32, "query error", ret.err().unwrap().to_string());
    return Json(serde_json::json!(res));
  }
  let account_infos = ret.unwrap();
  let mut ret_account_infos: Vec<PhoenixAccountAssetsInfo> = Vec::new();
  for val in account_infos {
    let view_ret = CommonService::convert_accountassetshis_to_assetsinfo(&val);
    ret_account_infos.push(view_ret);
  }
  let databody = DataBody::new(ret_account_infos.len(), ret_account_infos);
  let res = ResponseBody::new(HttpCode::OK as i32, "OK", databody);
  Json(serde_json::json!(res))
}

/// 查找分账户的持仓信息
/// method: post
/// parameters:
/// ```
/// {
///     "account_id":200849//必需
///     "start_date":20220214, //开始日期：必需
///     "end_date": 20220315,//结束日期：必需
/// }
/// ```
/// return array
pub async fn query_account_assets(Extension(db): Extension<Arc<DbConnection>>, Json(payload): Json<serde_json::Value>) -> Json<Value> {
  log::info!("query_account_assets, payload is: {:?}", &payload);
  let query_dto = serde_json::from_value::<querydto::QueryDto>(payload);
  if query_dto.is_err() {
    log::error!("decode query dto error: {:?}", query_dto.err().unwrap());
    let res = ResponseBody::new(HttpCode::Error as i32, "can't decode query_dto", "");
    return Json(serde_json::json!(res));
  }
  let query_dto = query_dto.unwrap();
  if query_dto.start_date == 0 {
    let res = ResponseBody::new(HttpCode::Error as i32, "开始日期不能为0", "");
    return Json(serde_json::json!(res));
  }
  if query_dto.end_date == 0 {
    let res = ResponseBody::new(HttpCode::Error as i32, "结束日期不能为0", "");
    return Json(serde_json::json!(res));
  }

  if query_dto.start_date > query_dto.end_date {
    let res = ResponseBody::new(HttpCode::Error as i32, "开始日期不能早于结束日期", "");
    return Json(serde_json::json!(res));
  }
  let mut current_date = current_date();
  let tsystemdate = PhoenixSysSystem::query(&db).await;
  if tsystemdate.is_ok() {
    current_date = tsystemdate.unwrap().init_date;
  }
  if query_dto.start_date < query_dto.end_date && query_dto.end_date == current_date as i64 {
    let res = ResponseBody::new(HttpCode::Error as i32, "目前暂不支持开始日期是历史日期，结束日期是当天的查询", "");
    return Json(serde_json::json!(res));
  }

  //当天数据
  if query_dto.start_date == query_dto.end_date && query_dto.end_date == current_date as i64 {
    //查询当天的数据
    let ret = PhoenixAccountAssets::query_many(query_dto.account_id, &db).await;
    if ret.is_err() {
      log::error!("error: {}", ret.as_ref().err().unwrap().to_string());
      let res = ResponseBody::new(HttpCode::Error as i32, "error", ret.err().unwrap().to_string());
      return Json(serde_json::json!(res));
    }
    let ret_infos = ret.unwrap();
    let mut assets_ret: Vec<PhoenixAccountAssetsInfo> = Vec::new();
    for val in ret_infos.iter() {
      let asset_info = CommonService::convert_accountassets_to_assetsinfo(val);
      assets_ret.push(asset_info);
    }

    let databody = DataBody::new(assets_ret.len(), assets_ret);
    log::info!("result:{:?}", &databody);
    let res = ResponseBody::new(HttpCode::OK as i32, "OK", databody);
    return Json(serde_json::json!(res));
  }

  let ret = PhoenixAccountAssetsHis::query_many_in_duration(query_dto.account_id, query_dto.start_date, query_dto.end_date, &db).await;
  if ret.is_err() {
    log::error!("query error: {}", ret.as_ref().err().unwrap().to_string());
    let res = ResponseBody::new(HttpCode::Error as i32, "query error", ret.err().unwrap().to_string());
    return Json(serde_json::json!(res));
  }
  let ret_infos = ret.unwrap();
  let mut assets_ret: Vec<PhoenixAccountAssetsInfo> = Vec::new();
  for val in ret_infos.iter() {
    let asset_info = CommonService::convert_accountassetshis_to_assetsinfo(val);
    assets_ret.push(asset_info);
  }

  let databody = DataBody::new(assets_ret.len(), assets_ret);
  log::info!("result:{:?}", &databody);
  let res = ResponseBody::new(HttpCode::OK as i32, "OK", databody);
  Json(serde_json::json!(res))
}

/// 查找分账户的持仓信息
/// method: post
/// parameters:
/// ```
/// {
///     "account_id":200849//必需
///     "start_date":20220214, //开始日期：必需
///     "end_date": 20220315,//结束日期：必需
/// }
/// ```
/// return array
pub async fn query_account_stock_positions(Extension(db): Extension<Arc<DbConnection>>, Json(payload): Json<serde_json::Value>) -> Json<Value> {
  log::info!("query account stock positions, payload is: {:?}", &payload);
  let query_dto = serde_json::from_value::<querydto::QueryDto>(payload);
  if query_dto.is_err() {
    log::error!("decode query dto error: {:?}", query_dto.err().unwrap());
    let res = ResponseBody::new(HttpCode::Error as i32, "can't decode query_dto", "");
    return Json(serde_json::json!(res));
  }
  let query_dto = query_dto.unwrap();
  if query_dto.start_date == 0 {
    let res = ResponseBody::new(HttpCode::Error as i32, "开始日期不能为0", "");
    return Json(serde_json::json!(res));
  }
  if query_dto.end_date == 0 {
    let res = ResponseBody::new(HttpCode::Error as i32, "结束日期不能为0", "");
    return Json(serde_json::json!(res));
  }

  if query_dto.start_date > query_dto.end_date {
    let res = ResponseBody::new(HttpCode::Error as i32, "开始日期不能早于结束日期", "");
    return Json(serde_json::json!(res));
  }

  let mut current_date = current_date();
  let tsystemdate = PhoenixSysSystem::query(&db).await;
  if tsystemdate.is_ok() {
    current_date = tsystemdate.unwrap().init_date;
  }
  if query_dto.start_date < query_dto.end_date && query_dto.end_date == current_date as i64 {
    let res = ResponseBody::new(HttpCode::Error as i32, "目前暂不支持开始日期是历史日期，结束日期是当天的查询", "");
    return Json(serde_json::json!(res));
  }

  //当天数据
  if query_dto.start_date == query_dto.end_date && query_dto.end_date == current_date as i64 {
    //查询当天的数据
    let ret = PhoenixStockPositionChannel::query_many(query_dto.account_id, constant::VALUE_ALL as i64, constant::VALUE_ALL as i64, &db).await;
    if ret.is_err() {
      log::error!("error: {}", ret.as_ref().err().unwrap().to_string());
      let res = ResponseBody::new(HttpCode::Error as i32, "error", ret.err().unwrap().to_string());
      return Json(serde_json::json!(res));
    }
    let ret_infos = ret.unwrap();
    let databody = DataBody::new(ret_infos.len(), ret_infos);
    log::info!("result:{:?}", &databody);
    let res = ResponseBody::new(HttpCode::OK as i32, "OK", databody);
    return Json(serde_json::json!(res));
  }

  let ret = PhoenixStockPositionChannelHis::query_many_in_duration(query_dto.account_id, query_dto.start_date, query_dto.end_date, &db).await;
  if ret.is_err() {
    log::error!("query error: {}", ret.as_ref().err().unwrap().to_string());
    let res = ResponseBody::new(HttpCode::Error as i32, "query error", ret.err().unwrap().to_string());
    return Json(serde_json::json!(res));
  }
  let ret_infos = ret.unwrap();
  let databody = DataBody::new(ret_infos.len(), ret_infos);
  let res = ResponseBody::new(HttpCode::OK as i32, "OK", databody);
  Json(serde_json::json!(res))
}

/// 查找账户的reset信息
/// method: post
/// parameters:
/// ```
/// {
///     "account_id":200849//必需
///     "start_date":20220214, //0：不限
///     "end_date": 20220315,//0：不限
/// }
/// ```
/// return array
pub async fn query_reset_his(Extension(db): Extension<Arc<DbConnection>>, Json(payload): Json<serde_json::Value>) -> Json<Value> {
  log::info!("query reset history, payload is: {:?}", &payload);
  let query_dto = serde_json::from_value::<querydto::QueryDto>(payload);
  if query_dto.is_err() {
    log::error!("decode query dto error: {:?}", query_dto.err().unwrap());
    let res = ResponseBody::new(HttpCode::Error as i32, "can't decode query_dto", "");
    return Json(serde_json::json!(res));
  }
  let query_dto = query_dto.unwrap();

  let mut start_timestamp: i64 = 0;
  if query_dto.start_date > 0 {
    let stdate_str = query_dto.start_date.to_string() + " 00:00:00";
    log::info!("start date:{}", &stdate_str);
    let stdate = NaiveDateTime::parse_from_str(stdate_str.as_str(), "%Y%m%d %H:%M:%S");
    log::info!("start date:{:?}", &stdate);
    if stdate.is_ok() {
      start_timestamp = stdate.unwrap().timestamp();
    }
  }
  let mut end_timestamp: i64 = 0;
  if query_dto.end_date > 0 {
    let date_str = query_dto.end_date.to_string() + " 00:00:00";
    log::info!("end date:{}", &date_str);
    let enddate = NaiveDateTime::parse_from_str(date_str.as_str(), "%Y%m%d %H:%M:%S");
    log::info!("end date:{:?}", &enddate);
    if enddate.is_ok() {
      end_timestamp = enddate.unwrap().timestamp();
    }
  }

  // if query_dto.end_date < query_dto.start_date {
  //     let res = ResponseBody::new(HttpCode::Error as i32, "end date must greater than start date", "");
  //     return Json(serde_json::json!(res));
  // }

  let ret = PhoenixAccountResetDetail::query_many(query_dto.account_id, start_timestamp, end_timestamp, &db).await;
  if ret.is_err() {
    log::error!("query reset error: {}", ret.as_ref().err().unwrap().to_string());
    let res = ResponseBody::new(HttpCode::Error as i32, "query error", ret.err().unwrap().to_string());
    return Json(serde_json::json!(res));
  }
  let ret_infos = ret.unwrap();
  let databody = DataBody::new(ret_infos.len(), ret_infos);
  let res = ResponseBody::new(HttpCode::OK as i32, "OK", databody);
  Json(serde_json::json!(res))
}

// /// 查找交易账户的历史资产信息
// /// method: post
// /// parameters:
// /// ```
// /// {
// ///     "account_id":200849//必需
// ///     "start_date":20220214, //必需
// ///     "end_date": 20220315,//必需
// /// }
// /// ```
// /// return array
// pub async fn query_user_assets_his(Extension(db): Extension<Arc<DbConnection>>, Json(payload): Json<serde_json::Value>) -> Json<Value> {
//   log::info!("query user assets history, payload is: {:?}", payload);
//   let query_dto = serde_json::from_value::<querydto::QueryDto>(payload);
//   if query_dto.is_err() {
//     log::error!("decode query dto error: {:?}", query_dto.err().unwrap());
//     let res = ResponseBody::new(HttpCode::Error as i32, "can't decode query_dto", "");
//     return Json(serde_json::json!(res));
//   }
//   let query_dto = query_dto.unwrap();
//   if query_dto.start_date <= 0 {
//     let res = ResponseBody::new(HttpCode::Error as i32, "start date must greater than 0", "");
//     return Json(serde_json::json!(res));
//   }
//   if query_dto.end_date <= 0 {
//     let res = ResponseBody::new(HttpCode::Error as i32, "end date must greater than 0", "");
//     return Json(serde_json::json!(res));
//   }
//   if query_dto.end_date < query_dto.start_date {
//     let res = ResponseBody::new(HttpCode::Error as i32, "end date must greater than start date", "");
//     return Json(serde_json::json!(res));
//   }
//   let ret = PhoenixUserAssetsHis::query_many_in_duration(query_dto.account_id, query_dto.start_date, query_dto.end_date, &db).await;
//   if ret.is_err() {
//     log::error!("query user assets error: {}", ret.as_ref().err().unwrap().to_string());
//     let res = ResponseBody::new(HttpCode::Error as i32, "query error", ret.err().unwrap().to_string());
//     return Json(serde_json::json!(res));
//   }
//   let ret_infos = ret.unwrap();
//   let databody = DataBody::new(ret_infos.len(), ret_infos);
//   let res = ResponseBody::new(HttpCode::OK as i32, "OK", databody);
//   Json(serde_json::json!(res))
// }

/* ====== demo ========= */
// `Query` gives you the query parameters and deserializes them.
// pub async fn query(Query(params): Query<HashMap<String, String>>) {
//     log::info!("query is: {:?}", params);
// }

// // Buffer the request body and deserialize it as JSON into a
// // `serde_json::Value`. `Json` supports any type that implements
// // `serde::Deserialize`.
// pub async fn post_json(Json(payload): Json<serde_json::Value>) -> Json<Value> {
//     log::info!("payload is: {:?}", payload);
//     let _databody = DataBody::new(2, vec!["12", "14"]);
//     let res = ResponseBody::new(20000, "message", payload);
//     Json(serde_json::json!(res))
// }
