#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeHqMsgReq {
    /// 动作类型 1：订阅，0：取消订阅。
    #[prost(int32, tag="1")]
    pub action: i32,
    /// 品种代码
    #[prost(string, tag="2")]
    pub goods: ::prost::alloc::string::String,
    /// 市场代码
    #[prost(string, tag="3")]
    pub exchange_no: ::prost::alloc::string::String,
    /// 合约编号
    #[prost(string, tag="4")]
    pub contract: ::prost::alloc::string::String,
    /// Redis中对应ID
    #[prost(string, tag="5")]
    pub id: ::prost::alloc::string::String,
    /// 商品类型，期货为F
    #[prost(string, tag="6")]
    pub commodity_type: ::prost::alloc::string::String,
}
/// 回复的数据包
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultMsg {
    #[prost(string, tag="1")]
    pub err_msg: ::prost::alloc::string::String,
    /// 0 代表成功，其他错误码自行定义
    #[prost(int32, tag="2")]
    pub err_code: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickHqReq {
    /// 合约编号可选
    #[prost(string, tag="1")]
    pub strcontractno: ::prost::alloc::string::String,
    /// 请求行情类型 0:1档行情(默认) 1:5档行情
    #[prost(int32, tag="2")]
    pub iticktype: i32,
    /// 请求tick时间 unix时间
    #[prost(int64, tag="3")]
    pub ticktime: i64,
    /// 0 延迟行情  1实时行情 
    #[prost(int32, tag="4")]
    pub realtime: i32,
}
/// repeated hqmsg.YsHqInfo tickhqinfo = 1;  
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickHqResp {
}
/// 备注:历史K线请求必传：合约编号,结束时间, 条数。当前K线请求：传条数 ,合约 
/// 历史分时请求：只传合约编号拼接日期 合约编号值为：合约|日期。 当前分时请求:合约+日期   例如:CL1810|20180910 
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KLineHqReq {
    /// 合约编号
    #[prost(string, tag="1")]
    pub strcontractno: ::prost::alloc::string::String,
    /// 1:一分钟 5:5分钟, 10:10分钟,30: 30分钟, 60：60分钟,24:日线
    #[prost(string, tag="2")]
    pub strklinetype: ::prost::alloc::string::String,
    /// 结束时间
    #[prost(string, tag="3")]
    pub strendtime: ::prost::alloc::string::String,
    /// 条数
    #[prost(int32, tag="4")]
    pub limit: i32,
    /// 0 延迟行情  1实时行情  
    #[prost(int32, tag="5")]
    pub realtime: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KLineHqResp {
    #[prost(message, repeated, tag="1")]
    pub klineinfo: ::prost::alloc::vec::Vec<KLineHqInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KLineHqInfo {
    #[prost(string, tag="1")]
    pub strkline: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPriceMsgReq {
    /// 证券代码 60001
    #[prost(string, tag="1")]
    pub stock_code: ::prost::alloc::string::String,
    /// 市场 沪深:101 102 港:103 ...
    #[prost(int32, tag="2")]
    pub exchange_id: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPriceMsgResp {
    #[prost(int32, tag="1")]
    pub err_code: i32,
    #[prost(string, tag="2")]
    pub err_msg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<LastPriceInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPriceInfo {
    /// 最新价
    #[prost(double, tag="1")]
    pub last_price: f64,
    /// 涨幅
    #[prost(double, tag="2")]
    pub change_rate: f64,
    /// 涨跌
    #[prost(double, tag="3")]
    pub change_value: f64,
}
/// Generated client implementations.
pub mod svr_post_subscribe_hq_msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SvrPostSubscribeHqMsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SvrPostSubscribeHqMsgClient<tonic::transport::Channel> {
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
    impl<T> SvrPostSubscribeHqMsgClient<T>
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
        ) -> SvrPostSubscribeHqMsgClient<InterceptedService<T, F>>
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
            SvrPostSubscribeHqMsgClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn post_subscribe_hq_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeHqMsgReq>,
        ) -> Result<tonic::Response<super::ResultMsg>, tonic::Status> {
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
                "/hqcenter.SvrPostSubscribeHqMsg/PostSubscribeHqMsg",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn post_history_k_line_hq(
            &mut self,
            request: impl tonic::IntoRequest<super::KLineHqReq>,
        ) -> Result<tonic::Response<super::KLineHqResp>, tonic::Status> {
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
                "/hqcenter.SvrPostSubscribeHqMsg/PostHistoryKLineHq",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn post_current_kline_hq(
            &mut self,
            request: impl tonic::IntoRequest<super::KLineHqReq>,
        ) -> Result<tonic::Response<super::KLineHqResp>, tonic::Status> {
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
                "/hqcenter.SvrPostSubscribeHqMsg/PostCurrentKlineHq",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn post_history_fen_shi_hq(
            &mut self,
            request: impl tonic::IntoRequest<super::KLineHqReq>,
        ) -> Result<tonic::Response<super::KLineHqResp>, tonic::Status> {
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
                "/hqcenter.SvrPostSubscribeHqMsg/PostHistoryFenShiHq",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn post_current_fen_shi_hq(
            &mut self,
            request: impl tonic::IntoRequest<super::KLineHqReq>,
        ) -> Result<tonic::Response<super::KLineHqResp>, tonic::Status> {
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
                "/hqcenter.SvrPostSubscribeHqMsg/PostCurrentFenShiHq",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn post_tick_hq(
            &mut self,
            request: impl tonic::IntoRequest<super::TickHqReq>,
        ) -> Result<tonic::Response<super::TickHqResp>, tonic::Status> {
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
                "/hqcenter.SvrPostSubscribeHqMsg/PostTickHq",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_last_price(
            &mut self,
            request: impl tonic::IntoRequest<super::LastPriceMsgReq>,
        ) -> Result<tonic::Response<super::LastPriceMsgResp>, tonic::Status> {
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
                "/hqcenter.SvrPostSubscribeHqMsg/get_last_price",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod svr_post_subscribe_hq_msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with SvrPostSubscribeHqMsgServer.
    #[async_trait]
    pub trait SvrPostSubscribeHqMsg: Send + Sync + 'static {
        async fn post_subscribe_hq_msg(
            &self,
            request: tonic::Request<super::SubscribeHqMsgReq>,
        ) -> Result<tonic::Response<super::ResultMsg>, tonic::Status>;
        async fn post_history_k_line_hq(
            &self,
            request: tonic::Request<super::KLineHqReq>,
        ) -> Result<tonic::Response<super::KLineHqResp>, tonic::Status>;
        async fn post_current_kline_hq(
            &self,
            request: tonic::Request<super::KLineHqReq>,
        ) -> Result<tonic::Response<super::KLineHqResp>, tonic::Status>;
        async fn post_history_fen_shi_hq(
            &self,
            request: tonic::Request<super::KLineHqReq>,
        ) -> Result<tonic::Response<super::KLineHqResp>, tonic::Status>;
        async fn post_current_fen_shi_hq(
            &self,
            request: tonic::Request<super::KLineHqReq>,
        ) -> Result<tonic::Response<super::KLineHqResp>, tonic::Status>;
        async fn post_tick_hq(
            &self,
            request: tonic::Request<super::TickHqReq>,
        ) -> Result<tonic::Response<super::TickHqResp>, tonic::Status>;
        async fn get_last_price(
            &self,
            request: tonic::Request<super::LastPriceMsgReq>,
        ) -> Result<tonic::Response<super::LastPriceMsgResp>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SvrPostSubscribeHqMsgServer<T: SvrPostSubscribeHqMsg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SvrPostSubscribeHqMsg> SvrPostSubscribeHqMsgServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for SvrPostSubscribeHqMsgServer<T>
    where
        T: SvrPostSubscribeHqMsg,
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
                "/hqcenter.SvrPostSubscribeHqMsg/PostSubscribeHqMsg" => {
                    #[allow(non_camel_case_types)]
                    struct PostSubscribeHqMsgSvc<T: SvrPostSubscribeHqMsg>(pub Arc<T>);
                    impl<
                        T: SvrPostSubscribeHqMsg,
                    > tonic::server::UnaryService<super::SubscribeHqMsgReq>
                    for PostSubscribeHqMsgSvc<T> {
                        type Response = super::ResultMsg;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeHqMsgReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).post_subscribe_hq_msg(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PostSubscribeHqMsgSvc(inner);
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
                "/hqcenter.SvrPostSubscribeHqMsg/PostHistoryKLineHq" => {
                    #[allow(non_camel_case_types)]
                    struct PostHistoryKLineHqSvc<T: SvrPostSubscribeHqMsg>(pub Arc<T>);
                    impl<
                        T: SvrPostSubscribeHqMsg,
                    > tonic::server::UnaryService<super::KLineHqReq>
                    for PostHistoryKLineHqSvc<T> {
                        type Response = super::KLineHqResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KLineHqReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).post_history_k_line_hq(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PostHistoryKLineHqSvc(inner);
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
                "/hqcenter.SvrPostSubscribeHqMsg/PostCurrentKlineHq" => {
                    #[allow(non_camel_case_types)]
                    struct PostCurrentKlineHqSvc<T: SvrPostSubscribeHqMsg>(pub Arc<T>);
                    impl<
                        T: SvrPostSubscribeHqMsg,
                    > tonic::server::UnaryService<super::KLineHqReq>
                    for PostCurrentKlineHqSvc<T> {
                        type Response = super::KLineHqResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KLineHqReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).post_current_kline_hq(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PostCurrentKlineHqSvc(inner);
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
                "/hqcenter.SvrPostSubscribeHqMsg/PostHistoryFenShiHq" => {
                    #[allow(non_camel_case_types)]
                    struct PostHistoryFenShiHqSvc<T: SvrPostSubscribeHqMsg>(pub Arc<T>);
                    impl<
                        T: SvrPostSubscribeHqMsg,
                    > tonic::server::UnaryService<super::KLineHqReq>
                    for PostHistoryFenShiHqSvc<T> {
                        type Response = super::KLineHqResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KLineHqReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).post_history_fen_shi_hq(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PostHistoryFenShiHqSvc(inner);
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
                "/hqcenter.SvrPostSubscribeHqMsg/PostCurrentFenShiHq" => {
                    #[allow(non_camel_case_types)]
                    struct PostCurrentFenShiHqSvc<T: SvrPostSubscribeHqMsg>(pub Arc<T>);
                    impl<
                        T: SvrPostSubscribeHqMsg,
                    > tonic::server::UnaryService<super::KLineHqReq>
                    for PostCurrentFenShiHqSvc<T> {
                        type Response = super::KLineHqResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KLineHqReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).post_current_fen_shi_hq(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PostCurrentFenShiHqSvc(inner);
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
                "/hqcenter.SvrPostSubscribeHqMsg/PostTickHq" => {
                    #[allow(non_camel_case_types)]
                    struct PostTickHqSvc<T: SvrPostSubscribeHqMsg>(pub Arc<T>);
                    impl<
                        T: SvrPostSubscribeHqMsg,
                    > tonic::server::UnaryService<super::TickHqReq>
                    for PostTickHqSvc<T> {
                        type Response = super::TickHqResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TickHqReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).post_tick_hq(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PostTickHqSvc(inner);
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
                "/hqcenter.SvrPostSubscribeHqMsg/get_last_price" => {
                    #[allow(non_camel_case_types)]
                    struct get_last_priceSvc<T: SvrPostSubscribeHqMsg>(pub Arc<T>);
                    impl<
                        T: SvrPostSubscribeHqMsg,
                    > tonic::server::UnaryService<super::LastPriceMsgReq>
                    for get_last_priceSvc<T> {
                        type Response = super::LastPriceMsgResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LastPriceMsgReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_last_price(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = get_last_priceSvc(inner);
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
    impl<T: SvrPostSubscribeHqMsg> Clone for SvrPostSubscribeHqMsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SvrPostSubscribeHqMsg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SvrPostSubscribeHqMsg> tonic::server::NamedService
    for SvrPostSubscribeHqMsgServer<T> {
        const NAME: &'static str = "hqcenter.SvrPostSubscribeHqMsg";
    }
}
