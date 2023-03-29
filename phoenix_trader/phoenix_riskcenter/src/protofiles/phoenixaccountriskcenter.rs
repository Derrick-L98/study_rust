#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixStockPositionRequest {
    /// stock id
    #[prost(int64, tag="1")]
    pub stock_id: i64,
    /// channel id 需要支持0查出所有
    #[prost(int64, tag="2")]
    pub channel_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixStockPositionResponse {
    /// 返回结果
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    /// 返回结果
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    /// position data
    #[prost(message, repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<PhoenixStockPositions>,
    /// 该券的所有用户持仓
    #[prost(int64, tag="4")]
    pub user_total_positions: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixStockPositions {
    /// stock id
    #[prost(int64, tag="1")]
    pub stock_id: i64,
    /// channel id
    #[prost(int64, tag="2")]
    pub channel_id: i64,
    /// current amount
    #[prost(int64, tag="3")]
    pub current_amount: i64,
    /// prebuy amount
    #[prost(int64, tag="4")]
    pub prebuy_amount: i64,
    #[prost(int64, tag="5")]
    pub frozen_amount: i64,
    #[prost(int64, tag="6")]
    pub frozen_amount_temp: i64,
    /// total value
    #[prost(double, tag="7")]
    pub total_value: f64,
    /// total value hkd
    #[prost(double, tag="8")]
    pub total_value_hkd: f64,
    /// stock type
    #[prost(int32, tag="9")]
    pub stock_type: i32,
    /// 是否QFII通道
    #[prost(int32, tag="10")]
    pub is_qfii: i32,
}
/// //分帐户资产信息请求
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixAccountQueryRequest {
    /// 0表示全部
    #[prost(int64, tag="1")]
    pub account_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixAccountResetRequest {
    #[prost(int64, tag="1")]
    pub account_id: i64,
    #[prost(int64, tag="2")]
    pub operator_id: i64,
}
/// 资金划转的请求
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixTransferRequest {
    #[prost(int64, tag="1")]
    pub target_account: i64,
    /// 调整类型，既增加:2,减少:1
    #[prost(int32, tag="2")]
    pub transfer_type: i32,
    /// 金额
    #[prost(double, tag="3")]
    pub transfer_value: f64,
    /// 1: 资金类型
    #[prost(int32, tag="4")]
    pub capital_type: i32,
    /// 操作人员
    #[prost(int64, tag="5")]
    pub operator_id: i64,
    /// 对应的源账户
    #[prost(int64, tag="6")]
    pub source_account: i64,
    /// 说明
    #[prost(string, tag="7")]
    pub memo: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixAssetsResponse {
    /// 返回结果
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    /// 返回结果
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<PhoenixAccountAssetsInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoenixAccountAssetsInfo {
    /// i64,
    #[prost(int64, tag="1")]
    pub id: i64,
    /// i64,                       
    #[prost(int64, tag="2")]
    pub p_account_id: i64,
    /// 本金:初始值+资金划拨得值+实际盈亏;   总账户:融资本金'
    #[prost(double, tag="3")]
    pub p_current_principal: f64,
    /// 信用金  所有账户：(本金*刚杠),仅对总账户有效，主账户和分帐户可以不处理
    #[prost(double, tag="4")]
    pub p_credit_cash: f64,
    /// 当前本金，总账户:当前本金,   分帐户:0,
    #[prost(double, tag="5")]
    pub p_current_financial: f64,
    /// 已借金额 分帐户：持仓表中所有品种的 ∑(total_value*(1-（保证金比率（1/4)))) * 参考汇率，总账户：持仓市值-当前本金，如数值小于等于0，可显示为空
    #[prost(double, tag="6")]
    pub p_financing_borrowed: f64,
    /// 持仓市值，包含科创版和创业板, 全部帐户： 分帐户持仓表中∑(current_amount*avg_price)*参考汇率
    #[prost(double, tag="7")]
    pub p_position_value: f64,
    /// (科创版+创业板）持仓市值，单独计算，全部账户计算方式一样
    #[prost(double, tag="8")]
    pub p_position_value_star: f64,
    /// 创业板开仓挂单金额汇总
    #[prost(double, tag="9")]
    pub p_prebuy_capital_star: f64,
    /// 浮动盈亏，∑((最新价-持仓均价)*数量)
    #[prost(double, tag="10")]
    pub p_floating_profit: f64,
    /// 创业板比率 = [（所属通道持仓市值（创）汇总 + 所属通道对应创业板开仓挂单金额汇总）/ 当前本金 *（杠杆 + 1）] * 100%
    #[prost(double, tag="11")]
    pub p_star_rate: f64,
    /// //风险率  \[已借金额/（持仓市值*0.75）\]*100%，
    #[prost(double, tag="12")]
    pub p_risk_value: f64,
    /// 保证金占用 ∑(品种1对应通道的持仓市值*\[品种1在该通道的保证金比例]+...品种N对应通道持仓市值*[品种N在该通道的保证金比例\])
    #[prost(double, tag="13")]
    pub p_financing_occupied: f64,
    /// 富余保证金 运营子账户运营本金+min(0,浮动盈亏）- 保证金占用
    #[prost(double, tag="14")]
    pub p_rich_security: f64,
    /// 账号类型，比如：0：分帐户, 1：主账户，2:总账户，
    #[prost(int32, tag="15")]
    pub p_account_type: i32,
    /// 上一个交易日
    #[prost(int32, tag="16")]
    pub p_lastdate: i32,
    /// 更新时间，时间戳
    #[prost(int64, tag="17")]
    pub p_updatetime: i64,
}
/// **********************************************账户风控**********************************************
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarginRatioReq {
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// *保证金比例与通道无关
    #[prost(int64, tag="2")]
    pub stock_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarginRatioResp {
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub unit_id: i64,
    #[prost(int64, tag="4")]
    pub stock_id: i64,
    /// 保证金比例
    #[prost(double, tag="5")]
    pub margin_ratio: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserAssetReq {
    #[prost(int64, tag="1")]
    pub unit_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserAssetResp {
    /// 返回结果
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    /// 返回结果
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub unit_id: i64,
    /// 总资产
    #[prost(double, tag="4")]
    pub total_asset: f64,
    /// 总持仓市值
    #[prost(double, tag="5")]
    pub total_position_value: f64,
    /// 创业板市值
    #[prost(double, tag="6")]
    pub gem_position_value: f64,
    /// 当前保证金占用
    #[prost(double, tag="7")]
    pub real_margin: f64,
    /// 当前本金
    #[prost(double, tag="8")]
    pub real_cash: f64,
    /// 风险率
    #[prost(double, tag="9")]
    pub risk_rate: f64,
    /// 账户交易状态
    #[prost(int32, tag="10")]
    pub trade_state: i32,
    /// 预警线
    #[prost(double, tag="11")]
    pub warning_line: f64,
    #[prost(message, repeated, tag="12")]
    pub positions: ::prost::alloc::vec::Vec<PositionInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPositionReq {
    /// 用户id, 必传
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    /// 0 返回全部
    #[prost(int64, tag="2")]
    pub stock_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPositionResp {
    #[prost(string, tag="1")]
    pub ret_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub positions: ::prost::alloc::vec::Vec<PositionInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionInfo {
    /// 用户
    #[prost(int64, tag="1")]
    pub unit_id: i64,
    #[prost(int64, tag="2")]
    pub stock_id: i64,
    #[prost(string, tag="3")]
    pub stock_code: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub exchange_id: i32,
    /// 总持仓量(含QFII数量)
    #[prost(int64, tag="5")]
    pub amount: i64,
    /// 冻结（含临时冻结）
    #[prost(int64, tag="6")]
    pub frozen_amount: i64,
    /// 预买数量
    #[prost(int64, tag="7")]
    pub prebuy_amount: i64,
    /// QFII持仓量  是
    #[prost(int64, tag="8")]
    pub qfii_amount: i64,
    /// 保证金比例
    #[prost(double, tag="9")]
    pub margin_ratio: f64,
    /// 开仓总成本
    #[prost(double, tag="10")]
    pub total_value_hkd: f64,
    /// 最新价
    #[prost(double, tag="11")]
    pub last_price: f64,
    /// 股票类别
    #[prost(int32, tag="12")]
    pub stock_type: i32,
}
/// Generated client implementations.
pub mod account_risk_center_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AccountRiskCenterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AccountRiskCenterClient<tonic::transport::Channel> {
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
    impl<T> AccountRiskCenterClient<T>
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
        ) -> AccountRiskCenterClient<InterceptedService<T, F>>
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
            AccountRiskCenterClient::new(InterceptedService::new(inner, interceptor))
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
        /// 查找股票在某个通道的持仓信息.
        pub async fn query_stock_positions(
            &mut self,
            request: impl tonic::IntoRequest<super::PhoenixStockPositionRequest>,
        ) -> Result<
            tonic::Response<super::PhoenixStockPositionResponse>,
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
                "/phoenixaccountriskcenter.AccountRiskCenter/query_stock_positions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///查找运营账号的资产信息
        pub async fn query_account_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::PhoenixAccountQueryRequest>,
        ) -> Result<tonic::Response<super::PhoenixAssetsResponse>, tonic::Status> {
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
                "/phoenixaccountriskcenter.AccountRiskCenter/query_account_assets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///资金划转
        pub async fn transfer_fund(
            &mut self,
            request: impl tonic::IntoRequest<super::PhoenixTransferRequest>,
        ) -> Result<tonic::Response<super::PhoenixAssetsResponse>, tonic::Status> {
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
                "/phoenixaccountriskcenter.AccountRiskCenter/transfer_fund",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///分帐户reset动作
        pub async fn reset_profit(
            &mut self,
            request: impl tonic::IntoRequest<super::PhoenixAccountResetRequest>,
        ) -> Result<tonic::Response<super::PhoenixAssetsResponse>, tonic::Status> {
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
                "/phoenixaccountriskcenter.AccountRiskCenter/reset_profit",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///**********************************************账户风控**********************************************
        pub async fn query_margin_ratio(
            &mut self,
            request: impl tonic::IntoRequest<super::MarginRatioReq>,
        ) -> Result<tonic::Response<super::MarginRatioResp>, tonic::Status> {
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
                "/phoenixaccountriskcenter.AccountRiskCenter/query_margin_ratio",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_user_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::UserAssetReq>,
        ) -> Result<tonic::Response<super::UserAssetResp>, tonic::Status> {
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
                "/phoenixaccountriskcenter.AccountRiskCenter/query_user_asset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod account_risk_center_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with AccountRiskCenterServer.
    #[async_trait]
    pub trait AccountRiskCenter: Send + Sync + 'static {
        /// 查找股票在某个通道的持仓信息.
        async fn query_stock_positions(
            &self,
            request: tonic::Request<super::PhoenixStockPositionRequest>,
        ) -> Result<tonic::Response<super::PhoenixStockPositionResponse>, tonic::Status>;
        ///查找运营账号的资产信息
        async fn query_account_assets(
            &self,
            request: tonic::Request<super::PhoenixAccountQueryRequest>,
        ) -> Result<tonic::Response<super::PhoenixAssetsResponse>, tonic::Status>;
        ///资金划转
        async fn transfer_fund(
            &self,
            request: tonic::Request<super::PhoenixTransferRequest>,
        ) -> Result<tonic::Response<super::PhoenixAssetsResponse>, tonic::Status>;
        ///分帐户reset动作
        async fn reset_profit(
            &self,
            request: tonic::Request<super::PhoenixAccountResetRequest>,
        ) -> Result<tonic::Response<super::PhoenixAssetsResponse>, tonic::Status>;
        ///**********************************************账户风控**********************************************
        async fn query_margin_ratio(
            &self,
            request: tonic::Request<super::MarginRatioReq>,
        ) -> Result<tonic::Response<super::MarginRatioResp>, tonic::Status>;
        async fn query_user_asset(
            &self,
            request: tonic::Request<super::UserAssetReq>,
        ) -> Result<tonic::Response<super::UserAssetResp>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct AccountRiskCenterServer<T: AccountRiskCenter> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AccountRiskCenter> AccountRiskCenterServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AccountRiskCenterServer<T>
    where
        T: AccountRiskCenter,
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
                "/phoenixaccountriskcenter.AccountRiskCenter/query_stock_positions" => {
                    #[allow(non_camel_case_types)]
                    struct query_stock_positionsSvc<T: AccountRiskCenter>(pub Arc<T>);
                    impl<
                        T: AccountRiskCenter,
                    > tonic::server::UnaryService<super::PhoenixStockPositionRequest>
                    for query_stock_positionsSvc<T> {
                        type Response = super::PhoenixStockPositionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PhoenixStockPositionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_stock_positions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_stock_positionsSvc(inner);
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
                "/phoenixaccountriskcenter.AccountRiskCenter/query_account_assets" => {
                    #[allow(non_camel_case_types)]
                    struct query_account_assetsSvc<T: AccountRiskCenter>(pub Arc<T>);
                    impl<
                        T: AccountRiskCenter,
                    > tonic::server::UnaryService<super::PhoenixAccountQueryRequest>
                    for query_account_assetsSvc<T> {
                        type Response = super::PhoenixAssetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PhoenixAccountQueryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_account_assets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_account_assetsSvc(inner);
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
                "/phoenixaccountriskcenter.AccountRiskCenter/transfer_fund" => {
                    #[allow(non_camel_case_types)]
                    struct transfer_fundSvc<T: AccountRiskCenter>(pub Arc<T>);
                    impl<
                        T: AccountRiskCenter,
                    > tonic::server::UnaryService<super::PhoenixTransferRequest>
                    for transfer_fundSvc<T> {
                        type Response = super::PhoenixAssetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PhoenixTransferRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).transfer_fund(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = transfer_fundSvc(inner);
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
                "/phoenixaccountriskcenter.AccountRiskCenter/reset_profit" => {
                    #[allow(non_camel_case_types)]
                    struct reset_profitSvc<T: AccountRiskCenter>(pub Arc<T>);
                    impl<
                        T: AccountRiskCenter,
                    > tonic::server::UnaryService<super::PhoenixAccountResetRequest>
                    for reset_profitSvc<T> {
                        type Response = super::PhoenixAssetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PhoenixAccountResetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).reset_profit(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = reset_profitSvc(inner);
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
                "/phoenixaccountriskcenter.AccountRiskCenter/query_margin_ratio" => {
                    #[allow(non_camel_case_types)]
                    struct query_margin_ratioSvc<T: AccountRiskCenter>(pub Arc<T>);
                    impl<
                        T: AccountRiskCenter,
                    > tonic::server::UnaryService<super::MarginRatioReq>
                    for query_margin_ratioSvc<T> {
                        type Response = super::MarginRatioResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarginRatioReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_margin_ratio(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_margin_ratioSvc(inner);
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
                "/phoenixaccountriskcenter.AccountRiskCenter/query_user_asset" => {
                    #[allow(non_camel_case_types)]
                    struct query_user_assetSvc<T: AccountRiskCenter>(pub Arc<T>);
                    impl<
                        T: AccountRiskCenter,
                    > tonic::server::UnaryService<super::UserAssetReq>
                    for query_user_assetSvc<T> {
                        type Response = super::UserAssetResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserAssetReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_user_asset(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = query_user_assetSvc(inner);
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
    impl<T: AccountRiskCenter> Clone for AccountRiskCenterServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: AccountRiskCenter> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AccountRiskCenter> tonic::server::NamedService
    for AccountRiskCenterServer<T> {
        const NAME: &'static str = "phoenixaccountriskcenter.AccountRiskCenter";
    }
}
