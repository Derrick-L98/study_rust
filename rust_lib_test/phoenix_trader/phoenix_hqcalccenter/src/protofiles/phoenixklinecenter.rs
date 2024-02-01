/// 备注:历史K线请求必传：合约编号,结束时间, 条数。当前K线请求：传条数 ,合约 
/// 历史分时请求：只传合约编号拼接日期 合约编号值为：合约|日期。 当前分时请求:合约+日期   例如:CL1810|20180910 
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KLineHqRequest {
    /// 合约编号
    #[prost(string, tag="1")]
    pub contract_no: ::prost::alloc::string::String,
    /// 1:一分钟 5:5分钟, 10:10分钟,30: 30分钟, 60：60分钟,24:日线
    #[prost(string, tag="2")]
    pub kline_type: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FenShiResp {
    #[prost(string, tag="1")]
    pub fenshi_hq: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KLineDataResp {
    /// 股票代码
    #[prost(string, tag="1")]
    pub stock_code: ::prost::alloc::string::String,
    /// 最新一条tick的时间, 来一条tick就更新此值, 和prev_minutes对应
    #[prost(string, tag="2")]
    pub tick_time: ::prost::alloc::string::String,
    /// 开
    #[prost(double, tag="3")]
    pub open_price: f64,
    /// 收
    #[prost(double, tag="4")]
    pub close_price: f64,
    /// 高
    #[prost(double, tag="5")]
    pub high_price: f64,
    /// 低
    #[prost(double, tag="6")]
    pub low_price: f64,
    /// 新
    #[prost(double, tag="7")]
    pub last_price: f64,
    /// 均价
    #[prost(double, tag="8")]
    pub average_price: f64,
    /// 昨收价
    #[prost(double, tag="9")]
    pub pre_close_price: f64,
    /// 本周期最新总成交量
    #[prost(double, tag="10")]
    pub current_period_volume: f64,
    /// 本周期最新总成交额
    #[prost(double, tag="11")]
    pub current_period_turnover: f64,
    /// 周期
    #[prost(int32, tag="12")]
    pub period: i32,
    /// 记录K线上一个周期在当天分钟数, 即最新的已形成的周期=====> 用来更新K线
    #[prost(int32, tag="13")]
    pub prev_minutes: i32,
}
/// Generated client implementations.
pub mod phoenix_kline_center_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PhoenixKlineCenterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PhoenixKlineCenterClient<tonic::transport::Channel> {
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
    impl<T> PhoenixKlineCenterClient<T>
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
        ) -> PhoenixKlineCenterClient<InterceptedService<T, F>>
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
            PhoenixKlineCenterClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_last_kline_data(
            &mut self,
            request: impl tonic::IntoRequest<super::KLineHqRequest>,
        ) -> Result<tonic::Response<super::KLineDataResp>, tonic::Status> {
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
                "/phoenixklinecenter.PhoenixKlineCenter/get_last_kline_data",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_generate_fenshi_hq(
            &mut self,
            request: impl tonic::IntoRequest<super::KLineHqRequest>,
        ) -> Result<tonic::Response<super::FenShiResp>, tonic::Status> {
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
                "/phoenixklinecenter.PhoenixKlineCenter/get_generate_fenshi_hq",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod phoenix_kline_center_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with PhoenixKlineCenterServer.
    #[async_trait]
    pub trait PhoenixKlineCenter: Send + Sync + 'static {
        async fn get_last_kline_data(
            &self,
            request: tonic::Request<super::KLineHqRequest>,
        ) -> Result<tonic::Response<super::KLineDataResp>, tonic::Status>;
        async fn get_generate_fenshi_hq(
            &self,
            request: tonic::Request<super::KLineHqRequest>,
        ) -> Result<tonic::Response<super::FenShiResp>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PhoenixKlineCenterServer<T: PhoenixKlineCenter> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PhoenixKlineCenter> PhoenixKlineCenterServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PhoenixKlineCenterServer<T>
    where
        T: PhoenixKlineCenter,
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
                "/phoenixklinecenter.PhoenixKlineCenter/get_last_kline_data" => {
                    #[allow(non_camel_case_types)]
                    struct get_last_kline_dataSvc<T: PhoenixKlineCenter>(pub Arc<T>);
                    impl<
                        T: PhoenixKlineCenter,
                    > tonic::server::UnaryService<super::KLineHqRequest>
                    for get_last_kline_dataSvc<T> {
                        type Response = super::KLineDataResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KLineHqRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_last_kline_data(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = get_last_kline_dataSvc(inner);
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
                "/phoenixklinecenter.PhoenixKlineCenter/get_generate_fenshi_hq" => {
                    #[allow(non_camel_case_types)]
                    struct get_generate_fenshi_hqSvc<T: PhoenixKlineCenter>(pub Arc<T>);
                    impl<
                        T: PhoenixKlineCenter,
                    > tonic::server::UnaryService<super::KLineHqRequest>
                    for get_generate_fenshi_hqSvc<T> {
                        type Response = super::FenShiResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KLineHqRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_generate_fenshi_hq(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = get_generate_fenshi_hqSvc(inner);
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
    impl<T: PhoenixKlineCenter> Clone for PhoenixKlineCenterServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PhoenixKlineCenter> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PhoenixKlineCenter> tonic::server::NamedService
    for PhoenixKlineCenterServer<T> {
        const NAME: &'static str = "phoenixklinecenter.PhoenixKlineCenter";
    }
}
