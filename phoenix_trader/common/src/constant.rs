// use std::collections::HashMap;
// use strum_macros::EnumString;
// use std::string::ToString;
use strum_macros::{Display, EnumString};

pub const VALUE_ALL: i64 = 0;
pub const EXCHANGE_HK: i64 = 103;
pub const SYSTEM_CODE: &str = "phoenix_oms";
pub const RISK_COEFFICIENT: f32 = 0.75; //风险率计算系数
pub const LEVERAGE: f32 = 0.25; //默认保证金比率
pub const LEVER: i32 = 3; //默认杠杆

#[derive(PartialEq, Clone, Debug)]
pub enum ResultCode {
  OK = 0,
  ERROR = 9999,
}

#[derive(PartialEq, Clone, Debug)]
pub enum TradeMode {
  USER = 1,
  AGENT = 2,
}
#[derive(PartialEq, Clone, Debug)]
pub enum YesOrNo {
  NO = 0,
  YES = 1,
}

#[derive(strum_macros::Display, Debug, EnumString)]
//资产查询类型 1:只查询资产，2：只查询持仓，3：资产查询都查
pub enum MarketToCurrency {
  #[strum(to_string = "CNY")]
  XSHG = 101,
  #[strum(to_string = "CNY")]
  XSHE = 102,
  // #[strum(to_string = "CNY")]
  #[strum(to_string = "HKD")]
  XHKG = 103,
}

#[derive(strum_macros::Display, Debug, EnumString)]
//资产查询类型 1:只查询资产，2：只查询持仓，3：资产查询都查
pub enum QueryAssetsType {
  #[strum(to_string = "1")]
  Assets = 1,
  #[strum(to_string = "2")]
  Positions = 2,
  #[strum(to_string = "3")]
  Both = 3,
}

#[derive(PartialEq, Clone, Debug)]
pub enum FrozenType {
  Normal = 1,
  Temp = 2,
}

#[derive(PartialEq, Clone, Debug)]
pub enum MarketType {
  HK = 1,
  US = 2,
  HS = 3,
}

#[derive(PartialEq, Clone, Debug)]
pub enum MarketCode {
  XSHG = 101,
  XSHE = 102,
  XHKG = 103,
}

#[derive(PartialEq, Clone, Debug)]
pub enum OperateType {
  Insert = 0,
  Edit = 1,
  Delete = 2,
}

// 1:普通A股(ESA.M)-不含创业板、科创板,、新三板、中小板,
// 2.港股，
// 3.美股,
// 4.创业板（ESA.GEM)
// 5.科创板（KSH)
#[derive(PartialEq, Clone)]
pub enum StockType {
  HSAGU = 1, //普通A股(ESA.M)-不含创业板、科创板,、新三板、中小板,
  GANGGU = 2,
  MEIGU = 3, //美股
  HSCY = 4,  //创业板（ESA.GEM)
  HSKC = 5,  //科创板（KSH)
  EMETF = 6, //ETF
}

//委托类型
// 1:app下单  2:跟单  3:风控止盈止损平仓单, 4:风控总资产预警平仓单 5:pc客户端单 6:结算平仓单
// 7:管理端强平仓单,8:app清仓,9:pc清仓,10,管理员平仓,11,合约到期日强平
pub enum OrderType {
  APPORDER = 1,
  FOLLOWUP = 2,
  RISKSTOP = 3,
  RISKASSETS = 4,
  PCORDER = 5,
  SETTLEORDER = 6,
  ADMINCLEAR = 7,
  APPCLEAR = 8,
  PCCLEAR = 9,
  ADMINCLEAR2 = 10, //管理员平仓
  EXPIRECLEAR = 11,
}

// 业务类型
pub enum BusinessType {
  ALL = 0,
  FUTURE = 1,
  STOCK = 2,
  MT4 = 3,
}

// 订单消息类型
pub enum OmsMessageType {
  ORDERSTATUSCHANGED = 1,
  ASSETSADJUSTED = 2, //资产调整
  ASSETSINOUT = 3,    //出金，入金
  SETTLEMENT = 98,    //结算
}
pub enum OrderDirection {
  BUY = 1,
  SELL = 2,
}
pub enum TransDirection {
  DEC = 1,
  INC = 2,
}
pub enum ChannelType {
  EXTERNAL = 1, //上手通道
  INTERNAL = 2, //自撮合通道
}

// 1.计算被移除正常通道的可下单量，3.计算被移除不可用通道的可下单量，2. 计算未被移除通道的可下单量，4. 计算未被移除但通道已被关闭的可下单量
pub enum OrderSellChannelPriority {
  RemovedNormal = 1,
  ExistedNormal = 2,
  RemovedClosed = 3,
  ExistedClosed = 4,
}

pub enum ChannelStatus {
  CLOSED = -1, // 关闭
  NORMAL = 1,  //：正常，
  BUY = 2,     //：只买，
  SELL = 3,    //：只卖，
  OFF = 4,     //：禁买卖
}
pub enum CommidityStatus {
  Normal = 1,   //：正常，
  SellOnly = 2, //：只买，
  Closed = 3,   //：禁买卖
}

//通道状态 1：开启 2：关闭
pub enum NormalStatus {
  ON = 1,
  OFF = 2, //
}

//帐号状态 '0:待配置, 1: 正常交易 ，2：禁止开仓， 3：禁止交易(只读)，4：账号冻结(禁用)'
pub enum AccountStatus {
  AccountNotReady = 0,
  AccountNormal = 1,
  AccountBuyClosed = 2,
  AccountOrderClosed = 3,
  AccountFrozed = 4,
}

pub enum AccountChannelPriority {
  AccountOnly = 1,  /*仅按账户*/
  GroupOnly = 2,    /*仅品种组*/
  AccountFirst = 3, /*先账户后品种组*/
}

#[derive(Debug, Clone)]
pub enum RateCategory {
  CJ = 1,
  CK = 2, //
}

#[derive(strum_macros::Display, Debug, EnumString)]
pub enum CurrencyType {
  /// Random Docs
  #[strum(to_string = "CNY")]
  CNY,
  #[strum(to_string = "HKD")]
  HKD,
  #[strum(to_string = "USD")]
  USD,
}

#[derive(Copy, Debug, Clone)]
//0：分帐户, 1：主账户，2:总账户，
pub enum AccountType {
  ChannelAccount = 0,
  MainAccount = 1,
  TotalAccount = 2,
}

#[derive(Copy, Debug, Clone)]
//交易类型 1:普通股票交易， 2:融券交易
pub enum TradeType {
  Normal = 1,
  T0 = 2,
}

#[derive(Copy, Debug, Clone)]
//交易类型 1:普通股票交易， 2:融券交易
pub enum MessageType {
  DateTimeMsg = 1,
  T0 = 2,
}

#[derive(Copy, Debug, Clone)]
//phoenix_exc_orderinfo 状态 0: 未知 1：未处理  2：已处理 3：撤单
pub enum ExOrderStatus {
  Unkonw = 0,
  UnExec = 1,
  Exec = 2,
  Canceled = 3,
}

//资产调整类型
pub enum AssetChangeType {
  AccessAsset = 1,        // 1：用户出入金调整
  OperationAsset = 2,     // 2：运营手动调整资产
  TradeOrderAsset = 3,    // 3：交易产生的资产变动
  SettleAsset = 4,        // 4：交收资产变动
  EquityDistribution = 5, // 5：权益分派资产变动
  SystemAsset = 6,        // 6：系统资产调整（结息）
}
// 资产调整方向
#[derive(Copy, Debug, Clone)]
pub enum AssetChangeDirection {
  AddCapital = 101,           // 101: 资金增加
  ReduceCapital = 102,        // 102：资金减少
  FrozenCapital = 103,        // 103：资金冻结
  UnFrozenCapital = 104,      // 104：资金解冻
  FrozenTradeCapital = 105,   //105：资产临时冻结
  UnFrozenTradeCapital = 106, //105：资产临时解冻
  CreateUnit = 902,           //902：创建资产用户
  ModifyUnitCredit = 901,     //901：用户信用倍数调整
  AddPosition = 201,          // 201：持仓增加
  ReducePosition = 202,       // 202：持仓减少
  FrozenPosition = 203,       // 203：持仓冻结
  UnFrozenPosition = 204,     // 204：持仓解冻
}

#[derive(Copy, Debug, Clone)]
pub enum OrderStatus {
    INITED = 1,          //1: 未报
    SUBMITTED = 4,       //4：已报
    INVALID = 5,         //5：废单
    PARTIALDEALED = 6,   //6：部成
    DEALED = 7,          //7：已成
    PARTIALCANCELED = 8, //8：部撤
    CANCELED = 9,        //9：已撤
}
pub trait Point<T> {
    fn precision(&self, index: usize) -> T;
} 
impl<T: 'static + std::str::FromStr + std::fmt::Display + std::default::Default> Point<T> for T {
    fn precision(&self, index: usize) -> T {
        format!("{0:.1$}", self, index).parse::<T>().unwrap_or_default()
    }
}

/// # Example
/// ```
/// use crate::common::constant::RateType;
/// use std::str::FromStr;
/// // simple from string
/// let variant = RateType::from_str("Cny2Hkd").unwrap();
/// assert_eq!(RateType::Cny2Hkd, variant);
/// let variant = RateType::from_str("CNY2HKD").unwrap();
/// assert_eq!(RateType::Cny2Hkd, variant);
/// ```
///
#[derive(Debug, PartialEq, EnumString)]
pub enum RateType {
  // We can make the comparison case insensitive (however Unicode is not supported at the moment)
  #[strum(ascii_case_insensitive)]
  Cny2Hkd,
  #[strum(ascii_case_insensitive)]
  Hkd2Cny,
  #[strum(ascii_case_insensitive)]
  Usd2Hkd,
  #[strum(ascii_case_insensitive)]
  Hkd2Usd,
  #[strum(ascii_case_insensitive)]
  CnyHkdBuy,
  #[strum(ascii_case_insensitive)]
  CnyHkdSell,
  #[strum(ascii_case_insensitive)]
  UsdHkdBuy,
  #[strum(ascii_case_insensitive)]
  UsdHkdSell,
}

pub fn get_rate_key(rate_type: &RateType) -> &str {
  match rate_type {
    RateType::Cny2Hkd => "RATE_CNY_HKD_STOCK_SELL",
    RateType::Hkd2Cny => "RATE_CNY_HKD_STOCK_BUY",
    RateType::Usd2Hkd => "RATE_USD_HKD_STOCK_SELL",
    RateType::Hkd2Usd => "RATE_USD_HKD_STOCK_BUY",
    RateType::CnyHkdBuy => "RATE_CNY_HKD_STOCK_BUY",
    RateType::CnyHkdSell => "RATE_CNY_HKD_STOCK_SELL",
    RateType::UsdHkdBuy => "RATE_USD_HKD_STOCK_BUY",
    RateType::UsdHkdSell => "RATE_USD_HKD_STOCK_SELL",
  }
}

// impl std::str::FromStr for RateType {
//     type Err = ::strum::ParseError;
//     fn from_str(s: &str) -> ::core::result::Result<RateType, Self::Err> {
//         match s {
//             s if s.eq_ignore_ascii_case("CNY2HKD") => ::core::result::Result::Ok(RateType::Cny2Hkd),
//             s if s.eq_ignore_ascii_case("HKD2CNY") => ::core::result::Result::Ok(RateType::Hkd2Cny),
//             s if s.eq_ignore_ascii_case("USD2HKD") => ::core::result::Result::Ok(RateType::Usd2Hkd),
//             s if s.eq_ignore_ascii_case("HKD2USD") => ::core::result::Result::Ok(RateType::Hkd2Usd),
//             s if s.eq_ignore_ascii_case("CNYHKDBUY") => ::core::result::Result::Ok(RateType::CnyHkdBuy),
//             s if s.eq_ignore_ascii_case("CNYHKDSELL") => ::core::result::Result::Ok(RateType::CnyHkdSell),
//             s if s.eq_ignore_ascii_case("USDHKDBUY") => ::core::result::Result::Ok(RateType::UsdHkdBuy),
//             s if s.eq_ignore_ascii_case("USDHKDSELL") => ::core::result::Result::Ok(RateType::UsdHkdSell),
//             _ => ::core::result::Result::Err(::strum::ParseError::VariantNotFound),
//         }
//     }
// }

#[derive(Copy, Debug, Clone)]
pub enum TradeDate {
  Normal = 0,
  QuerySpecial = 1,
  QuerySpecial2 = 2,
}

pub const EXCHANGE_ID_HS: [i32; 2] = [101, 102]; //沪深市场id
pub const EXCHANGE_ID_HK: i32 = 103; //港股市场id

#[derive(Debug, Clone, Copy)]
pub enum DividendProcess {
  HK, //港股
  HS, //沪深
}

//
#[derive(Debug, Clone, Copy)]
pub enum DealPoint {
  DirectList,        // 直接上市
  Register,          // 登记
  ListAfterRegister, //登记后上市
}

// 红利标记
pub enum BusinFlagDividend {
  Register = 41701,          // 红利登记
  ListAfterRegister = 41702, // 登记后到账
  DirectList = 41703,        // 红利直接到帐
  RateRecal = 41704,         // 汇率轧差
}

impl TryFrom<i32> for BusinFlagDividend {
  type Error = ();

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    match value {
      v if v == BusinFlagDividend::Register as i32 => Ok(BusinFlagDividend::Register),
      v if v == BusinFlagDividend::DirectList as i32 => Ok(BusinFlagDividend::DirectList),
      v if v == BusinFlagDividend::ListAfterRegister as i32 => Ok(BusinFlagDividend::ListAfterRegister),
      _ => Err(()),
    }
  }
}

// 红股标记
pub enum BusinFlagBonusShare {
  Register = 41801,          // 红股标记
  ListAfterRegister = 41802, // 红股后到账
  DirectList = 41803,        // 红股直接到帐
  RateRecal = 41804,         // 汇率轧差
}

impl TryFrom<i32> for BusinFlagBonusShare {
  type Error = ();

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    match value {
      v if v == BusinFlagBonusShare::Register as i32 => Ok(BusinFlagBonusShare::Register),
      v if v == BusinFlagBonusShare::DirectList as i32 => Ok(BusinFlagBonusShare::DirectList),
      v if v == BusinFlagBonusShare::ListAfterRegister as i32 => Ok(BusinFlagBonusShare::ListAfterRegister),
      _ => Err(()),
    }
  }
}

// 除权标记
pub enum DividendFlag {
  Dividend = 431,   // 红利
  BonusShare = 432, // 红股
}

impl TryFrom<i32> for DividendFlag {
  type Error = ();

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    match value {
      v if v == DividendFlag::Dividend as i32 => Ok(DividendFlag::Dividend),
      v if v == DividendFlag::BonusShare as i32 => Ok(DividendFlag::BonusShare),
      _ => Err(()),
    }
  }
}
