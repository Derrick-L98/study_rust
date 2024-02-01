/// EXCHANGE:  oms_msg
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OmsMessage {
    /// 用户id
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 0:全部   1:期货  2:证券  3:衍生品
    #[prost(int32, tag="2")]
    pub busin_type: i32,
    /// 消息类型  1: 订单状态变更  2. 资产调整  3.出金/入金(管理端,不处理) ... 98.结算通知 99.公告
    #[prost(int32, tag="3")]
    pub msg_type: i32,
    /// 委托号
    #[prost(int64, tag="4")]
    pub order_no: i64,
    /// 委托类型
    #[prost(int32, tag="5")]
    pub order_type: i32,
    /// 4：已报  5：废单  6：部成  7：已成  8：部撤  9：已撤  a：待撤
    #[prost(string, tag="6")]
    pub order_status: ::prost::alloc::string::String,
    /// 成交编号
    #[prost(int64, tag="7")]
    pub deal_no: i64,
    /// 备注
    #[prost(string, tag="8")]
    pub memo: ::prost::alloc::string::String,
    /// 本次成交数量
    #[prost(int32, tag="9")]
    pub order_quantity: i32,
}
