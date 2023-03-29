/// ===========================================================================
/// +++++++++++++++++++++++++++++++++++消息包+++++++++++++++++++++++++++++++++++
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouterMsg {
    /// 消息类型:见MsgType
    #[prost(enumeration="MsgType", tag="1")]
    pub msg_type: i32,
    /// 与msg_type对应
    #[prost(message, optional, tag="2")]
    pub msg_content: ::core::option::Option<MsgContent>,
    /// 消息ID
    #[prost(int64, tag="3")]
    pub msg_id: i64,
    /// 消息创建时间
    #[prost(int64, tag="4")]
    pub msg_time: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgContent {
    /// 注册
    #[prost(message, optional, tag="1")]
    pub register_req: ::core::option::Option<RegisterReq>,
    /// 下单消息
    #[prost(message, optional, tag="2")]
    pub order_msg: ::core::option::Option<OrderMsg>,
    /// 交易所回报
    #[prost(message, optional, tag="3")]
    pub exec_msg: ::core::option::Option<ExecMsg>,
    /// 响应请求消息
    #[prost(message, optional, tag="4")]
    pub resp: ::core::option::Option<ReqResp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterReq {
    #[prost(int64, repeated, tag="1")]
    pub channel_id: ::prost::alloc::vec::Vec<i64>,
}
// +++++++++++++++++++++++++++++++++++消息体+++++++++++++++++++++++++++++++++++

/// ----------------------------------下单请求----------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderMsg {
    /// 订单类型:见OrderType
    #[prost(enumeration="OrderType", tag="1")]
    pub order_type: i32,
    /// 委托id
    #[prost(int64, tag="2")]
    pub order_id: i64,
    /// 委托确认号
    #[prost(string, tag="3")]
    pub brk_order_id: ::prost::alloc::string::String,
    /// 证券代码
    #[prost(string, tag="4")]
    pub stock_code: ::prost::alloc::string::String,
    /// 市场id
    #[prost(int64, tag="5")]
    pub exchange_id: i64,
    /// 委托方向  1=buy  2=sell
    #[prost(int32, tag="6")]
    pub order_direction: i32,
    /// 订单数量或撤单数量
    #[prost(int32, tag="7")]
    pub order_qty: i32,
    /// 价格类型(市价限价)
    #[prost(int32, tag="8")]
    pub price_type: i32,
    /// 委托价格
    #[prost(double, tag="9")]
    pub order_price: f64,
    /// 币种
    #[prost(string, tag="10")]
    pub currency_no: ::prost::alloc::string::String,
    /// 通道类型 1 外盘, 2 内盘
    #[prost(int32, tag="11")]
    pub channel_type: i32,
    /// 通道id
    #[prost(int64, tag="12")]
    pub channel_id: i64,
    /// 撤单id
    #[prost(int64, tag="13")]
    pub cancel_id: i64,
}
// ----------------------------------交易所回报----------------------------------

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecMsg {
    /// 回报类别: 见ExecType
    #[prost(enumeration="ExecType", tag="1")]
    pub exec_type: i32,
    /// 委托序号
    #[prost(int64, tag="2")]
    pub order_id: i64,
    /// 本次成交数量或撤单数量
    #[prost(int32, tag="3")]
    pub exec_qty: i32,
    /// 本次成交价格
    #[prost(double, tag="4")]
    pub exec_price: f64,
    /// 成交编号
    #[prost(string, tag="5")]
    pub exec_id: ::prost::alloc::string::String,
    /// 发生时间  2019-12-19 11:21:19
    #[prost(string, tag="6")]
    pub exec_time: ::prost::alloc::string::String,
    /// 订单方向  1=Buy, 2=Sell
    #[prost(int32, tag="7")]
    pub order_direction: i32,
    /// 委托确认号
    #[prost(string, tag="8")]
    pub brk_order_id: ::prost::alloc::string::String,
    /// 通道id
    #[prost(int64, tag="9")]
    pub channel_id: i64,
    /// 1 外盘, 2 内盘
    #[prost(int32, tag="10")]
    pub channel_type: i32,
    /// 备注
    #[prost(string, tag="11")]
    pub memo: ::prost::alloc::string::String,
}
/// ----------------------------------请求响应----------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqResp {
    /// 与请求消息ID对应
    #[prost(int64, tag="1")]
    pub msg_id: i64,
    #[prost(int32, tag="2")]
    pub error_code: i32,
    #[prost(string, tag="3")]
    pub error_msg: ::prost::alloc::string::String,
}
/// ===========================================================================
/// ----------------------------------常量定义----------------------------------
/// 消息类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgType {
    /// 注册报盘
    Register = 0,
    /// 订单消息
    Order = 1,
    /// 订单执行回报
    Exec = 2,
    /// 请求响应
    Response = 999,
}
impl MsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MsgType::Register => "Register",
            MsgType::Order => "Order",
            MsgType::Exec => "Exec",
            MsgType::Response => "Response",
        }
    }
}
/// 订单类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    /// 未定义
    Undef = 0,
    /// 委托下单
    Place = 1,
    /// 撤销委托
    Cancel = 2,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Undef => "Undef",
            OrderType::Place => "Place",
            OrderType::Cancel => "Cancel",
        }
    }
}
/// 回报类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecType {
    /// 未定义
    ExecUndef = 0,
    /// 确认回报
    Confirm = 1,
    /// 成交
    Filled = 2,
    /// 撤单回报
    Canceled = 3,
    /// 下单拒绝
    Rejected = 4,
}
impl ExecType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecType::ExecUndef => "ExecUndef",
            ExecType::Confirm => "Confirm",
            ExecType::Filled => "Filled",
            ExecType::Canceled => "Canceled",
            ExecType::Rejected => "Rejected",
        }
    }
}
