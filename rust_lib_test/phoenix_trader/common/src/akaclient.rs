use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Ok, Result};
use cached::Cached;
use chrono::{Local, TimeZone};
use dashmap::DashMap;
use rust_decimal::prelude::ToPrimitive;
use std::sync::Arc;
use tokio::sync::RwLock;
use utility::timeutil;

use tonic::transport::Channel;

use cached::stores::TimedCache;

use crate::constant;
use crate::protofiles::phoenixakacenter::phoenix_aka_center_client::PhoenixAkaCenterClient;
use crate::protofiles::phoenixakacenter::*;
// use crate::protofiles::phoenixakacenter::{
//     AccountInfo, AccountInfoReq, AccountInfoResp, ChannelConfig, ChannelHoldLimit, ChannelHoldLimitReq, ChannelHoldLimitResp, ChannelInfo, ChannelInfoReq, ExchangeRateReq, ExchangeRateResp,
//     FeeSetting, FeeSettingReq, FeeSettingResp, MarketCloseInfoReq, MarketCloseInfoResp, MarketInfo, MarketInfoReq, MarketInfoResp, SpecialAccount, SpecialAccountInfoReq, SpecialAccountInfoResp,
//     StockChannelReq, StockChannelResp, StockInfo, StockInfoReq, StockInfoResp, TradeDateReq, TradeDateResp, UserStockMarginReq, UserStockMarginResp, TradeDateInfo,
// };

#[derive(Debug, Clone, Default)]
pub struct Rate {
  pub buy_rate: f64,
  pub sell_rate: f64,
  pub modify_time: i64,
}

#[derive(Debug, Clone, Default)]
pub struct TradeTime {
  pub op_type: i32,
  pub begin: String,
  pub end: String,
}

impl TradeTime {
  fn build(input: &StockTradeTime) -> Vec<TradeTime> {
    // let begintime: i64;
    // let endtime: i64;
    let mut tradetimes: Vec<TradeTime> = vec![];
    input.times.split(",").into_iter().for_each(|v| {
      let timestr: Vec<&str> = v.split("|").collect();
      if timestr.len() != 2 {
        panic!("err");
      }
      // timeutil
      tradetimes.push(TradeTime {
        op_type: input.op_type,
        begin: timestr[0].to_owned(),
        end: timestr[1].to_owned(),
      })
    });

    tradetimes
  }
}

#[derive(Debug, Clone)]
pub struct AkaClient {
  url: String, // logcenter地址
  cache_flag: bool,
  client: Arc<RwLock<PhoenixAkaCenterClient<Channel>>>,

  channels: DashMap<i64, ChannelInfo>,
  markets: DashMap<i64, MarketInfo>,
  accounts: Arc<RwLock<TimedCache<i64, AccountInfo>>>,
  stocks: Arc<RwLock<TimedCache<i64, StockInfo>>>,
  stockcodes: DashMap<String, i64>,
  spaccounts: DashMap<i64, Vec<AccountInfo>>,
  rates: Arc<RwLock<TimedCache<String, ExchangeRate>>>,
  stocktradetimes: DashMap<i64, DashMap<i32, Vec<TradeTime>>>,
  marktetradetimes: DashMap<i64, DashMap<i32, Vec<TradeTime>>>,
}

impl AkaClient {
  pub async fn init(url: String, cache_flag: bool, cache_time: i64) -> Self {
    let mut client = PhoenixAkaCenterClient::connect(url.clone()).await.expect("connect aka server error");
    // if client.as_ref().is_err() {
    //   return Err(anyhow!("connect to akacenter failed"));
    // }

    //初始化时，可以通过全部从后台获取数据进行缓存
    // 通道信息
    let stocks = Arc::new(RwLock::new(TimedCache::with_lifespan(cache_time as u64)));
    let stockcodes = DashMap::new();

    let accounts = Arc::new(RwLock::new(TimedCache::with_lifespan(cache_time as u64)));
    let rates = Arc::new(RwLock::new(TimedCache::with_lifespan(cache_time as u64)));

    let channels = DashMap::new();
    let markets = DashMap::new();
    let marktetradetimes = DashMap::new();

    if cache_flag {
      let channelreq = ChannelInfoReq { channel_id: 0 };
      let channelresponse = client.query_channel_info(channelreq).await.expect("init channel error").into_inner();
      channelresponse.data.into_iter().for_each(|v| {
        channels.insert(v.channel_id, v);
      });
      //   // 股票信息
      //   let stockinforeq = StockInfoReq {
      //     stock_id: 0,
      //     exchange_id: 0,
      //     stock_code: "".to_owned(),
      //   };
      //   let stockinforesp = client.query_stock_info(stockinforeq).await.expect("init stockinfo error").into_inner();
      //   stockinforesp.data.into_iter().for_each(|v| {
      //     stocks.cache_set(v.stock_id, v);
      //   });

      // 账户信息
      //   let accountinforeq = AccountInfoReq { unit_id: 0 };
      //   let accountinforesp = client.query_account_info(accountinforeq).await.expect("init accountinfo error").into_inner();
      //   accountinforesp.data.into_iter().for_each(|v| {
      //     accounts.insert(v.unit_id, v);
      //   });
    }

    //
    // 市场信息
    let marketinforeq = MarketInfoReq { market_id: 0 };
    let marketinforesp = client.query_market_info(marketinforeq).await.expect("init marketinfo error").into_inner();
    marketinforesp.data.clone().into_iter().for_each(|v| {
      markets.insert(v.market_id, v);
    });

    for m in marketinforesp.data {
      let tradetimereq = StockTradeTimeReq { stock_id: 0, exchange_id: m.market_id };
      let marketinforesp = client.query_stock_trade_time(tradetimereq).await.expect("init marketinfo error").into_inner();

      let busselltimes: DashMap<i32, Vec<TradeTime>> = DashMap::new();
      let mut buytimes: Vec<TradeTime> = vec![];
      let mut selltimes: Vec<TradeTime> = vec![];
      marketinforesp.tradetimes.into_iter().for_each(|v| match v.op_type {
        1 => buytimes = TradeTime::build(&v),
        2 => selltimes = TradeTime::build(&v),
        _ => log::error!("unknow op_type:"),
      });
      if buytimes.is_empty() || selltimes.is_empty() {
        panic!("market: {:?} trade time is empty", m.market_id);
      }
      busselltimes.insert(1, buytimes);
      busselltimes.insert(2, selltimes);
      marktetradetimes.insert(m.market_id, busselltimes);
    }
    //......
    let aka_cient = Arc::new(RwLock::new(client));
    Self {
      url,
      client: aka_cient,
      cache_flag,
      markets,
      channels,
      accounts,
      stocks,
      stockcodes,
      rates,
      marktetradetimes,
      spaccounts: DashMap::new(),
      stocktradetimes: DashMap::new(),
    }
  }

  //股票通道最大持仓量等信息
  pub async fn query_channel_hold_limit(&self, stock_id: i64, channel_id: i64) -> Result<(i64, Vec<ChannelHoldLimit>)> {
    log::info!("query_channel_hold_limit stock_id:{:?} channel_id:{:?}", stock_id, channel_id);

    let mut client_wr = self.client.write().await;
    let response = client_wr.query_channel_hold_limit(ChannelHoldLimitReq { stock_id, channel_id }).await;
    if response.as_ref().is_err() {
      log::error!("failed:{:?}", response.unwrap_err());
      return Err(anyhow!("failed"));
    }
    let resp = response.unwrap().into_inner();
    if resp.ret_code != 0 {
      log::error!("query_stock_channel failed: code:{:?} msg:{:?}", resp.ret_code, resp.ret_msg);
      return Err(anyhow!("failed"));
    }

    Ok((resp.total_max_hold, resp.data))
  }

  //通道基础信息
  pub async fn query_channel_info(&self, channel_id: i64) -> Result<ChannelInfo> {
    log::info!("query_channel_info {:?}", channel_id);
    if channel_id == constant::VALUE_ALL {
      return Err(anyhow!("error"));
    }
    let request = ChannelInfoReq { channel_id };
    //优先从缓存取
    let channelinfo = self.channels.get(&request.channel_id);
    if channelinfo.is_none() {
      let mut client_wr = self.client.write().await;
      let response = client_wr.query_channel_info(request.to_owned()).await;
      // client_wr.downgrade();

      if response.as_ref().is_err() {
        log::error!("failed:{:?}", response.unwrap_err());
        return Err(anyhow!("failed"));
      }
      let ret = response.unwrap().into_inner();
      if ret.ret_code != 0 {
        log::error!("query_channel_info failed: code:{:?} msg:{:?}", ret.ret_code, ret.ret_msg);
        return Err(anyhow!("failed"));
      }

      ret.data.into_iter().for_each(|data| {
        self.channels.insert(data.channel_id, data);
      });
    }

    let value = self.channels.get(&request.channel_id);
    if value.as_ref().is_none() {
      return Err(anyhow!("can't find channel info by id:{}", &request.channel_id));
    }
    Ok(value.unwrap().to_owned())
  }

  //查全部通道基础信息
  pub async fn query_all_channel_info(&self) -> Result<Vec<ChannelInfo>> {
    log::info!("query_all_channel_info ");

    let mut client_wr = self.client.write().await;
    let request = ChannelInfoReq { channel_id: 0 };

    let response = client_wr.query_channel_info(request.to_owned()).await;
    if response.as_ref().is_err() {
      log::error!("failed:{:?}", response.unwrap_err());
      return Err(anyhow!("failed"));
    }
    let ret = response.unwrap().into_inner();
    if ret.ret_code != 0 {
      log::error!("query_all_channel_info failed: code:{:?} msg:{:?}", ret.ret_code, ret.ret_msg);
      return Err(anyhow!("failed"));
    }
    self.channels.clear();

    ret.data.into_iter().for_each(|data| {
      self.channels.insert(data.channel_id, data);
    });

    let res: Vec<ChannelInfo> = self.channels.iter().map(|val| async_global_executor::block_on(async { val.to_owned() })).collect();
    Ok(res)
  }

  pub fn query_currency_by_exchangeid(&self, exchangeid: i64) -> String {
    let ret = match exchangeid {
      101 | 102 => "CNY".to_string(),
      103 => "HKD".to_string(),
      _ => "".to_string(),
    };
    ret
  }

  //股票通道配置优先级信息
  pub async fn query_stock_channel(&self, request: StockChannelReq) -> Result<Vec<ChannelConfig>> {
    log::info!("query_stock_channel request: {:?}", request);

    let mut client_wr = self.client.write().await;
    let response = client_wr.query_stock_channel(request).await;
    if response.as_ref().is_err() {
      log::error!("ailed:{:?}", response.unwrap_err());
      return Err(anyhow!("failed"));
    }
    let resp = response.unwrap().into_inner();
    if resp.ret_code != 0 {
      log::error!("query_stock_channel: code:{:?} msg:{:?}", resp.ret_code, resp.ret_msg);
      return Err(anyhow!("failed"));
    }
    Ok(resp.data)
  }

  //
  pub async fn query_stock_info_by_code(&self, stock_code: &str, market_code: &str) -> Option<StockInfo> {
    log::info!("query_stock_info_by_code  stock_code:{:?}  market_code:{:?}", stock_code, market_code);
    if stock_code.is_empty() || market_code.is_empty() {
      log::error!("stock_code or market_code is empty");
      return None;
    }

    let market = self.query_market_info_by_code(market_code).await;
    if market.as_ref().is_none() {
      log::error!("query market_id by code:{:?} error", market_code);
      return None;
    }
    let market_id = market.unwrap().market_id;
    let mut findflag: bool = false;
    let stockcode = format!("{}_{}", stock_code, market_code);

    let mut stock_id: i64 = 0;
    {
      let stockid = self.stockcodes.get(&stockcode);
      if !stockid.is_none() {
        findflag = true;
        stock_id = stockid.unwrap().to_owned();
      }
    };

    if !findflag {
      let request = StockInfoReq {
        stock_id: -1,
        exchange_id: market_id,
        stock_code: stock_code.to_owned(),
      };
      let mut client_wr = self.client.write().await;
      let response = client_wr.query_stock_info(request).await;
      if response.as_ref().is_err() {
        log::error!("ailed:{:?}", response.unwrap_err());
        return None;
      }

      let ret = response.unwrap().into_inner();
      if ret.ret_code != 0 {
        log::error!("query_stock_channel failed: code:{:?} msg:{:?}", ret.ret_code, ret.ret_msg);
        return None;
      }

      {
        let mut stocks_wr = self.stocks.write().await;
        ret.data.into_iter().for_each(|data| {
          stocks_wr.cache_set(data.stock_id, data.to_owned());
          self.stockcodes.insert(stockcode.clone(), data.stock_id);
        });
        let stockid = self.stockcodes.get(&stockcode);
        if stockid.is_none() {
          log::error!("query query_stock_info_by_code get none");
          return None;
        }
        stock_id = stockid.unwrap().to_owned();
        let stockinfo = stocks_wr.cache_get(&stock_id);
        if stockinfo.is_none() {
          log::error!("query query_stock_info_by_code get none");
          return None;
        }
        return Some(stockinfo.unwrap().to_owned());
      }
    }
    let stockinfo = self.query_stock_info(stock_id).await;
    if stockinfo.as_ref().is_err() {
      log::error!("query query_stock_info_by_code get none");
      None
    } else {
      Some(stockinfo.unwrap())
    }
  }

  //股票基础信息以及交易时间段
  pub async fn query_stock_info(&self, stock_id: i64) -> Result<StockInfo> {
    log::info!("query_stock_info  stock_id:{:?}", stock_id);
    if stock_id == constant::VALUE_ALL {
      return Err(anyhow!("error"));
    }

    let request = StockInfoReq {
      stock_id,
      exchange_id: 0,
      stock_code: "".to_owned(),
    };

    let mut findflag: bool = false;
    let mut value = StockInfo { ..Default::default() };
    {
      let mut stock_rd = self.stocks.write().await;
      let stockinfo = stock_rd.cache_get(&stock_id);
      if !stockinfo.is_none() {
        // return Ok(stockinfo.unwrap().to_owned());
        findflag = true;
        value = stockinfo.unwrap().to_owned();
      }
    };

    if !findflag {
      let mut client_wr = self.client.write().await;
      //   let client = PhoenixAkaCenterClient::connect(self.url.clone()).await;
      let response = client_wr.query_stock_info(request).await;
      if response.as_ref().is_err() {
        log::error!("ailed:{:?}", response.unwrap_err());
        return Err(anyhow!("failed"));
      }

      let ret = response.unwrap().into_inner();
      if ret.ret_code != 0 {
        log::error!("query_stock_info failed: code:{:?} msg:{:?}", ret.ret_code, ret.ret_msg);
        return Err(anyhow!("failed"));
      }

      {
        let mut stocks_wr = self.stocks.write().await;
        ret.data.into_iter().for_each(|data| {
          stocks_wr.cache_set(data.stock_id, data);
        });
        let stockinfo = stocks_wr.cache_get(&stock_id);
        if stockinfo.is_none() {
          return Err(anyhow!("failed"));
        }
        value = stockinfo.unwrap().to_owned();
      }
    }

    Ok(value)
  }

  //全部交易对手方账号和特殊账号信息
  pub async fn query_special_account(&self, account_type: i32) -> Result<Vec<SpecialAccount>> {
    log::info!("query_special_account  account_type:{:?}", account_type);

    let mut client_wr = self.client.write().await;
    let response = client_wr.query_special_account(SpecialAccountInfoReq { account_type }).await;
    if response.as_ref().is_err() {
      log::error!("ailed:{:?}", response.unwrap_err());
      return Err(anyhow!("failed"));
    }
    let resp = response.unwrap().into_inner();
    if resp.ret_code != 0 {
      log::error!("query_special_account failed: code:{:?} msg:{:?}", resp.ret_code, resp.ret_msg);
      return Err(anyhow!("failed"));
    }
    Ok(resp.accounts)
  }

  //市场信息以及交易日信息
  pub async fn query_market_info(&self, market_id: i64) -> Result<MarketInfo> {
    log::info!("query_market_info market_id:{:?}", market_id);
    if market_id == constant::VALUE_ALL {
      return Err(anyhow!("error"));
    }
    let request = MarketInfoReq { market_id };
    //优先从缓存取
    let marketinfo = self.markets.get(&request.market_id);
    if marketinfo.is_none() {
      let mut client_wr = self.client.write().await;
      let response = client_wr.query_market_info(request.to_owned()).await;
      // client_wr.downgrade();

      if response.as_ref().is_err() {
        log::error!("failed:{:?}", response.unwrap_err());
        return Err(anyhow!("failed"));
      }
      let ret = response.unwrap().into_inner();
      if ret.ret_code != 0 {
        log::error!("query_market_info failed: code:{:?} msg:{:?}", ret.ret_code, ret.ret_msg);
        return Err(anyhow!("failed"));
      }

      ret.data.into_iter().for_each(|data| {
        self.markets.insert(data.market_id, data);
      });
    }

    let value = self.markets.get(&request.market_id);
    if value.as_ref().is_none() {
      return Err(anyhow!("can't find market info by id:{}", &request.market_id));
    }
    Ok(value.unwrap().to_owned())
  }

  //根据市场代码查询 市场信息以及交易日信息
  pub async fn query_market_info_by_code(&self, market_code: &str) -> Option<MarketInfo> {
    log::info!("query_market_info_by_code market_code:{:?}", market_code);

    if !self.cache_flag {
      let mut client_wr = self.client.write().await;
      let response = client_wr.query_market_info(MarketInfoReq { market_id: 0 }).await;
      if response.as_ref().is_err() {
        log::error!("failed:{:?}", response.unwrap_err());
        return None;
      }
      let ret = response.unwrap().into_inner();
      if ret.ret_code != 0 {
        log::error!("query_market_info_by_code failed: code:{:?} msg:{:?}", ret.ret_code, ret.ret_msg);
        return None;
      }

      self.markets.clear();

      ret.data.into_iter().for_each(|data| {
        self.markets.insert(data.market_id, data);
      });
    }
    let res = self.markets.iter().find(|m| m.market_code == market_code);
    if res.is_none() {
      return None;
    }
    Some(res.unwrap().to_owned())
  }

  //查询全部市场信息
  pub async fn query_all_market_info(&self, market_id: i64) -> Result<Vec<MarketInfo>> {
    log::info!("query_all_market_info ");

    let mut client_wr = self.client.write().await;
    let request = MarketInfoReq { market_id: 0 };

    let response = client_wr.query_market_info(request.to_owned()).await;
    if response.as_ref().is_err() {
      log::error!("failed:{:?}", response.unwrap_err());
      return Err(anyhow!("failed"));
    }
    let ret = response.unwrap().into_inner();
    if ret.ret_code != 0 {
      log::error!("query_all_market_info failed: code:{:?} msg:{:?}", ret.ret_code, ret.ret_msg);
      return Err(anyhow!("failed"));
    }

    self.markets.clear();

    ret.data.into_iter().for_each(|data| {
      self.markets.insert(data.market_id, data);
    });

    let res: Vec<MarketInfo> = self.markets.iter().map(|val| async_global_executor::block_on(async { val.to_owned() })).collect();
    Ok(res)
  }

  //临时休市信息
  pub async fn query_market_close_info(&self) -> Result<Vec<MarketCloseInfo>> {
    log::info!("query_market_close_info");

    let mut client_wr = self.client.write().await;

    let response = client_wr.query_market_close_info(MarketCloseInfoReq {}).await;
    if response.as_ref().is_err() {
      log::error!("ailed:{:?}", response.unwrap_err());
      return Err(anyhow!("failed"));
    }
    let resp = response.unwrap().into_inner();
    if resp.ret_code != 0 {
      log::error!("query_market_close_info failed: code:{:?} msg:{:?}", resp.ret_code, resp.ret_msg);
      return Err(anyhow!("failed"));
    }
    Ok(resp.data)
  }

  // 汇率查询
  pub async fn query_exchange_rate(&self, currency: &str) -> Result<Rate> {
    log::info!("query_exchange_rate currency:{:?}", currency);
    let mut req = ExchangeRateReq::default();

    match currency.to_uppercase().as_str() {
      "HKD" => {
        return Ok(Rate {
          buy_rate: 1.0,
          sell_rate: 1.0,
          modify_time: 0,
        });
      }
      "CNY" => req.currency = Currency::Cny as i32,
      "USD" => req.currency = Currency::Usd as i32,
      &_ => return Err(anyhow!("unknow currency type")),
    };

    let mut findflag: bool = false;
    let mut value = ExchangeRate { ..Default::default() };
    {
      let mut rate_rd = self.rates.write().await;
      let exch_rate = rate_rd.cache_get(&currency.to_string());
      if !exch_rate.is_none() {
        // return Ok(exch_rate.unwrap().to_owned());
        findflag = true;
        value = exch_rate.unwrap().to_owned();
      }
    };

    if !findflag {
      let mut client_wr = self.client.write().await;
      //   let client = PhoenixAkaCenterClient::connect(self.url.clone()).await;
      let response = client_wr.query_exchange_rate(req).await;
      if response.as_ref().is_err() {
        log::error!("ailed:{:?}", response.unwrap_err());
        return Err(anyhow!("failed"));
      }

      let ret = response.unwrap().into_inner();
      if ret.ret_code != 0 {
        log::error!("query_exchange_rate failed: code:{:?} msg:{:?}", ret.ret_code, ret.ret_msg);
        return Err(anyhow!("failed"));
      }
      {
        let mut rates_wr = self.rates.write().await;
        ret.data.into_iter().for_each(|data| {
          rates_wr.cache_set(currency.to_owned(), data);
        });
        let exch_rate = rates_wr.cache_get(&currency.to_string());
        if exch_rate.is_none() {
          return Err(anyhow!("failed"));
        }
        value = exch_rate.unwrap().to_owned();
      }
    }

    Ok(Rate {
      buy_rate: value.buy_rate,
      sell_rate: value.sell_rate,
      modify_time: value.modify_time,
    })
  }

  // 账户信息查询
  pub async fn query_account_info(&self, unit_id: i64) -> Result<AccountInfo> {
    log::info!("query_account_info unit_id:{:?}", unit_id);
    if unit_id == constant::VALUE_ALL {
      return Err(anyhow!("error"));
    }
    let request = AccountInfoReq { unit_id };

    let mut findflag: bool = false;
    let mut value = AccountInfo { ..Default::default() };
    {
      let mut accounts_rd = self.accounts.write().await;
      let accountinfo = accounts_rd.cache_get(&unit_id);
      if !accountinfo.is_none() {
        // return Ok(accountinfo.unwrap().to_owned());
        findflag = true;
        value = accountinfo.unwrap().to_owned();
      }
    };

    if !findflag {
      let mut client_wr = self.client.write().await;
      //   let client = PhoenixAkaCenterClient::connect(self.url.clone()).await;
      let response = client_wr.query_account_info(request).await;
      if response.as_ref().is_err() {
        log::error!("ailed:{:?}", response.unwrap_err());
        return Err(anyhow!("failed"));
      }

      let ret = response.unwrap().into_inner();
      if ret.ret_code != 0 {
        log::error!("query_account_info failed: code:{:?} msg:{:?}", ret.ret_code, ret.ret_msg);
        return Err(anyhow!("failed"));
      }

      {
        let mut accounts_wr = self.accounts.write().await;
        ret.data.into_iter().for_each(|data| {
          accounts_wr.cache_set(data.unit_id, data);
        });
        let accountinfo = accounts_wr.cache_get(&unit_id);
        if accountinfo.is_none() {
          return Err(anyhow!("failed"));
        }
        value = accountinfo.unwrap().to_owned();
      }
    }

    Ok(value)
  }

  // 查询全部账户信息查询
  pub async fn query_all_account_info(&self) -> Result<Vec<AccountInfo>> {
    log::info!("query_all_account_info",);

    let mut client_wr = self.client.write().await;

    let response = client_wr.query_account_info(AccountInfoReq { unit_id: 0 }).await;
    if response.as_ref().is_err() {
      log::error!("ailed:{:?}", response.unwrap_err());
      return Err(anyhow!("failed"));
    }
    let resp = response.unwrap().into_inner();
    if resp.ret_code != 0 {
      log::error!("query_all_account_info failed: code:{:?} msg:{:?}", resp.ret_code, resp.ret_msg);
      return Err(anyhow!("failed"));
    }
    Ok(resp.data)
  }

  // 交易日信息查询
  pub async fn query_trade_date(&self, market_id: i64, query_date: i32, query_type: i32, date_offset: i32) -> Result<Vec<TradeDateInfo>> {
    log::info!("query_trade_date market_id:{:?} query_date:{:?} query_type:{:?} date_offset:{:?}", market_id, query_date, query_type, date_offset);
    let mut client_wr = self.client.write().await;

    let response = client_wr
      .query_trade_date(TradeDateReq {
        market_id,
        query_date,
        query_type,
        date_offset,
      })
      .await;
    if response.as_ref().is_err() {
      log::error!("ailed:{:?}", response.unwrap_err());
      return Err(anyhow!("failed"));
    }

    let resp = response.unwrap().into_inner();
    if resp.ret_code != 0 {
      log::error!("query_trade_date failed: code:{:?} msg:{:?}", resp.ret_code, resp.ret_msg);
      return Err(anyhow!("failed"));
    }
    Ok(resp.data)
  }

  //查询用户品种的保证金
  pub async fn query_unit_stock_margin(&self, unit_id: i64, stock_id: i64) -> Result<f64> {
    log::info!("query_unit_stock_margin unit_id:{:?} stock_id:{:?}", unit_id, stock_id);

    let mut client_wr = self.client.write().await;

    let response = client_wr.query_unit_stock_margin(UserStockMarginReq { unit_id, stock_id }).await;
    if response.as_ref().is_err() {
      log::error!("ailed:{:?}", response.unwrap_err());
      return Err(anyhow!("failed"));
    }
    let resp = response.unwrap().into_inner();
    if resp.ret_code != 0 {
      log::error!("query_unit_stock_margin failed: code:{:?} msg:{:?}", resp.ret_code, resp.ret_msg);
      return Err(anyhow!("failed"));
    }
    Ok(resp.margin_rate)
  }

  //查询品种通道保证金比例
  pub async fn query_stock_channel_margin(&self, stock_id: i64, channel_id: i64) -> Result<f64> {
    log::info!("query_stock_channel_margin stock_id:{:?} channel_id:{:?}", stock_id, channel_id,);

    let mut client_wr = self.client.write().await;

    let response = client_wr.query_stock_channel_margin(StockMarginReq { stock_id, channel_id }).await;
    if response.as_ref().is_err() {
      log::error!("ailed:{:?}", response.unwrap_err());
      return Err(anyhow!("failed"));
    }
    let resp = response.unwrap().into_inner();
    if resp.ret_code != 0 {
      log::error!("query_stock_channel_margin failed: code:{:?} msg:{:?}", resp.ret_code, resp.ret_msg);
      return Err(anyhow!("failed"));
    }
    Ok(resp.stock_margin.unwrap_or(StockMargin { ..Default::default() }).margin_rate)
  }

  // 查询费用设置
  pub async fn query_fee_setting(&self, fee_type: String, exchange_id: i64, order_direction: i32, unit_id: i64, channel_id: i64, stock_type: i32) -> Result<Vec<FeeSetting>> {
    log::info!(
      "query_fee_setting fee_type:{:?} exchange_id:{:?} order_direction:{:?} unit_id:{:?} channel_id:{:?} stock_type:{:?}",
      fee_type,
      exchange_id,
      order_direction,
      unit_id,
      channel_id,
      stock_type
    );

    let mut client_wr = self.client.write().await;

    let response = client_wr
      .query_fee_setting(FeeSettingReq {
        fee_type,
        exchange_id,
        order_direction,
        unit_id,
        channel_id,
        stock_type,
      })
      .await;
    if response.as_ref().is_err() {
      log::error!("ailed:{:?}", response.unwrap_err());
      return Err(anyhow!("failed"));
    }
    let resp = response.unwrap().into_inner();
    if resp.ret_code != 0 {
      log::error!("query_fee_setting failed: code:{:?} msg:{:?}", resp.ret_code, resp.ret_msg);
      return Err(anyhow!("failed"));
    }
    Ok(resp.fee_settings)
  }

  //查询股票停牌信息
  pub async fn query_stock_suspension_info(&self) -> Result<Vec<StockSuspension>> {
    log::info!("query_stock_suspension_info");

    let mut client_wr = self.client.write().await;

    let response = client_wr.query_stock_suspension_info(StockSuspensionReq {}).await;
    if response.as_ref().is_err() {
      log::error!("ailed:{:?}", response.unwrap_err());
      return Err(anyhow!("failed"));
    }
    let resp = response.unwrap().into_inner();
    if resp.ret_code != 0 {
      log::error!("query_stock_suspension_info failed: code:{:?} msg:{:?}", resp.ret_code, resp.ret_msg);
      return Err(anyhow!("failed"));
    }
    Ok(resp.stock_suspension)
  }

  //查询股票交易时间
  pub async fn query_stock_trade_time(&self, stock_id: i64, order_direction: i32) -> Result<Vec<TradeTime>> {
    log::info!("query_stock_trade_time stock_id:{:?}  order_direction:{:?}", stock_id, order_direction);

    // 优先从缓存中获取
    {
      let sttradetimes = self.stocktradetimes.get(&stock_id);
      if !sttradetimes.is_none() {
        // return Ok(accountinfo.unwrap().to_owned());
        let sttradetimes = sttradetimes.unwrap();
        let sttradetime = sttradetimes.get(&order_direction);
        if !sttradetime.is_none() {
          let value = sttradetime.unwrap().to_owned();
          let fv: Vec<TradeTime> = value.into_iter().filter(|v| v.op_type == order_direction).collect();
          if fv.len() > 0 {
            return Ok(fv);
          }
        }
      }
    };

    //缓存中不存在 则重新获取
    {
      let mut client_wr = self.client.write().await;
      let response = client_wr.query_stock_trade_time(StockTradeTimeReq { stock_id: stock_id, exchange_id: 0 }).await;
      if response.as_ref().is_err() {
        log::error!("ailed:{:?}", response.unwrap_err());
        return Err(anyhow!("failed"));
      }
      let resp = response.unwrap().into_inner();
      if resp.ret_code != 0 {
        log::error!("query_stock_trade_time failed: code:{:?} msg:{:?}", resp.ret_code, resp.ret_msg);
        return Err(anyhow!("failed"));
      }

      let mut buytimes: Vec<TradeTime> = vec![];
      let mut selltimes: Vec<TradeTime> = vec![];
      resp.tradetimes.into_iter().for_each(|v| match v.op_type {
        1 => buytimes = TradeTime::build(&v),
        2 => selltimes = TradeTime::build(&v),
        _ => log::error!("unknow op_type:"),
      });
      if buytimes.is_empty() || selltimes.is_empty() {
        // 不存在则取市场的交易时间
        let stockinfo = self.query_stock_info(stock_id).await;
        if stockinfo.as_ref().is_err() {
          log::error!("query_stock_trade_time failed:query stock-info err");
          return Err(anyhow!("query stock-info err"));
        }
        let stock = stockinfo.unwrap();
        let markettradetime = self.marktetradetimes.get(&stock.market_id);
        if markettradetime.as_ref().is_none() {
          log::error!("query_stock_trade_time failed:get market trade time err");
          return Err(anyhow!("get market trade time err"));
        }
        // let fv: Vec<TradeTime> = markettradetime.unwrap().to_owned().into_iter().filter(|v| v.op_type == order_direction).collect();
        // if fv.is_empty() {
        //   return Err(anyhow!("get market trade time err"));
        // }

        // let mut marketbuytimes: Vec<TradeTime> = vec![];
        // let mut marketselltimes: Vec<TradeTime> = vec![];

        // markettradetime.unwrap().value().into_iter().for_each(|v| match v.op_type {
        //   1 => marketbuytimes.push(v.clone()),
        //   2 => marketselltimes.push(v.clone()),
        //   _ => log::error!("unknow op_type:"),
        // });
        let markettradetime = markettradetime.unwrap();
        let marketbuytimes = markettradetime.value().get(&1);
        if marketbuytimes.as_ref().is_none() {
          log::error!("query_stock_trade_time failed:marketbuytimes is none");
          return Err(anyhow!("marketbuytimes is none"));
        }
        let marketselltimes = markettradetime.value().get(&2);
        if marketselltimes.as_ref().is_none() {
          log::error!("query_stock_trade_time failed:marketselltimes is none");
          return Err(anyhow!("marketselltimes is none"));
        }

        if buytimes.is_empty() {
          buytimes = marketbuytimes.unwrap().value().clone();
        }
        if selltimes.is_empty() {
          selltimes = marketselltimes.unwrap().value().clone();
        }
      }
      let stocktradetime = DashMap::new();
      stocktradetime.insert(1, buytimes);
      stocktradetime.insert(2, selltimes);
      self.stocktradetimes.insert(stock_id, stocktradetime);
    }

    let tradetimes = self.stocktradetimes.get(&stock_id);
    if tradetimes.as_ref().is_none() {
      log::error!("query_stock_trade_time failed:查询股票交易时间为空");
      return Err(anyhow!("failed"));
    }
    let tradetimes = tradetimes.unwrap();
    let tradetime = tradetimes.value().get(&order_direction);
    if tradetime.as_ref().is_none() {
      log::error!("query_stock_trade_time failed:查询交易时间为空");
      return Err(anyhow!("failed"));
    }
    let fv = tradetime.unwrap().value().to_owned();
    if fv.len() == 0 {
      log::error!("query_stock_trade_time failed:查询交易时间结果为空");
      return Err(anyhow!("failed"));
    }
    Ok(fv)
  }

  // 交易时间检查
  pub async fn trade_time_check(&self, stock_id: i64, order_direction: i32) -> bool {
    let mut check_pass: bool = false;
    let tradetimes = self.query_stock_trade_time(stock_id, order_direction).await;
    if tradetimes.as_ref().is_err() {
      log::error!("trade_time_check: stock_id({:?}) order_direction({:?}) 查询交易时间出错", stock_id, order_direction);
      return false;
    }
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("get_current_unix_err").as_secs().to_i64().unwrap();
    let tradetimes = tradetimes.unwrap();
    for tt in tradetimes {
      let tradebegin = time_parser(&tt.begin);
      let tradeend = time_parser(&tt.end);
      if tradebegin.as_ref().is_err() || tradeend.as_ref().is_err() {
        log::error!("trade_time_check: stock_id({:?}) order_direction({:?}) time_parser error", stock_id, order_direction);
        return false;
      }

      if current_time >= tradebegin.unwrap() && current_time < tradeend.unwrap() {
        check_pass = true;
        break;
      }
    }
    check_pass
  }
}

// 根据币种代码获取币种类别
pub fn get_currency_by_code(currency: &str) -> Currency {
  match currency.to_uppercase().as_str() {
    "HKD" => Currency::Hkd,
    "CNY" => Currency::Cny,
    "USD" => Currency::Usd,
    &_ => Currency::Undef,
  }
}

pub fn time_parser(time_without_date: &str) -> Result<i64> {
  let timewithdate: String;
  let datefmt = "%Y%m%d";
  let fullfmt = "%Y%m%d %H:%M:%S";
  let nowdate = Local::now().format(datefmt);
  let c = time_without_date.matches(":").count() as i32;
  if c == 1 {
    timewithdate = format!("{} {}:00", nowdate, time_without_date);
  } else if c == 2 {
    timewithdate = format!("{} {}", nowdate, time_without_date);
  } else {
    return Err(anyhow!("time format error:{:?}", time_without_date));
  }

  let datetime = Local.datetime_from_str(&timewithdate, fullfmt);
  if datetime.is_err() {
    return Err(anyhow!("time parser error:{:?}", timewithdate));
  }
  Ok(datetime.unwrap().to_owned().timestamp())
}
