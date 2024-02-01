#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationMessage {
    /// 1：时间类消息
    #[prost(int32, tag="1")]
    pub msg_type: i32,
    /// 消息内容
    #[prost(message, optional, tag="2")]
    pub msg_body: ::core::option::Option<MessageBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageBody {
    /// 这些都是optional的
    #[prost(message, optional, tag="1")]
    pub msg_datetime: ::core::option::Option<NotificationDatetime>,
    #[prost(message, optional, tag="2")]
    pub msg_orders: ::core::option::Option<NotificationOrders>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationDatetime {
    /// 0:新增，1：修改，2：删除
    #[prost(enumeration="OperationType", tag="1")]
    pub operation_type: i32,
    /// 1:休市 2:节假日
    #[prost(int32, tag="2")]
    pub datetime_type: i32,
    /// 主键id
    #[prost(int64, tag="3")]
    pub notif_id: i64,
    /// 市场id
    #[prost(int64, tag="4")]
    pub market_id: i64,
    /// 时间，时间格式为 2020-12-01 09:30:00|2020-12-01 15:30:00,2020-12-02 09:30:00|2020-12-02 15:30:00
    #[prost(string, tag="5")]
    pub msg_time: ::prost::alloc::string::String,
    /// 1:陆股通 2：QFII
    #[prost(int32, tag="6")]
    pub msg_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationOrders {
    /// 订单ID
    #[prost(int64, tag="1")]
    pub order_id: i64,
}
/// 定义枚举类型，
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
