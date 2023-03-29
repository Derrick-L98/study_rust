#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixassetscenterRequest {
    /// 消息id,(短时间内禁止重复申请)
    #[prost(int64, tag="1")]
    pub message_id: i64,
    /// 操作人信息：操作员id或者用户id
    #[prost(int64, tag="2")]
    pub operator_id: i64,
    /// 资金业务大类，参考资金类型.txt
    #[prost(int32, tag="3")]
    pub business_flag: i32,
    /// 用户id
    #[prost(int64, tag="4")]
    pub unit_id: i64,
    #[prost(message, repeated, tag="5")]
    pub assets: ::prost::alloc::vec::Vec<PhoenixassetscenterRequestInfo>,
    #[prost(message, repeated, tag="6")]
    pub postions: ::prost::alloc::vec::Vec<PhoenixassetspostionrequestInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixassetscenterRequestInfo {
    /// 变化的金额（或者修改后的信用倍数）必传
    #[prost(double, tag="1")]
    pub change_amount: f64,
    /// 操作类型，101：本金增加，102：本金减少，103：本金冻结，104：本金解冻，105：交易金额临时冻结，106：交易金额临时解冻，901：信用倍数调整，902：创建资产用户
    #[prost(int32, tag="2")]
    pub op_type: i32,
    ///    101,102时代表是否出入金的标志，0：否 1：是    ,105,106时代表 是否创业板保证金占用冻结
    #[prost(int32, tag="3")]
    pub flag: i32,
    /// 备注
    #[prost(string, tag="4")]
    pub memo: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixassetspostionrequestInfo {
    /// 201：持仓增加，202：持仓减少，203：其他冻结解冻操作
    #[prost(int32, tag="1")]
    pub op_type: i32,
    /// 变化的数量，此金额必须为正数 或者0， 203时此值为0
    #[prost(int32, tag="2")]
    pub deal_amount: i32,
    /// 品种id
    #[prost(int64, tag="3")]
    pub stock_id: i64,
    /// 多空方向
    #[prost(int32, tag="4")]
    pub position_flag: i32,
    /// 交易币种费用 费用为正数
    #[prost(double, tag="5")]
    pub fee_value: f64,
    /// 当前交易币种成交价格,
    #[prost(double, tag="6")]
    pub deal_price: f64,
    /// 1:qf持仓,0:非qf持仓
    #[prost(int32, tag="7")]
    pub qfii_state: i32,
    /// 保证金比例
    #[prost(double, tag="8")]
    pub margin_rate: f64,
    /// 冻结数量，+为冻结，-为解冻
    #[prost(int32, tag="9")]
    pub frozen_amount: i32,
    /// 临时冻结数量 +为冻结，-为解冻
    #[prost(int32, tag="10")]
    pub temp_frozen_amount: i32,
    /// 预买数量 +为加预买。-为减预买
    #[prost(int32, tag="11")]
    pub prebuy_amount: i32,
    /// 预卖数量 +为加预卖。-为减预卖
    #[prost(int32, tag="12")]
    pub presale_amount: i32,
    /// 成交编号
    #[prost(int64, tag="13")]
    pub deal_no: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixassetscenterResponse {
    /// 返回结果
    #[prost(int32, tag="1")]
    pub ret_code: i32,
    /// 返回信息
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub assetsinfo: ::prost::alloc::vec::Vec<PhoenixassetsResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixassetscenterQueryRequest {
    /// 单次最大50个
    #[prost(int64, repeated, tag="1")]
    pub unit_id: ::prost::alloc::vec::Vec<i64>,
    /// 查询类型 1：资金查询，2：持仓查询，3：资金和持仓查询
    #[prost(int32, tag="2")]
    pub query_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixassetsResult {
    /// 用户id
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 资产
    #[prost(message, optional, tag="2")]
    pub assets: ::core::option::Option<PhoenixassetsResultInfo>,
    /// 持仓
    #[prost(message, repeated, tag="3")]
    pub positionsinfo: ::prost::alloc::vec::Vec<Phoenixassetspostioninfo>,
}
/// 资金查询返回
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixassetsResultInfo {
    /// 用户id
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 当前本金 对应数据库当前本金字段
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
    /// 今日入金
    #[prost(double, tag="9")]
    pub today_deposit: f64,
    /// 今日出金
    #[prost(double, tag="10")]
    pub today_withdraw: f64,
    /// 总入金
    #[prost(double, tag="11")]
    pub total_deposit: f64,
    /// 总出金
    #[prost(double, tag="12")]
    pub total_withdraw: f64,
    /// 昨日资金
    #[prost(double, tag="13")]
    pub last_cash: f64,
    /// 创业板挂单保证金占用
    #[prost(double, tag="14")]
    pub gem_frozen_capital: f64,
}
/// 持仓返回
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Phoenixassetspostioninfo {
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
    /// 最新价
    #[prost(double, tag="23")]
    pub last_price: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixassetspostioninfoResponse {
    /// 返回结果
    #[prost(int32, tag="1")]
    pub ret_code: i32,
    /// 返回信息
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub respinfo: ::prost::alloc::vec::Vec<Phoenixassetspostioninfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionMarginRateReq {
    #[prost(message, repeated, tag="1")]
    pub rates: ::prost::alloc::vec::Vec<PositionMarginRate>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionMarginRate {
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    #[prost(int64, tag="2")]
    pub stock_id: i64,
    #[prost(double, tag="3")]
    pub margin_rate: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionMarginRateResp {
    /// 返回结果
    #[prost(int32, tag="1")]
    pub ret_code: i32,
    /// 返回信息
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionPriceChangeReq {
    #[prost(message, repeated, tag="1")]
    pub list: ::prost::alloc::vec::Vec<PositionPriceChangeItemReq>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionPriceChangeItemReq {
    /// 品种id
    #[prost(int64, tag="1")]
    pub stock_id: i64,
    /// 最新价
    #[prost(double, tag="2")]
    pub last_price: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionPriceChangeResp {
    /// 返回结果
    #[prost(int32, tag="1")]
    pub ret_code: i32,
    /// 返回信息
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod phoenixassetscenter_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PhoenixassetscenterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PhoenixassetscenterClient<tonic::transport::Channel> {
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
    impl<T> PhoenixassetscenterClient<T>
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
        ) -> PhoenixassetscenterClient<InterceptedService<T, F>>
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
            PhoenixassetscenterClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn phoenix_assets_change(
            &mut self,
            request: impl tonic::IntoRequest<super::PhoenixassetscenterRequest>,
        ) -> Result<tonic::Response<super::PhoenixassetscenterResponse>, tonic::Status> {
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
                "/assetscenter.Phoenixassetscenter/phoenix_assets_change",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn phoenix_assets_query(
            &mut self,
            request: impl tonic::IntoRequest<super::PhoenixassetscenterQueryRequest>,
        ) -> Result<tonic::Response<super::PhoenixassetscenterResponse>, tonic::Status> {
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
                "/assetscenter.Phoenixassetscenter/phoenix_assets_query",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn phoenix_positions_marginrate_change(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionMarginRateReq>,
        ) -> Result<tonic::Response<super::PositionMarginRateResp>, tonic::Status> {
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
                "/assetscenter.Phoenixassetscenter/phoenix_positions_marginrate_change",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn phoenix_positions_price_change(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionPriceChangeReq>,
        ) -> Result<tonic::Response<super::PositionPriceChangeResp>, tonic::Status> {
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
                "/assetscenter.Phoenixassetscenter/phoenix_positions_price_change",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod phoenixassetscenter_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with PhoenixassetscenterServer.
    #[async_trait]
    pub trait Phoenixassetscenter: Send + Sync + 'static {
        async fn phoenix_assets_change(
            &self,
            request: tonic::Request<super::PhoenixassetscenterRequest>,
        ) -> Result<tonic::Response<super::PhoenixassetscenterResponse>, tonic::Status>;
        async fn phoenix_assets_query(
            &self,
            request: tonic::Request<super::PhoenixassetscenterQueryRequest>,
        ) -> Result<tonic::Response<super::PhoenixassetscenterResponse>, tonic::Status>;
        async fn phoenix_positions_marginrate_change(
            &self,
            request: tonic::Request<super::PositionMarginRateReq>,
        ) -> Result<tonic::Response<super::PositionMarginRateResp>, tonic::Status>;
        async fn phoenix_positions_price_change(
            &self,
            request: tonic::Request<super::PositionPriceChangeReq>,
        ) -> Result<tonic::Response<super::PositionPriceChangeResp>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PhoenixassetscenterServer<T: Phoenixassetscenter> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Phoenixassetscenter> PhoenixassetscenterServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PhoenixassetscenterServer<T>
    where
        T: Phoenixassetscenter,
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
                "/assetscenter.Phoenixassetscenter/phoenix_assets_change" => {
                    #[allow(non_camel_case_types)]
                    struct phoenix_assets_changeSvc<T: Phoenixassetscenter>(pub Arc<T>);
                    impl<
                        T: Phoenixassetscenter,
                    > tonic::server::UnaryService<super::PhoenixassetscenterRequest>
                    for phoenix_assets_changeSvc<T> {
                        type Response = super::PhoenixassetscenterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PhoenixassetscenterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).phoenix_assets_change(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = phoenix_assets_changeSvc(inner);
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
                "/assetscenter.Phoenixassetscenter/phoenix_assets_query" => {
                    #[allow(non_camel_case_types)]
                    struct phoenix_assets_querySvc<T: Phoenixassetscenter>(pub Arc<T>);
                    impl<
                        T: Phoenixassetscenter,
                    > tonic::server::UnaryService<super::PhoenixassetscenterQueryRequest>
                    for phoenix_assets_querySvc<T> {
                        type Response = super::PhoenixassetscenterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PhoenixassetscenterQueryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).phoenix_assets_query(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = phoenix_assets_querySvc(inner);
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
                "/assetscenter.Phoenixassetscenter/phoenix_positions_marginrate_change" => {
                    #[allow(non_camel_case_types)]
                    struct phoenix_positions_marginrate_changeSvc<
                        T: Phoenixassetscenter,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: Phoenixassetscenter,
                    > tonic::server::UnaryService<super::PositionMarginRateReq>
                    for phoenix_positions_marginrate_changeSvc<T> {
                        type Response = super::PositionMarginRateResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PositionMarginRateReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).phoenix_positions_marginrate_change(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = phoenix_positions_marginrate_changeSvc(inner);
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
                "/assetscenter.Phoenixassetscenter/phoenix_positions_price_change" => {
                    #[allow(non_camel_case_types)]
                    struct phoenix_positions_price_changeSvc<T: Phoenixassetscenter>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Phoenixassetscenter,
                    > tonic::server::UnaryService<super::PositionPriceChangeReq>
                    for phoenix_positions_price_changeSvc<T> {
                        type Response = super::PositionPriceChangeResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PositionPriceChangeReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).phoenix_positions_price_change(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = phoenix_positions_price_changeSvc(inner);
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
    impl<T: Phoenixassetscenter> Clone for PhoenixassetscenterServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Phoenixassetscenter> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Phoenixassetscenter> tonic::server::NamedService
    for PhoenixassetscenterServer<T> {
        const NAME: &'static str = "assetscenter.Phoenixassetscenter";
    }
}
