/// ----------------------------------下单请求----------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderReq {
    /// 消息ID
    #[prost(int64, tag="1")]
    pub msg_id: i64,
    /// 用户id
    #[prost(int64, tag="2")]
    pub unit_id: i64,
    /// 证券id
    #[prost(int64, tag="3")]
    pub stock_id: i64,
    /// 委托方向  1=买  2=卖
    #[prost(int32, tag="4")]
    pub order_direction: i32,
    /// 订单数量
    #[prost(int32, tag="5")]
    pub order_qty: i32,
    /// 价格类型(市价限价)
    #[prost(int32, tag="6")]
    pub price_type: i32,
    /// 委托价格
    #[prost(double, tag="7")]
    pub order_price: f64,
    /// 操作员
    #[prost(int64, tag="8")]
    pub operator_no: i64,
    /// 委托类型 1:app下单  2:跟单  3:风控止盈止损平仓单,4:风控总资产预警平仓单 5:pc客户端单 6:结算平仓单 7:管理端强平仓单,8:app清仓,9:pc清仓,10,管理员平仓,11,合约到期日强平
    #[prost(int32, tag="9")]
    pub order_type: i32,
    /// 1:USER(用户直连) 2:AGENT(代理托管)
    #[prost(int32, tag="10")]
    pub trade_mode: i32,
    /// 代理账户
    #[prost(int64, tag="11")]
    pub agent_account: i64,
    /// repeated Riskinfo risk_info = 13;
    #[prost(int64, tag="12")]
    pub order_id: i64,
}
///   下单请求响应
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderResp {
    /// 与请求消息ID对应
    #[prost(int64, tag="1")]
    pub msg_id: i64,
    #[prost(int64, tag="2")]
    pub order_id: i64,
    #[prost(int32, tag="3")]
    pub error_code: i32,
    #[prost(string, tag="4")]
    pub error_msg: ::prost::alloc::string::String,
}
/// TCancelOrder 撤单请求数据类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelReq {
    /// 消息ID
    #[prost(int64, tag="1")]
    pub msg_id: i64,
    /// 用户id
    #[prost(int64, tag="2")]
    pub unit_id: i64,
    /// 订单id(撤单用)
    #[prost(int64, tag="3")]
    pub order_id: i64,
    /// 操作员
    #[prost(int64, tag="4")]
    pub operator_no: i64,
    /// 撤单类型 1:app撤单  2:pc撤单  3:风控撤单  4:管理员撤单
    #[prost(int32, tag="5")]
    pub cancel_type: i32,
    /// 1:USER(用户直连) 2:AGENT(代理托管)
    #[prost(int32, tag="6")]
    pub trade_mode: i32,
    /// 代理账户
    #[prost(int64, tag="7")]
    pub agent_account: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Riskinfo {
    #[prost(int64, tag="1")]
    pub channel_id: i64,
    #[prost(int32, tag="2")]
    pub channel_type: i32,
    #[prost(int32, tag="3")]
    pub order_amount: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplenishOrderReq {
    #[prost(message, optional, tag="1")]
    pub order: ::core::option::Option<OrderReq>,
    #[prost(message, repeated, tag="2")]
    pub riskinfo: ::prost::alloc::vec::Vec<Riskinfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplenishOrderResp {
    #[prost(string, tag="1")]
    pub err_msg: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub err_code: i32,
}
/// Generated client implementations.
pub mod order_center_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OrderCenterServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrderCenterServiceClient<tonic::transport::Channel> {
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
    impl<T> OrderCenterServiceClient<T>
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
        ) -> OrderCenterServiceClient<InterceptedService<T, F>>
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
            OrderCenterServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn place_order(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderReq>,
        ) -> Result<tonic::Response<super::OrderResp>, tonic::Status> {
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
                "/phoenixordercenter.OrderCenterService/PlaceOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn cancel_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelReq>,
        ) -> Result<tonic::Response<super::OrderResp>, tonic::Status> {
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
                "/phoenixordercenter.OrderCenterService/CancelOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn replenishment_order(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplenishOrderReq>,
        ) -> Result<tonic::Response<super::ReplenishOrderResp>, tonic::Status> {
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
                "/phoenixordercenter.OrderCenterService/ReplenishmentOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod order_center_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with OrderCenterServiceServer.
    #[async_trait]
    pub trait OrderCenterService: Send + Sync + 'static {
        async fn place_order(
            &self,
            request: tonic::Request<super::OrderReq>,
        ) -> Result<tonic::Response<super::OrderResp>, tonic::Status>;
        async fn cancel_order(
            &self,
            request: tonic::Request<super::CancelReq>,
        ) -> Result<tonic::Response<super::OrderResp>, tonic::Status>;
        async fn replenishment_order(
            &self,
            request: tonic::Request<super::ReplenishOrderReq>,
        ) -> Result<tonic::Response<super::ReplenishOrderResp>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct OrderCenterServiceServer<T: OrderCenterService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: OrderCenterService> OrderCenterServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OrderCenterServiceServer<T>
    where
        T: OrderCenterService,
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
                "/phoenixordercenter.OrderCenterService/PlaceOrder" => {
                    #[allow(non_camel_case_types)]
                    struct PlaceOrderSvc<T: OrderCenterService>(pub Arc<T>);
                    impl<
                        T: OrderCenterService,
                    > tonic::server::UnaryService<super::OrderReq> for PlaceOrderSvc<T> {
                        type Response = super::OrderResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).place_order(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PlaceOrderSvc(inner);
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
                "/phoenixordercenter.OrderCenterService/CancelOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CancelOrderSvc<T: OrderCenterService>(pub Arc<T>);
                    impl<
                        T: OrderCenterService,
                    > tonic::server::UnaryService<super::CancelReq>
                    for CancelOrderSvc<T> {
                        type Response = super::OrderResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).cancel_order(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelOrderSvc(inner);
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
                "/phoenixordercenter.OrderCenterService/ReplenishmentOrder" => {
                    #[allow(non_camel_case_types)]
                    struct ReplenishmentOrderSvc<T: OrderCenterService>(pub Arc<T>);
                    impl<
                        T: OrderCenterService,
                    > tonic::server::UnaryService<super::ReplenishOrderReq>
                    for ReplenishmentOrderSvc<T> {
                        type Response = super::ReplenishOrderResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReplenishOrderReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).replenishment_order(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReplenishmentOrderSvc(inner);
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
    impl<T: OrderCenterService> Clone for OrderCenterServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: OrderCenterService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: OrderCenterService> tonic::server::NamedService
    for OrderCenterServiceServer<T> {
        const NAME: &'static str = "phoenixordercenter.OrderCenterService";
    }
}
