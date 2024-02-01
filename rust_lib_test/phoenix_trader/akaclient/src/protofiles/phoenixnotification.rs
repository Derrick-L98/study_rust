#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationMessage {
    /// 1：时间类消息
    #[prost(enumeration="NotificationType", tag="1")]
    pub msg_type: i32,
    /// 消息内容
    #[prost(message, optional, tag="2")]
    pub msg_body: ::core::option::Option<MessageBody>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageBody {
    /// 这些都是optional的
    #[prost(message, optional, tag="1")]
    pub msg_datetime: ::core::option::Option<NotificationDatetime>,
    /// 订单状态变化
    #[prost(message, optional, tag="2")]
    pub msg_orderstatus: ::core::option::Option<NotificationOrderStatus>,
    /// 订单执行回报消息
    #[prost(message, optional, tag="3")]
    pub msg_orderexec: ::core::option::Option<NotificationOrderExec>,
    /// 资产变化
    #[prost(message, optional, tag="4")]
    pub msg_asset: ::core::option::Option<NotificationAsset>,
    /// 持仓变化
    #[prost(message, optional, tag="5")]
    pub msg_position: ::core::option::Option<NotificationPosition>,
    /// 资产调整
    #[prost(message, optional, tag="6")]
    pub msg_assetadjusted: ::core::option::Option<NotificationAssetAdjusted>,
    /// 结算通知
    #[prost(message, optional, tag="7")]
    pub msg_settlement: ::core::option::Option<NotificationSettlement>,
    /// 通道基础信息发生变化
    #[prost(message, optional, tag="8")]
    pub msg_channelinfo: ::core::option::Option<ChannelInfoChange>,
    /// 股票信息修改
    #[prost(message, optional, tag="9")]
    pub msg_stockinfo: ::core::option::Option<StockInfoChange>,
    /// 汇率变化
    #[prost(message, optional, tag="10")]
    pub msg_exchangerate: ::core::option::Option<ExchangeRateChange>,
    /// 用户信息发生变化
    #[prost(message, optional, tag="11")]
    pub msg_accountinfo: ::core::option::Option<AccountInfoChange>,
    /// 用户品种保证金变化
    #[prost(message, optional, tag="12")]
    pub msg_userstockmargin: ::core::option::Option<UserStockMarginChange>,
    /// 费用变化
    #[prost(message, optional, tag="13")]
    pub msg_feesetting: ::core::option::Option<FeeSettingChange>,
    /// 停牌信息
    #[prost(message, optional, tag="14")]
    pub msg_stocksuspension: ::core::option::Option<StockSuspensionChange>,
    /// 用户客户端推送
    #[prost(message, optional, tag="15")]
    pub msg_userclientnotify: ::core::option::Option<UserClientNotifyDataChange>,
    /// 订单详情
    #[prost(message, optional, tag="16")]
    pub msg_orderinfo: ::core::option::Option<NotificationOrderInfo>,
    /// 公告通知
    #[prost(message, optional, tag="17")]
    pub msg_notice: ::core::option::Option<NoticeDataChange>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationDatetime {
    /// 1:休市 ,2:交易时间
    #[prost(int32, tag="1")]
    pub datetime_type: i32,
    /// 市场id
    #[prost(int64, tag="2")]
    pub market_id: i64,
    /// 品种id 品种为0推送的是市场的
    #[prost(int64, tag="3")]
    pub stock_id: i64,
    /// 时间，时间格式为 2020-12-01 09:30:00|2020-12-01 15:30:00,2020-12-02 09:30:00|2020-12-02 15:30:00
    #[prost(string, tag="4")]
    pub msg_time: ::prost::alloc::string::String,
    /// 交易时间时，1：开仓，2：平仓
    #[prost(int32, tag="5")]
    pub trade_type: i32,
}
/// 订单状态变化
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationOrderStatus {
    /// 用户id
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 订单ID
    #[prost(int64, tag="2")]
    pub order_id: i64,
    /// 发生数量
    #[prost(int32, tag="3")]
    pub order_quantity: i32,
    #[prost(double, tag="4")]
    pub order_price: f64,
    /// 1:新订单  2:成交  3:撤单
    #[prost(string, tag="5")]
    pub order_action: ::prost::alloc::string::String,
    /// 当前状态
    #[prost(int32, tag="6")]
    pub order_status: i32,
    /// 成交编号
    #[prost(int64, tag="7")]
    pub deal_no: i64,
    /// 备注
    #[prost(string, tag="8")]
    pub memo: ::prost::alloc::string::String,
}
/// 订单成交撤单回报
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationOrderExec {
    /// 用户id
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 订单ID
    #[prost(int64, tag="2")]
    pub order_id: i64,
    /// 股票id
    #[prost(int64, tag="3")]
    pub stock_id: i64,
    /// 通道id
    #[prost(int32, tag="4")]
    pub channel_id: i32,
    /// 通道类别
    #[prost(int32, tag="5")]
    pub channel_type: i32,
    /// 发生数量
    #[prost(int32, tag="6")]
    pub order_quantity: i32,
    /// 成交价格
    #[prost(double, tag="7")]
    pub order_price: f64,
    /// 订单方向
    #[prost(int32, tag="8")]
    pub order_direction: i32,
    /// 1:新订单  2:成交  3:撤单
    #[prost(enumeration="OrderExecType", tag="9")]
    pub exec_type: i32,
    /// 成交编号
    #[prost(int64, tag="10")]
    pub deal_no: i64,
    /// 备注
    #[prost(string, tag="11")]
    pub memo: ::prost::alloc::string::String,
    /// 消息id
    #[prost(int64, tag="12")]
    pub msg_id: i64,
}
/// 订单详情    topic key：  order.info.unit_id
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationOrderInfo {
    /// 用户id
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 委托方向
    #[prost(int32, tag="2")]
    pub order_direction: i32,
    /// 证券代码
    #[prost(string, tag="3")]
    pub stock_code: ::prost::alloc::string::String,
    /// 委托价格
    #[prost(double, tag="4")]
    pub order_price: f64,
    /// 委托编号
    #[prost(int64, tag="5")]
    pub order_no: i64,
    /// 委托类别
    #[prost(int32, tag="6")]
    pub order_type: i32,
    /// 成交均价
    #[prost(double, tag="7")]
    pub deal_price: f64,
    /// 委托数量
    #[prost(int32, tag="8")]
    pub order_amount: i32,
    /// 成交数量
    #[prost(int32, tag="9")]
    pub deal_amount: i32,
    /// 已撤数量
    #[prost(int32, tag="10")]
    pub canceled_amount: i32,
    /// 委托时间 2023-02-01 14:35:25
    #[prost(string, tag="11")]
    pub create_time: ::prost::alloc::string::String,
    /// 最后成交时间 2023-02-01 14:35:25
    #[prost(string, tag="12")]
    pub last_deal_time: ::prost::alloc::string::String,
    /// 订单状态
    #[prost(int32, tag="13")]
    pub order_status: i32,
    /// 证券id
    #[prost(int64, tag="14")]
    pub stock_id: i64,
    /// 废单原因
    #[prost(string, tag="15")]
    pub order_memo: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationAssetPositions {
    #[prost(message, optional, tag="1")]
    pub assets: ::core::option::Option<NotificationAsset>,
    #[prost(message, optional, tag="2")]
    pub positions: ::core::option::Option<NotificationPosition>,
}
/// 资产变化
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationAsset {
    /// 用户id
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 当前本金
    #[prost(double, tag="2")]
    pub current_cash: f64,
    /// 冻结资金
    #[prost(double, tag="3")]
    pub frozen_capital: f64,
    /// 交易临时冻结
    #[prost(double, tag="4")]
    pub trade_frozen_capital: f64,
    /// 期初本金
    #[prost(double, tag="5")]
    pub begin_cash: f64,
    /// 在途资金
    #[prost(double, tag="6")]
    pub cash_in_transit: f64,
    /// 币种
    #[prost(string, tag="7")]
    pub currency_no: ::prost::alloc::string::String,
    /// 信用倍数
    #[prost(double, tag="8")]
    pub credit_multiple: f64,
    #[prost(int64, tag="9")]
    pub timestamp: i64,
    /// 今日入金
    #[prost(double, tag="10")]
    pub today_deposit: f64,
    /// 今日出金
    #[prost(double, tag="11")]
    pub today_withdraw: f64,
    /// 总入金
    #[prost(double, tag="12")]
    pub total_deposit: f64,
    /// 总出金
    #[prost(double, tag="13")]
    pub total_withdraw: f64,
    /// 昨日本金
    #[prost(double, tag="14")]
    pub today_total_value: f64,
    /// 创业板挂单保证金占用
    #[prost(double, tag="15")]
    pub gem_frozen_capital: f64,
}
/// 持仓变化
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationPosition {
    #[prost(int64, tag="1")]
    pub position_no: i64,
    /// 用户id
    #[prost(int64, tag="2")]
    pub unit_id: i64,
    /// 证券代码
    #[prost(string, tag="3")]
    pub stock_code: ::prost::alloc::string::String,
    /// 证券id
    #[prost(int64, tag="4")]
    pub stock_id: i64,
    /// 市场ID
    #[prost(int64, tag="5")]
    pub exchange_id: i64,
    /// 1多 2空
    #[prost(int32, tag="6")]
    pub position_flag: i32,
    /// 期初数量
    #[prost(int32, tag="7")]
    pub begin_amount: i32,
    /// 当前数量
    #[prost(int32, tag="8")]
    pub current_amount: i32,
    /// 冻结数量
    #[prost(int32, tag="9")]
    pub frozen_amount: i32,
    /// 临时冻结数量
    #[prost(int32, tag="10")]
    pub temp_frozen_amount: i32,
    /// 今买数量
    #[prost(int32, tag="11")]
    pub buy_amount: i32,
    /// 今卖数量
    #[prost(int32, tag="12")]
    pub sale_amount: i32,
    /// 预买数量
    #[prost(int32, tag="13")]
    pub prebuy_amount: i32,
    /// 预卖数量
    #[prost(int32, tag="14")]
    pub presale_amount: i32,
    /// 在途持仓数量(买)
    #[prost(int32, tag="15")]
    pub buy_in_transit: i32,
    /// 在途持仓数量(卖)
    #[prost(int32, tag="16")]
    pub sale_in_transit: i32,
    /// 通道id
    #[prost(int64, tag="17")]
    pub channel_id: i64,
    /// 股票类别
    #[prost(int32, tag="18")]
    pub stock_type: i32,
    /// 保证金比例
    #[prost(double, tag="19")]
    pub margin_rate: f64,
    /// 开仓成本;
    #[prost(double, tag="20")]
    pub total_value: f64,
    /// 港币开仓成本
    #[prost(double, tag="21")]
    pub total_value_hkd: f64,
    /// qf持仓数量
    #[prost(int32, tag="22")]
    pub qfii_amount: i32,
    #[prost(int64, tag="23")]
    pub timestamp: i64,
    /// 最新价
    #[prost(double, tag="24")]
    pub last_price: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationAssetAdjusted {
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 资金业务大类：1：用户出入金调整 2：运营手动调整资产 3：交易产生的资产变动 4：交收资产变动 5：权益分派资产变动 6：系统资产调整（结息）
    #[prost(int32, tag="2")]
    pub business_flag: i32,
    /// 操作类型，101：本金增加，102：本金减少，103：本金冻结，104：本金解冻，105：交易金额临时冻结，106：交易金额临时解冻，901：信用倍数调整，902：创建资产用户
    #[prost(int32, tag="3")]
    pub op_type: i32,
    /// 调整的金额
    #[prost(double, tag="4")]
    pub adjusted_value: f64,
    /// 备注
    #[prost(string, tag="5")]
    pub memo: ::prost::alloc::string::String,
    #[prost(int64, tag="6")]
    pub msg_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationSettlement {
    /// 结算状态  1: 开始  2: 完成
    #[prost(int32, tag="1")]
    pub settle_status: i32,
    /// 市场类别: 0:全部 1:A股 2:港股 3:美股  
    #[prost(int32, tag="2")]
    pub exchange_type: i32,
}
/// 通道基础信息发生变化   exchanger:    topic key：  channel_info_change.{channel_id}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelInfoChange {
    #[prost(int64, tag="1")]
    pub channel_id: i64,
    #[prost(string, tag="2")]
    pub channel_name: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub channel_state: i32,
    /// 通道类型 1：外盘通道 2：内盘通道
    #[prost(int32, tag="4")]
    pub channel_type: i32,
    /// 分账号id
    #[prost(int64, tag="5")]
    pub account_id: i64,
    /// 是否支持qfii，用于判断交易时间
    #[prost(int32, tag="6")]
    pub qfii_state: i32,
}
/// 股票信息修改  topic key: stock_info_change.{stock_id}  ,重新获取query_channel_hold_limit和query_stock_info
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StockInfoChange {
    /// 证券id,
    #[prost(int64, tag="1")]
    pub stock_id: i64,
}
/// 汇率发生变化  topic key: exchange_rate_change
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeRateChange {
    /// 交易币种
    #[prost(int32, tag="1")]
    pub currency: i32,
    /// 基准币 默认为HKD
    #[prost(int32, tag="2")]
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
/// 用户信息发生变更  topic key: account_info_change.{unit_id}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountInfoChange {
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
/// 用户品种保证金变化 topic key: unit_stock_margin_change
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserStockMarginChange {
    /// 用户id
    #[prost(message, repeated, tag="1")]
    pub margin_data: ::prost::alloc::vec::Vec<UserStockMarginData>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserStockMarginData {
    /// 用户id
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 用户id
    #[prost(int64, tag="2")]
    pub stock_id: i64,
    /// 用户品种保证金
    #[prost(double, tag="3")]
    pub margin_rate: f64,
}
/// 费用变更 topic key: fee_setting_change
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeSettingChange {
    #[prost(int64, repeated, tag="1")]
    pub id: ::prost::alloc::vec::Vec<i64>,
}
/// 停牌信息  topic_key :  stock_suspension
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StockSuspensionChange {
    #[prost(message, repeated, tag="1")]
    pub stock_suspension: ::prost::alloc::vec::Vec<StockSuspensionData>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StockSuspensionData {
    /// 证券id
    #[prost(int64, tag="1")]
    pub stock_id: i64,
    /// 停牌日期
    #[prost(int32, tag="2")]
    pub susp_date: i32,
    /// 处理日期
    #[prost(int32, tag="3")]
    pub process_date: i32,
    /// 保证金增量
    #[prost(double, tag="4")]
    pub margin_rate_increment: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserClientNotifyDataChange {
    /// 用户id
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 品种id
    #[prost(int64, tag="2")]
    pub stock_id: i64,
    /// 保证金比例
    #[prost(double, tag="3")]
    pub margin_rate: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoticeDataChange {
    /// 公告内容
    #[prost(string, tag="1")]
    pub content: ::prost::alloc::string::String,
}
/// 通知类别
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotificationType {
    UndefNotificationType = 0,
    /// 交易时间调整
    TimeChanged = 1,
    /// 订单状态变化
    OrderStatusChanged = 2,
    /// 订单执行回报消息
    OrderExecMsg = 3,
    /// 资产变化（变化后的结果数据）
    AssetChanged = 4,
    /// 资产调整（调整的内容（不是结果数据））
    AssetAdjusted = 5,
    /// 持仓变化
    PositionChanged = 6,
    /// 持仓变化（调整内容）- 类型暂时保留
    PositionAdjusted = 7,
    /// 结算
    Settlement = 8,
    /// 通道基础信息发生变化
    ChannelInfoChanged = 9,
    StockInfoChanged = 10,
    ExchangeRateChanged = 11,
    AccountInfoChanged = 12,
    UserStockMarginChanged = 13,
    FeeSettingChanged = 14,
    StockSuspensionChanged = 15,
    /// 通知客户端保证金比例发生变化
    UserClientNofity = 16,
    /// 客户订单详情
    UserOrderInfo = 17,
    /// 公告通知
    NoticeNotify = 18,
}
impl NotificationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NotificationType::UndefNotificationType => "UndefNotificationType",
            NotificationType::TimeChanged => "TimeChanged",
            NotificationType::OrderStatusChanged => "OrderStatusChanged",
            NotificationType::OrderExecMsg => "OrderExecMsg",
            NotificationType::AssetChanged => "AssetChanged",
            NotificationType::AssetAdjusted => "AssetAdjusted",
            NotificationType::PositionChanged => "PositionChanged",
            NotificationType::PositionAdjusted => "PositionAdjusted",
            NotificationType::Settlement => "Settlement",
            NotificationType::ChannelInfoChanged => "ChannelInfoChanged",
            NotificationType::StockInfoChanged => "StockInfoChanged",
            NotificationType::ExchangeRateChanged => "ExchangeRateChanged",
            NotificationType::AccountInfoChanged => "AccountInfoChanged",
            NotificationType::UserStockMarginChanged => "UserStockMarginChanged",
            NotificationType::FeeSettingChanged => "FeeSettingChanged",
            NotificationType::StockSuspensionChanged => "StockSuspensionChanged",
            NotificationType::UserClientNofity => "UserClientNofity",
            NotificationType::UserOrderInfo => "UserOrderInfo",
            NotificationType::NoticeNotify => "NoticeNotify",
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderExecType {
    UndefExecType = 0,
    /// 新订单
    NewOrder = 1,
    /// 订单成交
    OrderFill = 2,
    /// 撤单
    OrderCancelled = 3,
}
impl OrderExecType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderExecType::UndefExecType => "UndefExecType",
            OrderExecType::NewOrder => "NewOrder",
            OrderExecType::OrderFill => "OrderFill",
            OrderExecType::OrderCancelled => "OrderCancelled",
        }
    }
}
/// 定义枚举类型，
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationType {
    Insert = 0,
    Edit = 1,
    Delete = 2,
}
impl OperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationType::Insert => "INSERT",
            OperationType::Edit => "EDIT",
            OperationType::Delete => "DELETE",
        }
    }
}
