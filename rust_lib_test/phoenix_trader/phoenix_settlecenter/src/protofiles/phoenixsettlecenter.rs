/// ----------------------------------查询清算明细----------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidateDetailReq {
    /// 市场, 为0时查询全部市场
    #[prost(int32, tag="1")]
    pub market_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidateDetailResp {
    #[prost(message, repeated, tag="1")]
    pub details: ::prost::alloc::vec::Vec<LiquidateDetail>,
    #[prost(int32, tag="2")]
    pub error_code: i32,
    #[prost(string, tag="3")]
    pub error_msg: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidateDetail {
    /// 市场
    #[prost(int32, tag="1")]
    pub market_type: i32,
    /// 交易系统日期
    #[prost(int32, tag="2")]
    pub sys_date: i32,
    /// 一般流程 ->1->2->3->
    #[prost(enumeration="LiqStep", tag="3")]
    pub current_step: i32,
    /// 当前步骤的执行状态
    #[prost(enumeration="LiqState", tag="4")]
    pub state: i32,
    /// 当前步骤的详细说明
    #[prost(string, tag="5")]
    pub liq_memo: ::prost::alloc::string::String,
    /// 执行时间
    #[prost(int64, tag="6")]
    pub exec_time: i64,
}
/// ----------------------------------结算请求----------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidateReq {
    /// 消息ID
    #[prost(int64, tag="1")]
    pub msg_id: i64,
    /// 市场类别: 1 沪深  2 港股  3 美股
    #[prost(int32, tag="2")]
    pub market_type: i32,
    #[prost(enumeration="LiqStep", tag="3")]
    pub liqstep: i32,
}
///   
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidateResp {
    /// 与请求消息ID对应
    #[prost(int64, tag="1")]
    pub msg_id: i64,
    #[prost(int32, tag="2")]
    pub error_code: i32,
    #[prost(string, tag="3")]
    pub error_msg: ::prost::alloc::string::String,
}
/// ----------------------------------分红送股登记----------------------------------
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DividendInfoReq {
    /// 消息ID
    #[prost(int64, tag="1")]
    pub msg_id: i64,
    /// 市场id
    #[prost(int32, tag="2")]
    pub exchange_id: i32,
    /// 证券代码
    #[prost(string, tag="3")]
    pub stock_code: ::prost::alloc::string::String,
    /// 权益类型  431:红利  432:红股 
    #[prost(string, tag="4")]
    pub dividend_flag: ::prost::alloc::string::String,
    /// 除权日
    #[prost(int32, tag="5")]
    pub ex_date: i32,
    /// 发放日
    #[prost(int32, tag="6")]
    pub settle_date: i32,
    /// 除权价格
    #[prost(double, tag="7")]
    pub dividend_price: f64,
    /// 每股分配
    #[prost(double, tag="8")]
    pub stock_rate: f64,
    /// 每股税率
    #[prost(double, tag="9")]
    pub tax_rate: f64,
    /// 最后更新日期
    #[prost(int32, tag="11")]
    pub lastupdate_date: i32,
    /// 登记日
    #[prost(int32, tag="12")]
    pub record_date: i32,
    /// 操作员
    #[prost(int32, tag="13")]
    pub operator_no: i32,
    /// 状态  填1:已经配对
    #[prost(string, tag="14")]
    pub status: ::prost::alloc::string::String,
    /// 币种
    #[prost(string, tag="15")]
    pub currency_no: ::prost::alloc::string::String,
    /// 操作标记 0:新增 1:修改
    #[prost(int32, tag="16")]
    pub operate_flag: i32,
    /// 权益分派记录号
    #[prost(int64, tag="17")]
    pub dividend_no: i64,
    /// 备注
    #[prost(string, tag="18")]
    pub remarks: ::prost::alloc::string::String,
}
/// 回复的数据包
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DividendInfoResp {
    /// 与请求消息ID对应
    #[prost(int64, tag="1")]
    pub msg_id: i64,
    #[prost(int32, tag="2")]
    pub error_code: i32,
    #[prost(string, tag="3")]
    pub error_msg: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub devidend_no: i64,
}
/// 清算步骤
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LiqStep {
    None = 0,
    /// 归档
    Archive = 1,
    /// 初始化
    Initialize = 2,
    /// 交收
    Settle = 3,
    /// 权益分派-登记
    DvdRegister = 4,
    /// 权益分派-到账
    DvdSettle = 5,
}
impl LiqStep {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LiqStep::None => "None",
            LiqStep::Archive => "Archive",
            LiqStep::Initialize => "Initialize",
            LiqStep::Settle => "Settle",
            LiqStep::DvdRegister => "DvdRegister",
            LiqStep::DvdSettle => "DvdSettle",
        }
    }
}
/// 清算状态
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LiqState {
    /// 未执行
    UnExec = 0,
    /// 执行完成
    Done = 1,
    /// 执行出错
    Failed = 2,
}
impl LiqState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LiqState::UnExec => "UnExec",
            LiqState::Done => "Done",
            LiqState::Failed => "Failed",
        }
    }
}
/// Generated client implementations.
pub mod settlecenter_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SettlecenterServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SettlecenterServiceClient<tonic::transport::Channel> {
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
    impl<T> SettlecenterServiceClient<T>
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
        ) -> SettlecenterServiceClient<InterceptedService<T, F>>
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
            SettlecenterServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn query_liquidate_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::LiquidateDetailReq>,
        ) -> Result<tonic::Response<super::LiquidateDetailResp>, tonic::Status> {
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
                "/phoenixsettlecenter.SettlecenterService/QueryLiquidateDetail",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn do_liquidate(
            &mut self,
            request: impl tonic::IntoRequest<super::LiquidateReq>,
        ) -> Result<tonic::Response<super::LiquidateResp>, tonic::Status> {
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
                "/phoenixsettlecenter.SettlecenterService/DoLiquidate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn dividend_info(
            &mut self,
            request: impl tonic::IntoRequest<super::DividendInfoReq>,
        ) -> Result<tonic::Response<super::DividendInfoResp>, tonic::Status> {
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
                "/phoenixsettlecenter.SettlecenterService/DividendInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod settlecenter_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with SettlecenterServiceServer.
    #[async_trait]
    pub trait SettlecenterService: Send + Sync + 'static {
        async fn query_liquidate_detail(
            &self,
            request: tonic::Request<super::LiquidateDetailReq>,
        ) -> Result<tonic::Response<super::LiquidateDetailResp>, tonic::Status>;
        async fn do_liquidate(
            &self,
            request: tonic::Request<super::LiquidateReq>,
        ) -> Result<tonic::Response<super::LiquidateResp>, tonic::Status>;
        async fn dividend_info(
            &self,
            request: tonic::Request<super::DividendInfoReq>,
        ) -> Result<tonic::Response<super::DividendInfoResp>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SettlecenterServiceServer<T: SettlecenterService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SettlecenterService> SettlecenterServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SettlecenterServiceServer<T>
    where
        T: SettlecenterService,
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
                "/phoenixsettlecenter.SettlecenterService/QueryLiquidateDetail" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLiquidateDetailSvc<T: SettlecenterService>(pub Arc<T>);
                    impl<
                        T: SettlecenterService,
                    > tonic::server::UnaryService<super::LiquidateDetailReq>
                    for QueryLiquidateDetailSvc<T> {
                        type Response = super::LiquidateDetailResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LiquidateDetailReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_liquidate_detail(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryLiquidateDetailSvc(inner);
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
                "/phoenixsettlecenter.SettlecenterService/DoLiquidate" => {
                    #[allow(non_camel_case_types)]
                    struct DoLiquidateSvc<T: SettlecenterService>(pub Arc<T>);
                    impl<
                        T: SettlecenterService,
                    > tonic::server::UnaryService<super::LiquidateReq>
                    for DoLiquidateSvc<T> {
                        type Response = super::LiquidateResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LiquidateReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).do_liquidate(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DoLiquidateSvc(inner);
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
                "/phoenixsettlecenter.SettlecenterService/DividendInfo" => {
                    #[allow(non_camel_case_types)]
                    struct DividendInfoSvc<T: SettlecenterService>(pub Arc<T>);
                    impl<
                        T: SettlecenterService,
                    > tonic::server::UnaryService<super::DividendInfoReq>
                    for DividendInfoSvc<T> {
                        type Response = super::DividendInfoResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DividendInfoReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).dividend_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DividendInfoSvc(inner);
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
    impl<T: SettlecenterService> Clone for SettlecenterServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SettlecenterService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SettlecenterService> tonic::server::NamedService
    for SettlecenterServiceServer<T> {
        const NAME: &'static str = "phoenixsettlecenter.SettlecenterService";
    }
}
