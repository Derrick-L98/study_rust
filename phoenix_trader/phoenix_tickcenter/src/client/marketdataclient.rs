use crate::protofiles::{hqmsg::YsHqInfo, market_data_servers_client::MarketDataServersClient, ContractMsg};
use anyhow::Result;
use tokio::sync::broadcast::Sender;
use tokio_stream::StreamExt;
use tonic::transport::Channel;
use common::logclient::LogClient;

// #[derive(Debug)]
pub struct MarketDataclient {
    exchangeno: Vec<String>,
    pub client: MarketDataServersClient<Channel>,
    // pub client_hk: MarketDataServersClient<Channel>,
    tx_tick: Sender<YsHqInfo>,
    uri: String,
}

impl MarketDataclient {
    pub async fn new(uri: &String, exchangenos: &String, tx_tick: Sender<YsHqInfo>) -> Self {
        log::info!("连接MarketDataServers服务: {}", &uri);
        let ret = MarketDataServersClient::connect(uri.to_owned()).await;//.expect("行情服务连接失败");
        if ret.as_ref().is_err() {
            LogClient::get().push_error(format!("MarketDataServers连接失败: {:?}", uri).as_str()).await;
        }
        let client = ret.expect("MarketDataServers服务连接失败");
        let exchangeno: Vec<String> = exchangenos.split(',').map(|x| x.to_string()).collect();
        log::info!("行情服务器连接成功...");
        MarketDataclient { client, exchangeno, tx_tick, uri: uri.to_string() }
    }

    // exchange_id: "XSHE", commodity_no: "", contract_no1: "000667_XSHE", currency_no: "", tapidtstamp: "2022-05-11 09:35:57.000",
    // q_pre_settle_price: 1.61, q_pre_position_qty: 0, q_opening_price: 1.62, q_last_price: 1.61, q_high_price: 1.63, q_low_price: 1.61,
    //q_limit_up_price: 1.77, q_limit_down_price: 1.45, q_total_qty: 58661, q_total_turnover: 9512184.0, q_position_qty: 0, q_average_price:
    //1.622, q_closing_price: 0.0, q_last_qty: 78.0, q_bid_price: [1.61, 1.6, 1.59, 1.58, 1.57], q_bid_qty: [15169, 20942, 22925, 30405, 14243],
    // q_ask_price: [1.62, 1.63, 1.64, 1.65, 1.66], q_ask_qty: [21165, 14400, 18194, 29361, 14025], q_change_rate: 0.0, q_change_value: 0.0,
    //q_pre_closing_price: 1.61, q_total_bid_qty: 0, q_total_ask_qty: 0, q_turnover_ratio: 0.24, q_amplitude: 1.24, q_pe_rate: -7.305,
    //q_dyn_pb_rate: 0.91, q_vol_ratio: 2.656, q_circulation_amount: 2439311281, q_total_shares: 2466988633, q_market_value: 3971851699.0,
    //q_money_type: "CNY", q_industry_info: "", q_last_turnover: 12629.0, q_entrust_rate: 3.26, q_bid_qty2: [], q_ask_qty2: [], q_total_qty2: 0.0 }

    pub async fn do_subscribe_market_data(&mut self) -> Result<()> {
        let data = tokio_stream::iter(1..usize::MAX).map(move |_i| ContractMsg {
            exchange_no: "ALL".to_owned(),
            commodity_no: "ALL".to_string(),
            contract_no: "ALL".to_string(),
            channel_no: 0,
            subscribe: true,
        });
        let in_stream = data.take(10); //创建一个新流，该流最多包含n个基础流项。

        let sub_ret = self.client.subscribe_market_data(in_stream).await;
        if sub_ret.as_ref().is_err() {
            log::error!("subscribe error:{:?}", sub_ret.as_ref().err());
            return Err(anyhow!("subscribe error"));
        }
        let response = sub_ret.unwrap();
        // if let Ok(response) = self.client.subscribe_market_data(in_stream).await {
        let mut inbound = response.into_inner();
        while let Ok(inbound_data) = inbound.message().await {
            if inbound_data.is_some() {
                let hqinfo = inbound_data.unwrap();
                // if hqinfo.contract_no1 == "600000_XSHG" {
                    if self.exchangeno.iter().find(|&x| x == &hqinfo.exchange_id).is_some() {
                        // log::info!("code: {}, time: {}", &hqinfo.contract_no1, &hqinfo.tapidtstamp);
                        if let Err(e) = self.tx_tick.send(hqinfo.to_owned()) {
                            log::error!("send to channel error...........:{:?}", &e);
                        }
                    }
                // }
            } else {
                log::error!("inbound data empty");
                break;
            }
        }

        Ok(())
    }

    pub async fn retry_do_subscribe_market_data(&mut self) -> Result<()>{
        let client = MarketDataServersClient::connect(self.uri.to_owned()).await;//.expect("行情服务连接失败");
        if client.as_ref().is_err() {
            log::error!("{:?}", client);
            return Err(anyhow!("{:?}", client));
        }
        self.client = client.unwrap();
        let get_ret = self.do_subscribe_market_data().await;
        if get_ret.as_ref().is_err() {
            log::error!("can't subscribe market data......{:?}", get_ret.as_ref().err().unwrap());
            return Err(anyhow!("{:?}", get_ret));
        }
        Ok(())
    }
    // async fn get_client(&self) -> MarketDataServersClient<Channel> {
    //     self.client.clone()
    // }

    // pub fn convert_market_data_to_quotationinfo(mkdata: &MarketData) -> QuotationInfo {
    //     QuotationInfo { ..Default::default() }
    // }
}
