/// -----------------------------------------股票通道最大持仓量等信息-----------------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelHoldLimitReq {
    #[prost(int64, tag="1")]
    pub stock_id: i64,
    /// 默认为0查出所有
    #[prost(int64, tag="2")]
    pub channel_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelHoldLimitResp {
    /// 返回结果
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    /// 返回结果
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<ChannelHoldLimit>,
    /// 整体最大持仓量
    #[prost(int64, tag="4")]
    pub total_max_hold: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelHoldLimit {
    /// 通道编号,
    #[prost(int64, tag="1")]
    pub channel_id: i64,
    /// 证券id,
    #[prost(int64, tag="2")]
    pub stock_id: i64,
    /// 通道最大持仓量,
    #[prost(int64, tag="3")]
    pub max_holdnum: i64,
    /// 最大持仓市值,
    #[prost(int64, tag="4")]
    pub max_holdvalue: i64,
}
/// -----------------------------------------------通道基础信息-----------------------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelInfoReq {
    /// 默认为0查出所有
    #[prost(int64, tag="1")]
    pub channel_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelInfoResp {
    /// 返回结果
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    /// 返回结果
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<ChannelInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelInfo {
    #[prost(int64, tag="1")]
    pub channel_id: i64,
    #[prost(string, tag="2")]
    pub channel_name: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub channel_state: i32,
    /// 通道类型 1：外盘通道 2：内盘通道
    #[prost(int32, tag="4")]
    pub channel_type: i32,
    /// 对应账号
    #[prost(int64, tag="5")]
    pub unit_id: i64,
    /// 是否支持qfii，用于判断交易时间
    #[prost(int32, tag="6")]
    pub qfii_state: i32,
}
/// -------------------------------------------股票通道配置优先级信息-----------------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StockChannelReq {
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 股票id
    #[prost(int64, tag="2")]
    pub stock_id: i64,
    /// 交易方向 1：买 2：卖
    #[prost(int32, tag="3")]
    pub order_direction: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StockChannelResp {
    /// 返回结果
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    /// 返回结果
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<ChannelConfig>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelConfig {
    #[prost(int64, tag="1")]
    pub id: i64,
    /// 通道ID
    #[prost(int64, tag="2")]
    pub channel_id: i64,
    /// 通道名称
    #[prost(string, tag="3")]
    pub channel_name: ::prost::alloc::string::String,
    /// 用户iD，0表示全部用户
    #[prost(int64, tag="4")]
    pub unit_id: i64,
    /// level 通道优先级
    #[prost(int32, tag="5")]
    pub channel_level: i32,
    /// 通道类型 1：内盘 2：外盘
    #[prost(int32, tag="6")]
    pub channel_type: i32,
    /// 通道状态 -1:关闭 1：正常，2：只买，3：只卖，4：禁止交易
    #[prost(int32, tag="7")]
    pub channel_status: i32,
    /// 品种组 
    #[prost(int64, tag="8")]
    pub commodity_group: i64,
    /// qf state 0: no, 1:yes
    #[prost(int32, tag="9")]
    pub qfii_state: i32,
}
/// -------------------------------------------股票基础信息以及交易时间段--------------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StockInfoReq {
    /// 股票id //默认为0查出所有
    #[prost(int64, tag="1")]
    pub stock_id: i64,
    /// 市场id
    #[prost(int32, tag="2")]
    pub exchange_id: i32,
    /// 证券代码
    #[prost(string, tag="3")]
    pub stock_code: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StockInfoResp {
    /// 返回结果
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    /// 返回结果
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<StockInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StockInfo {
    /// 股票id,
    #[prost(int64, tag="1")]
    pub stock_id: i64,
    /// 股票代码
    #[prost(string, tag="2")]
    pub stock_code: ::prost::alloc::string::String,
    /// 股票名称,
    #[prost(string, tag="3")]
    pub stock_name: ::prost::alloc::string::String,
    ///   1:正常，2：只能平仓，3：禁止交易,
    #[prost(int32, tag="4")]
    pub trade_state: i32,
    /// 证券-每手股数,
    #[prost(string, tag="5")]
    pub hands_num: ::prost::alloc::string::String,
    /// 股票类型  1:普通A股(ESA.M)-不含创业板、科创板,、新三板、中小板,2.港股， 3.美股, 4.创业板（ESA.GEM)5.科创板（KSH)
    #[prost(int32, tag="6")]
    pub stock_type: i32,
    /// 单笔最小数量
    #[prost(int32, tag="7")]
    pub min_value: i32,
    /// 单笔最大数量
    #[prost(int32, tag="8")]
    pub max_value: i32,
    /// 单笔最大金额
    #[prost(double, tag="9")]
    pub max_single_money: f64,
    /// 市场id
    #[prost(int64, tag="10")]
    pub market_id: i64,
    /// 市场code
    #[prost(string, tag="11")]
    pub market_code: ::prost::alloc::string::String,
    /// 交易时间段
    #[prost(string, tag="12")]
    pub trading_time: ::prost::alloc::string::String,
    /// 交易币种
    #[prost(enumeration="Currency", tag="13")]
    pub trade_currency: i32,
    /// 品种保证金比例
    #[prost(double, tag="14")]
    pub margin_rate: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpecialAccountInfoReq {
    #[prost(enumeration="SpecialAccountType", tag="1")]
    pub account_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpecialAccountInfoResp {
    #[prost(message, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<SpecialAccount>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpecialAccount {
    #[prost(enumeration="SpecialAccountType", tag="1")]
    pub account_type: i32,
    #[prost(int64, tag="2")]
    pub unid_id: i64,
}
/// ------------------------------------------市场信息以及交易日信息-------------------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketInfoReq {
    /// 市场id 默认0为查出所有
    #[prost(int64, tag="1")]
    pub market_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketInfoResp {
    /// 返回结果
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    /// 返回结果
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<MarketInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketInfo {
    /// 市场id
    #[prost(int64, tag="1")]
    pub market_id: i64,
    /// 币种
    #[prost(enumeration="Currency", tag="2")]
    pub currency_type: i32,
    /// 市场code
    #[prost(string, tag="3")]
    pub market_code: ::prost::alloc::string::String,
    /// 市场类型
    #[prost(int64, tag="4")]
    pub market_type: i64,
    /// 当前日期
    #[prost(int64, tag="5")]
    pub current_date: i64,
    /// 0 非交易日  1 交易日 2 半日市
    #[prost(int32, tag="6")]
    pub date_type: i32,
}
/// -----------------------------------------------临时休市信息-----------------------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketCloseInfoReq {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketCloseInfoResp {
    /// 返回结果
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    /// 返回结果
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<MarketCloseInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketCloseInfo {
    #[prost(int64, tag="1")]
    pub market_id: i64,
    /// 休市开始时间
    #[prost(string, tag="2")]
    pub start_time: ::prost::alloc::string::String,
    /// 休市结束时间
    #[prost(string, tag="3")]
    pub end_time: ::prost::alloc::string::String,
    /// 休市类型
    #[prost(int32, tag="4")]
    pub close_type: i32,
}
/// -----------------------------------------------汇率查询-----------------------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeRateReq {
    /// 交易币种
    #[prost(enumeration="Currency", tag="1")]
    pub currency: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeRateResp {
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<ExchangeRate>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeRate {
    /// 交易币种
    #[prost(enumeration="Currency", tag="1")]
    pub currency: i32,
    /// 基准币 默认为HKD
    #[prost(enumeration="Currency", tag="2")]
    pub base_currency: i32,
    /// 买入交易币
    #[prost(double, tag="3")]
    pub buy_rate: f64,
    /// 卖出交易币
    #[prost(double, tag="4")]
    pub sell_rate: f64,
    /// 汇率维护时间
    #[prost(int64, tag="5")]
    pub modify_time: i64,
}
/// -----------------------------------------------账户信息查询-----------------------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountInfoReq {
    /// 为0时查询全部交易账户
    #[prost(int64, tag="1")]
    pub unit_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountInfoResp {
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<AccountInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountInfo {
    /// 用户ID(目前系统等同于user_id)
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 接入方式 1:USER(用户直连) 2:AGENT(代理托管)
    #[prost(int32, tag="2")]
    pub trade_mode: i32,
    /// 代理账户(接入方式为2:AGENT时不能为空, 非开户代理, 原merchant)
    #[prost(int64, tag="3")]
    pub agent_account: i64,
    /// 融资杠杆比例
    #[prost(string, tag="4")]
    pub level_rate: ::prost::alloc::string::String,
    /// 到期日期
    #[prost(int64, tag="5")]
    pub expire_date: i64,
    /// 预警线
    #[prost(double, tag="6")]
    pub warning_line: f64,
    /// 平仓线
    #[prost(double, tag="7")]
    pub close_line: f64,
    /// 交易账号状态 1.正常交易  2.禁止开仓 3.禁止交易 4.账号冻结
    #[prost(int64, tag="8")]
    pub trade_state: i64,
    /// 创业板限制
    #[prost(double, tag="9")]
    pub gem_limit: f64,
}
/// ------------------------------------------交易日查询-------------------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeDateReq {
    /// 市场id 默认0为查出所有
    #[prost(int64, tag="1")]
    pub market_id: i64,
    /// 0 表示当前系统日期, 非0表示查指定日期
    #[prost(int32, tag="2")]
    pub query_date: i32,
    /// 0 日期查询  1 交易日查询  2交收日查询
    #[prost(int32, tag="3")]
    pub query_type: i32,
    /// 0 当天  1 下一日/交易日/交收日/ 2  下下一日/交易日/交收日 3.....5 最大不超过5
    #[prost(int32, tag="4")]
    pub date_offset: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeDateResp {
    /// 返回结果
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    /// 返回结果
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<TradeDateInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeDateInfo {
    /// 市场id
    #[prost(int64, tag="1")]
    pub market_id: i64,
    /// 当前系统日期
    #[prost(int32, tag="2")]
    pub current_date: i32,
    /// 根据查询条件获取到的日期
    #[prost(int32, tag="3")]
    pub target_date: i32,
    /// 交易日, 半日市 非交易市
    #[prost(int32, tag="4")]
    pub date_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserStockMarginReq {
    /// 用户ID
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 品种id
    #[prost(int64, tag="2")]
    pub stock_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserStockMarginResp {
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    /// 用户品种保证金
    #[prost(double, tag="3")]
    pub margin_rate: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeSettingReq {
    /// 费用类型 非空时必须匹配, 为空则查询全部
    #[prost(string, tag="1")]
    pub fee_type: ::prost::alloc::string::String,
    /// 市场id   请求时必传,查询询时 没有的话取通用
    #[prost(int64, tag="2")]
    pub exchange_id: i64,
    /// 委托方向 请求时必传,查询询时 没有的话取通用
    #[prost(int32, tag="3")]
    pub order_direction: i32,
    /// 用户id   请求时必传,查询询时 没有的话取通用
    #[prost(int64, tag="4")]
    pub unit_id: i64,
    /// 通道id   请求时必传,查询询时 没有的话取通用
    #[prost(int64, tag="5")]
    pub channel_id: i64,
    /// 证券类别 请求时必传,查询询时 没有的话取通用
    #[prost(int32, tag="6")]
    pub stock_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeSettingResp {
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    /// 费用类型
    #[prost(message, repeated, tag="3")]
    pub fee_settings: ::prost::alloc::vec::Vec<FeeSetting>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeSetting {
    /// 费用类型
    #[prost(string, tag="1")]
    pub fee_type: ::prost::alloc::string::String,
    /// 市场id
    #[prost(int64, tag="2")]
    pub exchange_id: i64,
    /// 委托方向
    #[prost(int32, tag="3")]
    pub order_direction: i32,
    /// 用户id
    #[prost(int64, tag="4")]
    pub unit_id: i64,
    /// 通道id
    #[prost(int64, tag="5")]
    pub channel_id: i64,
    /// 证券类别
    #[prost(int32, tag="6")]
    pub stock_type: i32,
    /// 成交金额比例
    #[prost(double, tag="7")]
    pub fee_ratio: f64,
    /// 最高费用
    #[prost(double, tag="8")]
    pub maximum_fee: f64,
    /// 最低费用
    #[prost(double, tag="9")]
    pub minimum_fee: f64,
    /// 最高/低收费币种类型: CNY/HKD/USD
    #[prost(enumeration="Currency", tag="10")]
    pub currency_type: i32,
    /// 小数位处理方式 1:四舍五入  2: 全舍  3: 全入
    #[prost(int32, tag="11")]
    pub decimal_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StockMarginReq {
    #[prost(int64, tag="1")]
    pub stock_id: i64,
    #[prost(int64, tag="2")]
    pub channel_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StockMarginResp {
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    /// 费用类型
    #[prost(message, optional, tag="3")]
    pub stock_margin: ::core::option::Option<StockMargin>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StockMargin {
    #[prost(int64, tag="1")]
    pub stock_id: i64,
    #[prost(int64, tag="2")]
    pub channel_id: i64,
    #[prost(double, tag="3")]
    pub margin_rate: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Currency {
    Undef = 0,
    Hkd = 1,
    Cny = 2,
    Usd = 3,
}
impl Currency {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Currency::Undef => "UNDEF",
            Currency::Hkd => "HKD",
            Currency::Cny => "CNY",
            Currency::Usd => "USD",
        }
    }
}
/// ---------------------------------------全部交易对手方账号和特殊账号信息---------------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SpecialAccountType {
    All = 0,
    Counterparty = 1,
}
impl SpecialAccountType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SpecialAccountType::All => "ALL",
            SpecialAccountType::Counterparty => "Counterparty",
        }
    }
}
/// Generated client implementations.
pub mod phoenix_aka_center_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PhoenixAkaCenterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PhoenixAkaCenterClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PhoenixAkaCenterClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PhoenixAkaCenterClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PhoenixAkaCenterClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn query_channel_hold_limit(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelHoldLimitReq>,
        ) -> Result<tonic::Response<super::ChannelHoldLimitResp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/phoenixakacenter.PhoenixAkaCenter/query_channel_hold_limit",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_channel_info(
            &mut self,
            request: impl tonic::IntoRequest<super::ChannelInfoReq>,
        ) -> Result<tonic::Response<super::ChannelInfoResp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/phoenixakacenter.PhoenixAkaCenter/query_channel_info",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_stock_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::StockChannelReq>,
        ) -> Result<tonic::Response<super::StockChannelResp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/phoenixakacenter.PhoenixAkaCenter/query_stock_channel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_stock_info(
            &mut self,
            request: impl tonic::IntoRequest<super::StockInfoReq>,
        ) -> Result<tonic::Response<super::StockInfoResp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/phoenixakacenter.PhoenixAkaCenter/query_stock_info",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_special_account(
            &mut self,
            request: impl tonic::IntoRequest<super::SpecialAccountInfoReq>,
        ) -> Result<tonic::Response<super::SpecialAccountInfoResp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/phoenixakacenter.PhoenixAkaCenter/query_special_account",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_market_info(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketInfoReq>,
        ) -> Result<tonic::Response<super::MarketInfoResp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/phoenixakacenter.PhoenixAkaCenter/query_market_info",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_market_close_info(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketCloseInfoReq>,
        ) -> Result<tonic::Response<super::MarketCloseInfoResp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/phoenixakacenter.PhoenixAkaCenter/query_market_close_info",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_exchange_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::ExchangeRateReq>,
        ) -> Result<tonic::Response<super::ExchangeRateResp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/phoenixakacenter.PhoenixAkaCenter/query_exchange_rate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_account_info(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountInfoReq>,
        ) -> Result<tonic::Response<super::AccountInfoResp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/phoenixakacenter.PhoenixAkaCenter/query_account_info",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_trade_date(
            &mut self,
            request: impl tonic::IntoRequest<super::TradeDateReq>,
        ) -> Result<tonic::Response<super::TradeDateResp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/phoenixakacenter.PhoenixAkaCenter/query_trade_date",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_unit_stock_margin(
            &mut self,
            request: impl tonic::IntoRequest<super::UserStockMarginReq>,
        ) -> Result<tonic::Response<super::UserStockMarginResp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/phoenixakacenter.PhoenixAkaCenter/query_unit_stock_margin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_fee_setting(
            &mut self,
            request: impl tonic::IntoRequest<super::FeeSettingReq>,
        ) -> Result<tonic::Response<super::FeeSettingResp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/phoenixakacenter.PhoenixAkaCenter/query_fee_setting",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_stock_channel_margin(
            &mut self,
            request: impl tonic::IntoRequest<super::StockMarginReq>,
        ) -> Result<tonic::Response<super::StockMarginResp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/phoenixakacenter.PhoenixAkaCenter/query_stock_channel_margin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod phoenix_aka_center_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with PhoenixAkaCenterServer.
    #[async_trait]
    pub trait PhoenixAkaCenter: Send + Sync + 'static {
        async fn query_channel_hold_limit(
            &self,
            request: tonic::Request<super::ChannelHoldLimitReq>,
        ) -> Result<tonic::Response<super::ChannelHoldLimitResp>, tonic::Status>;
        async fn query_channel_info(
            &self,
            request: tonic::Request<super::ChannelInfoReq>,
        ) -> Result<tonic::Response<super::ChannelInfoResp>, tonic::Status>;
        async fn query_stock_channel(
            &self,
            request: tonic::Request<super::StockChannelReq>,
        ) -> Result<tonic::Response<super::StockChannelResp>, tonic::Status>;
        async fn query_stock_info(
            &self,
            request: tonic::Request<super::StockInfoReq>,
        ) -> Result<tonic::Response<super::StockInfoResp>, tonic::Status>;
        async fn query_special_account(
            &self,
            request: tonic::Request<super::SpecialAccountInfoReq>,
        ) -> Result<tonic::Response<super::SpecialAccountInfoResp>, tonic::Status>;
        async fn query_market_info(
            &self,
            request: tonic::Request<super::MarketInfoReq>,
        ) -> Result<tonic::Response<super::MarketInfoResp>, tonic::Status>;
        async fn query_market_close_info(
            &self,
            request: tonic::Request<super::MarketCloseInfoReq>,
        ) -> Result<tonic::Response<super::MarketCloseInfoResp>, tonic::Status>;
        async fn query_exchange_rate(
            &self,
            request: tonic::Request<super::ExchangeRateReq>,
        ) -> Result<tonic::Response<super::ExchangeRateResp>, tonic::Status>;
        async fn query_account_info(
            &self,
            request: tonic::Request<super::AccountInfoReq>,
        ) -> Result<tonic::Response<super::AccountInfoResp>, tonic::Status>;
        async fn query_trade_date(
            &self,
            request: tonic::Request<super::TradeDateReq>,
        ) -> Result<tonic::Response<super::TradeDateResp>, tonic::Status>;
        async fn query_unit_stock_margin(
            &self,
            request: tonic::Request<super::UserStockMarginReq>,
        ) -> Result<tonic::Response<super::UserStockMarginResp>, tonic::Status>;
        async fn query_fee_setting(
            &self,
            request: tonic::Request<super::FeeSettingReq>,
        ) -> Result<tonic::Response<super::FeeSettingResp>, tonic::Status>;
        async fn query_stock_channel_margin(
            &self,
            request: tonic::Request<super::StockMarginReq>,
        ) -> Result<tonic::Response<super::StockMarginResp>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PhoenixAkaCenterServer<T: PhoenixAkaCenter> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PhoenixAkaCenter> PhoenixAkaCenterServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PhoenixAkaCenterServer<T>
    where
        T: PhoenixAkaCenter,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/phoenixakacenter.PhoenixAkaCenter/query_channel_hold_limit" => {
                    #[allow(non_camel_case_types)]
                    struct query_channel_hold_limitSvc<T: PhoenixAkaCenter>(pub Arc<T>);
                    impl<
                        T: PhoenixAkaCenter,
                    > tonic::server::UnaryService<super::ChannelHoldLimitReq>
                    for query_channel_hold_limitSvc<T> {
                        type Response = super::ChannelHoldLimitResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelHoldLimitReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_channel_hold_limit(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_channel_hold_limitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/phoenixakacenter.PhoenixAkaCenter/query_channel_info" => {
                    #[allow(non_camel_case_types)]
                    struct query_channel_infoSvc<T: PhoenixAkaCenter>(pub Arc<T>);
                    impl<
                        T: PhoenixAkaCenter,
                    > tonic::server::UnaryService<super::ChannelInfoReq>
                    for query_channel_infoSvc<T> {
                        type Response = super::ChannelInfoResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChannelInfoReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_channel_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_channel_infoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/phoenixakacenter.PhoenixAkaCenter/query_stock_channel" => {
                    #[allow(non_camel_case_types)]
                    struct query_stock_channelSvc<T: PhoenixAkaCenter>(pub Arc<T>);
                    impl<
                        T: PhoenixAkaCenter,
                    > tonic::server::UnaryService<super::StockChannelReq>
                    for query_stock_channelSvc<T> {
                        type Response = super::StockChannelResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StockChannelReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_stock_channel(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_stock_channelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/phoenixakacenter.PhoenixAkaCenter/query_stock_info" => {
                    #[allow(non_camel_case_types)]
                    struct query_stock_infoSvc<T: PhoenixAkaCenter>(pub Arc<T>);
                    impl<
                        T: PhoenixAkaCenter,
                    > tonic::server::UnaryService<super::StockInfoReq>
                    for query_stock_infoSvc<T> {
                        type Response = super::StockInfoResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StockInfoReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_stock_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_stock_infoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/phoenixakacenter.PhoenixAkaCenter/query_special_account" => {
                    #[allow(non_camel_case_types)]
                    struct query_special_accountSvc<T: PhoenixAkaCenter>(pub Arc<T>);
                    impl<
                        T: PhoenixAkaCenter,
                    > tonic::server::UnaryService<super::SpecialAccountInfoReq>
                    for query_special_accountSvc<T> {
                        type Response = super::SpecialAccountInfoResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SpecialAccountInfoReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_special_account(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_special_accountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/phoenixakacenter.PhoenixAkaCenter/query_market_info" => {
                    #[allow(non_camel_case_types)]
                    struct query_market_infoSvc<T: PhoenixAkaCenter>(pub Arc<T>);
                    impl<
                        T: PhoenixAkaCenter,
                    > tonic::server::UnaryService<super::MarketInfoReq>
                    for query_market_infoSvc<T> {
                        type Response = super::MarketInfoResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarketInfoReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_market_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_market_infoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/phoenixakacenter.PhoenixAkaCenter/query_market_close_info" => {
                    #[allow(non_camel_case_types)]
                    struct query_market_close_infoSvc<T: PhoenixAkaCenter>(pub Arc<T>);
                    impl<
                        T: PhoenixAkaCenter,
                    > tonic::server::UnaryService<super::MarketCloseInfoReq>
                    for query_market_close_infoSvc<T> {
                        type Response = super::MarketCloseInfoResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarketCloseInfoReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_market_close_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_market_close_infoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/phoenixakacenter.PhoenixAkaCenter/query_exchange_rate" => {
                    #[allow(non_camel_case_types)]
                    struct query_exchange_rateSvc<T: PhoenixAkaCenter>(pub Arc<T>);
                    impl<
                        T: PhoenixAkaCenter,
                    > tonic::server::UnaryService<super::ExchangeRateReq>
                    for query_exchange_rateSvc<T> {
                        type Response = super::ExchangeRateResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExchangeRateReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_exchange_rate(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_exchange_rateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/phoenixakacenter.PhoenixAkaCenter/query_account_info" => {
                    #[allow(non_camel_case_types)]
                    struct query_account_infoSvc<T: PhoenixAkaCenter>(pub Arc<T>);
                    impl<
                        T: PhoenixAkaCenter,
                    > tonic::server::UnaryService<super::AccountInfoReq>
                    for query_account_infoSvc<T> {
                        type Response = super::AccountInfoResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountInfoReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_account_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_account_infoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/phoenixakacenter.PhoenixAkaCenter/query_trade_date" => {
                    #[allow(non_camel_case_types)]
                    struct query_trade_dateSvc<T: PhoenixAkaCenter>(pub Arc<T>);
                    impl<
                        T: PhoenixAkaCenter,
                    > tonic::server::UnaryService<super::TradeDateReq>
                    for query_trade_dateSvc<T> {
                        type Response = super::TradeDateResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TradeDateReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_trade_date(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_trade_dateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/phoenixakacenter.PhoenixAkaCenter/query_unit_stock_margin" => {
                    #[allow(non_camel_case_types)]
                    struct query_unit_stock_marginSvc<T: PhoenixAkaCenter>(pub Arc<T>);
                    impl<
                        T: PhoenixAkaCenter,
                    > tonic::server::UnaryService<super::UserStockMarginReq>
                    for query_unit_stock_marginSvc<T> {
                        type Response = super::UserStockMarginResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserStockMarginReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_unit_stock_margin(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_unit_stock_marginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/phoenixakacenter.PhoenixAkaCenter/query_fee_setting" => {
                    #[allow(non_camel_case_types)]
                    struct query_fee_settingSvc<T: PhoenixAkaCenter>(pub Arc<T>);
                    impl<
                        T: PhoenixAkaCenter,
                    > tonic::server::UnaryService<super::FeeSettingReq>
                    for query_fee_settingSvc<T> {
                        type Response = super::FeeSettingResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FeeSettingReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_fee_setting(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_fee_settingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/phoenixakacenter.PhoenixAkaCenter/query_stock_channel_margin" => {
                    #[allow(non_camel_case_types)]
                    struct query_stock_channel_marginSvc<T: PhoenixAkaCenter>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PhoenixAkaCenter,
                    > tonic::server::UnaryService<super::StockMarginReq>
                    for query_stock_channel_marginSvc<T> {
                        type Response = super::StockMarginResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StockMarginReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_stock_channel_margin(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_stock_channel_marginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PhoenixAkaCenter> Clone for PhoenixAkaCenterServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PhoenixAkaCenter> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PhoenixAkaCenter> tonic::server::NamedService for PhoenixAkaCenterServer<T> {
        const NAME: &'static str = "phoenixakacenter.PhoenixAkaCenter";
    }
}
