#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixRiskCheckRequest {
    #[prost(message, optional, tag="1")]
    pub queryinfo: ::core::option::Option<PhoenixRiskCheckInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixRiskCheckInfo {
    /// 用户unit_id, 
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 股票ID, 
    #[prost(int64, tag="2")]
    pub stock_id: i64,
    /// 价格, 
    #[prost(double, tag="3")]
    pub order_price: f64,
    /// 数量，
    #[prost(int32, tag="4")]
    pub order_amount: i32,
    /// 方向（ 1:买 2：卖）
    #[prost(int32, tag="5")]
    pub order_direction: i32,
    /// 通道类型：1：外盘 2：内盘
    #[prost(int32, tag="6")]
    pub channel_type: i32,
    /// 委托类型 1:app下单  2:跟单  3:风控止盈止损平仓单,4:风控总资产预警平仓单 5:pc客户端单 6:结算平仓单 7:管理端强平仓单,8:app清仓,9:pc清仓,10,管理员平仓,11,合约到期日强平
    #[prost(int32, tag="7")]
    pub order_type: i32,
    /// 通道
    #[prost(int64, tag="8")]
    pub order_channel: i64,
    /// 市场ID
    #[prost(int64, tag="9")]
    pub market_id: i64,
    /// 1:USER(用户直连) 2:AGENT(代理托管)
    #[prost(int32, tag="10")]
    pub trade_mode: i32,
    /// 代理账户
    #[prost(int64, tag="11")]
    pub agent_account: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixRiskCheckResponse {
    /// 返回结果
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    /// 返回结果
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub retinfo: ::prost::alloc::vec::Vec<PhoenixRiskCheckInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixRiskRequest {
    #[prost(message, optional, tag="1")]
    pub queryinfo: ::core::option::Option<PhoenixRiskCheckInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixRiskResponse {
    /// 返回结果
    #[prost(int32, tag="1")]
    pub ret_code: i32,
    /// 返回信息
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod phoenix_riskcenter_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PhoenixRiskcenterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PhoenixRiskcenterClient<tonic::transport::Channel> {
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
    impl<T> PhoenixRiskcenterClient<T>
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
        ) -> PhoenixRiskcenterClient<InterceptedService<T, F>>
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
            PhoenixRiskcenterClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn phoenix_risk_check(
            &mut self,
            request: impl tonic::IntoRequest<super::PhoenixRiskCheckRequest>,
        ) -> Result<tonic::Response<super::PhoenixRiskCheckResponse>, tonic::Status> {
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
                "/phoenixriskcenter.PhoenixRiskcenter/phoenix_risk_check",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn phoenix_risk_test(
            &mut self,
            request: impl tonic::IntoRequest<super::PhoenixRiskRequest>,
        ) -> Result<tonic::Response<super::PhoenixRiskResponse>, tonic::Status> {
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
                "/phoenixriskcenter.PhoenixRiskcenter/phoenix_risk_test",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod phoenix_riskcenter_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with PhoenixRiskcenterServer.
    #[async_trait]
    pub trait PhoenixRiskcenter: Send + Sync + 'static {
        async fn phoenix_risk_check(
            &self,
            request: tonic::Request<super::PhoenixRiskCheckRequest>,
        ) -> Result<tonic::Response<super::PhoenixRiskCheckResponse>, tonic::Status>;
        async fn phoenix_risk_test(
            &self,
            request: tonic::Request<super::PhoenixRiskRequest>,
        ) -> Result<tonic::Response<super::PhoenixRiskResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PhoenixRiskcenterServer<T: PhoenixRiskcenter> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PhoenixRiskcenter> PhoenixRiskcenterServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PhoenixRiskcenterServer<T>
    where
        T: PhoenixRiskcenter,
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
                "/phoenixriskcenter.PhoenixRiskcenter/phoenix_risk_check" => {
                    #[allow(non_camel_case_types)]
                    struct phoenix_risk_checkSvc<T: PhoenixRiskcenter>(pub Arc<T>);
                    impl<
                        T: PhoenixRiskcenter,
                    > tonic::server::UnaryService<super::PhoenixRiskCheckRequest>
                    for phoenix_risk_checkSvc<T> {
                        type Response = super::PhoenixRiskCheckResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PhoenixRiskCheckRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).phoenix_risk_check(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = phoenix_risk_checkSvc(inner);
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
                "/phoenixriskcenter.PhoenixRiskcenter/phoenix_risk_test" => {
                    #[allow(non_camel_case_types)]
                    struct phoenix_risk_testSvc<T: PhoenixRiskcenter>(pub Arc<T>);
                    impl<
                        T: PhoenixRiskcenter,
                    > tonic::server::UnaryService<super::PhoenixRiskRequest>
                    for phoenix_risk_testSvc<T> {
                        type Response = super::PhoenixRiskResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PhoenixRiskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).phoenix_risk_test(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = phoenix_risk_testSvc(inner);
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
    impl<T: PhoenixRiskcenter> Clone for PhoenixRiskcenterServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PhoenixRiskcenter> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PhoenixRiskcenter> tonic::server::NamedService
    for PhoenixRiskcenterServer<T> {
        const NAME: &'static str = "phoenixriskcenter.PhoenixRiskcenter";
    }
}
