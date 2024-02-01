#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HqMsgReq {
    /// ctp行情
    #[prost(message, optional, tag="1")]
    pub ctphqinfo: ::core::option::Option<CtpHqInfo>,
    /// 易盛行情
    #[prost(message, optional, tag="2")]
    pub yshqinfo: ::core::option::Option<YsHqInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CtpHqInfo {
    /// 交易日
    #[prost(string, tag="1")]
    pub trading_day: ::prost::alloc::string::String,
    /// 合约代码
    #[prost(string, tag="2")]
    pub instrument_id: ::prost::alloc::string::String,
    /// 市场代码
    #[prost(string, tag="3")]
    pub exchange_id: ::prost::alloc::string::String,
    /// 合约在交易所的代码
    #[prost(string, tag="4")]
    pub exchange_inst_id: ::prost::alloc::string::String,
    /// 最新价
    #[prost(double, tag="5")]
    pub last_price: f64,
    /// 上次结算价
    #[prost(double, tag="6")]
    pub pre_settlement_price: f64,
    /// 昨收盘
    #[prost(double, tag="7")]
    pub pre_close_price: f64,
    /// 昨持仓量
    #[prost(double, tag="8")]
    pub pre_open_interest: f64,
    /// 今开盘
    #[prost(double, tag="9")]
    pub open_price: f64,
    /// 最高价
    #[prost(double, tag="10")]
    pub highest_price: f64,
    /// 最低价
    #[prost(double, tag="11")]
    pub lowest_price: f64,
    /// 数量
    #[prost(int32, tag="12")]
    pub volume: i32,
    /// 成交金额
    #[prost(double, tag="13")]
    pub turnover: f64,
    /// 持仓量
    #[prost(double, tag="14")]
    pub open_interest: f64,
    /// 今收盘
    #[prost(double, tag="15")]
    pub close_price: f64,
    /// 本次结算价
    #[prost(double, tag="16")]
    pub settlement_price: f64,
    /// 涨停板价
    #[prost(double, tag="17")]
    pub upper_limit_price: f64,
    /// 跌停板价
    #[prost(double, tag="18")]
    pub lower_limit_price: f64,
    /// 昨虚实度
    #[prost(double, tag="19")]
    pub pre_delta: f64,
    /// 今虚实度
    #[prost(double, tag="20")]
    pub curr_delta: f64,
    /// 最后修改时间
    #[prost(string, tag="21")]
    pub update_time: ::prost::alloc::string::String,
    /// 最后修改毫秒
    #[prost(int32, tag="22")]
    pub update_millisec: i32,
    /// 申买价一
    #[prost(double, tag="23")]
    pub bid_price1: f64,
    /// 申买量一
    #[prost(int32, tag="24")]
    pub bid_volume1: i32,
    /// 申卖价一
    #[prost(double, tag="25")]
    pub ask_price1: f64,
    /// 申卖量一
    #[prost(int32, tag="26")]
    pub ask_volume1: i32,
    /// 申买价二
    #[prost(double, tag="27")]
    pub bid_price2: f64,
    /// 申买量二
    #[prost(int32, tag="28")]
    pub bid_volume2: i32,
    /// 申卖价二
    #[prost(double, tag="29")]
    pub ask_price2: f64,
    /// 申卖量二
    #[prost(int32, tag="30")]
    pub ask_volume2: i32,
    /// 申买价三
    #[prost(double, tag="31")]
    pub bid_price3: f64,
    /// 申买量三
    #[prost(int32, tag="32")]
    pub bid_volume3: i32,
    /// 申卖价三
    #[prost(double, tag="33")]
    pub ask_price3: f64,
    /// 申卖量三
    #[prost(int32, tag="34")]
    pub ask_volume3: i32,
    /// 申买价四
    #[prost(double, tag="35")]
    pub bid_price4: f64,
    /// 申买量四
    #[prost(int32, tag="36")]
    pub bid_volume4: i32,
    /// 申卖价四
    #[prost(double, tag="37")]
    pub ask_price4: f64,
    /// 申卖量四
    #[prost(int32, tag="38")]
    pub ask_volume4: i32,
    /// 申买价五
    #[prost(double, tag="39")]
    pub bid_price5: f64,
    /// 申买量五
    #[prost(int32, tag="40")]
    pub bid_volume5: i32,
    /// 申卖价五
    #[prost(double, tag="41")]
    pub ask_price5: f64,
    /// 申卖量五
    #[prost(int32, tag="42")]
    pub ask_volume5: i32,
    /// 当日均价
    #[prost(double, tag="43")]
    pub average_price: f64,
    /// 业务日期
    #[prost(string, tag="44")]
    pub action_day: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YsHqInfo {
    /// 市场代码
    #[prost(string, tag="1")]
    pub exchange_id: ::prost::alloc::string::String,
    /// 品种编号
    #[prost(string, tag="2")]
    pub commodity_no: ::prost::alloc::string::String,
    /// 合约代码
    #[prost(string, tag="3")]
    pub contract_no1: ::prost::alloc::string::String,
    /// 币种编号
    #[prost(string, tag="4")]
    pub currency_no: ::prost::alloc::string::String,
    /// 时间戳
    #[prost(string, tag="5")]
    pub tapidtstamp: ::prost::alloc::string::String,
    /// 昨结算价
    #[prost(double, tag="6")]
    pub q_pre_settle_price: f64,
    /// 昨持仓量
    #[prost(int64, tag="7")]
    pub q_pre_position_qty: i64,
    /// 开盘价
    #[prost(double, tag="8")]
    pub q_opening_price: f64,
    /// 最新价
    #[prost(double, tag="9")]
    pub q_last_price: f64,
    /// 最高价
    #[prost(double, tag="10")]
    pub q_high_price: f64,
    /// 最低价
    #[prost(double, tag="11")]
    pub q_low_price: f64,
    /// 涨停价
    #[prost(double, tag="12")]
    pub q_limit_up_price: f64,
    /// 跌停价
    #[prost(double, tag="13")]
    pub q_limit_down_price: f64,
    /// 当日总成交量
    #[prost(int64, tag="14")]
    pub q_total_qty: i64,
    /// 当日成交金额
    #[prost(double, tag="15")]
    pub q_total_turnover: f64,
    /// 持仓量
    #[prost(int64, tag="16")]
    pub q_position_qty: i64,
    /// 均价
    #[prost(double, tag="17")]
    pub q_average_price: f64,
    /// 收盘价
    #[prost(double, tag="18")]
    pub q_closing_price: f64,
    /// 最新成交量
    #[prost(double, tag="19")]
    pub q_last_qty: f64,
    /// 买价1-5档
    #[prost(double, repeated, tag="20")]
    pub q_bid_price: ::prost::alloc::vec::Vec<f64>,
    /// 买量1-5档
    #[prost(int64, repeated, tag="21")]
    pub q_bid_qty: ::prost::alloc::vec::Vec<i64>,
    /// 卖价1-5档
    #[prost(double, repeated, tag="22")]
    pub q_ask_price: ::prost::alloc::vec::Vec<f64>,
    /// 卖量1-5档
    #[prost(int64, repeated, tag="23")]
    pub q_ask_qty: ::prost::alloc::vec::Vec<i64>,
    /// 涨幅
    #[prost(double, tag="24")]
    pub q_change_rate: f64,
    /// 涨跌值
    #[prost(double, tag="25")]
    pub q_change_value: f64,
    /// 昨收价
    #[prost(double, tag="26")]
    pub q_pre_closing_price: f64,
    /// 委买总量
    #[prost(int64, tag="27")]
    pub q_total_bid_qty: i64,
    /// 委卖总量
    #[prost(int64, tag="28")]
    pub q_total_ask_qty: i64,
}
