#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractMsg {
    /// 交易所
    #[prost(string, tag="1")]
    pub exchange_no: ::prost::alloc::string::String,
    /// 商品
    #[prost(string, tag="2")]
    pub commodity_no: ::prost::alloc::string::String,
    /// 合约
    #[prost(string, tag="3")]
    pub contract_no: ::prost::alloc::string::String,
    /// 通道
    #[prost(int32, tag="4")]
    pub channel_no: i32,
    /// true:订阅 false:取消订阅
    #[prost(bool, tag="5")]
    pub subscribe: bool,
}
/// 查询品种
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommodityMsg {
    /// 交易所
    #[prost(string, tag="1")]
    pub exchange_no: ::prost::alloc::string::String,
    /// 商品
    #[prost(string, tag="2")]
    pub commodity_no: ::prost::alloc::string::String,
}
/// 返回查询品种信息
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QryContractMsg {
    /// 交易所
    #[prost(string, tag="1")]
    pub exchange_no: ::prost::alloc::string::String,
    /// 商品
    #[prost(string, tag="2")]
    pub commodity_no: ::prost::alloc::string::String,
    /// 合约
    #[prost(string, tag="3")]
    pub contract_no: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod market_data_servers_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MarketDataServersClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MarketDataServersClient<tonic::transport::Channel> {
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
    impl<T> MarketDataServersClient<T>
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
        ) -> MarketDataServersClient<InterceptedService<T, F>>
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
            MarketDataServersClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn subscribe_market_data(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ContractMsg>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::hqmsg::YsHqInfo>>,
            tonic::Status,
        > {
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
                "/MarketDataServers/SubscribeMarketData",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn query_commodity_data(
            &mut self,
            request: impl tonic::IntoRequest<super::CommodityMsg>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::QryContractMsg>>,
            tonic::Status,
        > {
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
                "/MarketDataServers/QueryCommodityData",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod market_data_servers_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with MarketDataServersServer.
    #[async_trait]
    pub trait MarketDataServers: Send + Sync + 'static {
        ///Server streaming response type for the SubscribeMarketData method.
        type SubscribeMarketDataStream: futures_core::Stream<
                Item = Result<super::super::hqmsg::YsHqInfo, tonic::Status>,
            >
            + Send
            + 'static;
        async fn subscribe_market_data(
            &self,
            request: tonic::Request<tonic::Streaming<super::ContractMsg>>,
        ) -> Result<tonic::Response<Self::SubscribeMarketDataStream>, tonic::Status>;
        ///Server streaming response type for the QueryCommodityData method.
        type QueryCommodityDataStream: futures_core::Stream<
                Item = Result<super::QryContractMsg, tonic::Status>,
            >
            + Send
            + 'static;
        async fn query_commodity_data(
            &self,
            request: tonic::Request<super::CommodityMsg>,
        ) -> Result<tonic::Response<Self::QueryCommodityDataStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MarketDataServersServer<T: MarketDataServers> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MarketDataServers> MarketDataServersServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MarketDataServersServer<T>
    where
        T: MarketDataServers,
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
                "/MarketDataServers/SubscribeMarketData" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeMarketDataSvc<T: MarketDataServers>(pub Arc<T>);
                    impl<
                        T: MarketDataServers,
                    > tonic::server::StreamingService<super::ContractMsg>
                    for SubscribeMarketDataSvc<T> {
                        type Response = super::super::hqmsg::YsHqInfo;
                        type ResponseStream = T::SubscribeMarketDataStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::ContractMsg>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_market_data(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeMarketDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/MarketDataServers/QueryCommodityData" => {
                    #[allow(non_camel_case_types)]
                    struct QueryCommodityDataSvc<T: MarketDataServers>(pub Arc<T>);
                    impl<
                        T: MarketDataServers,
                    > tonic::server::ServerStreamingService<super::CommodityMsg>
                    for QueryCommodityDataSvc<T> {
                        type Response = super::QryContractMsg;
                        type ResponseStream = T::QueryCommodityDataStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CommodityMsg>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_commodity_data(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryCommodityDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T: MarketDataServers> Clone for MarketDataServersServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: MarketDataServers> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MarketDataServers> tonic::server::NamedService
    for MarketDataServersServer<T> {
        const NAME: &'static str = "marketdata.MarketDataServers";
    }
}
