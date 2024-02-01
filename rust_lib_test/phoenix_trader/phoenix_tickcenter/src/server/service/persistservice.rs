use anyhow::{anyhow, Result};

// use sled::Db;
use std::fmt::Write;
use utility::timeutil;
use crate::{client::cassandraclient::CassandraClient, protofiles::hqmsg::YsHqInfo};

pub struct PersistService {}

#[derive(scylla::ValueList, Clone)]
pub struct CassandraData {
    pub key: String,
    pub uptime: String,
    pub val: String,
}
#[derive(scylla::ValueList, Clone)]
pub struct CassandraKLineData {
    pub peroid: i32,
    pub vc_code: String,
    pub l_update_time: String,
    pub en_close_price: f64,
    pub en_high_price: f64,
    pub en_last_price: f64,
    pub en_low_price: f64,
    pub en_open_price: f64,
    pub l_volume: f64,
}

impl PersistService {
    // pub async fn persist_file_db(ticks: &Vec<YsHqInfo>, db: &Db) -> Result<()> {
    //     Ok(())
    // }

    // tapidtstamp: "2022-05-30 11:29:19.290"
    pub async fn insert_ticks_into_cassandra(ticks: &Vec<YsHqInfo>, cass_client: &CassandraClient) -> Result<()> {
        let contents: Vec<CassandraData> = ticks
            .iter()
            .map(|x| CassandraData {
                key: x.contract_no1.clone(),
                uptime: format!("{}", timeutil::convert_datetime_to_timestamp(&x.tapidtstamp)),
                val: PersistService::consolidated_data_format(x),
            })
            .collect();

        let cql = "insert into tstockhqtick (vc_contract_code, l_update_time, vc_content) values (?, ?, ?)";
        // ========================= insert one by one ===============================
        // let ret = cass_client.do_prepare(cql).await;
        // if ret.as_ref().is_err() {
        //     return Err(anyhow!("prepare error:{}", ret.as_ref().err().unwrap().to_string()));
        // }
        // let prepared = ret.unwrap();

        // for val in contents {
        //     let ret = cass_client.execute_prepare(&prepared, &val).await;
        //     if ret.as_ref().is_err() {
        //         return Err(anyhow!("insert cassandra error:{}", ret.as_ref().err().unwrap().to_string()));
        //     }
        // }

        //======================= insert batch =======================
        let ret = cass_client.execute_batch(cql, &contents).await;
        if ret.as_ref().is_err() {
            return Err(anyhow!("insert cassandra error:{}", ret.as_ref().err().unwrap().to_string()));
        }

        Ok(())
    }

    pub async fn get_markettype(stock_exchange: &String) -> i32 {
        let mut market_type = 0;
        if stock_exchange == "XSHG" || stock_exchange == "XSHE" || stock_exchange == "HS" {
            market_type = 3;
        } else if stock_exchange == "XHKG" || stock_exchange == "HK" {
            market_type = 1;
        } else if stock_exchange == "XASE" || stock_exchange == "XNYS" || stock_exchange == "XNAS" || stock_exchange == "US" {
            market_type = 2;
        }

        market_type
    }

    //|2022-03-10 14:59:09.000|9.630000|9.874000|9.840000|0.000000|13790|13616200.000000|0.210000|2.180000|-41.530000|
    //0.610000|1.130000|9.780000|0.000000|9.970000|9.770000|

    //121740438|178963188|1760997769.000000|46.852000|2.502000|CNY|
    //bidprice|9.850000|0.000000|0.000000|0.000000|0.000000|
    //bidqty|138|0|0|0|0|
    //askprice|9.850000|0.000000|0.000000|0.000000|0.000000|
    //askqty|138|196|0|0|0|
    pub fn consolidated_data_format(ptick: &YsHqInfo) -> String {
        //|时间|昨收|均价|最新|现手|交易量|交易额|涨跌值|涨跌幅|委比|量比|换手率|开盘|收盘|最高|最低
        let mut data = format!(
            "|{}|{:.6}|{:.6}|{:.6}|{:.6}|{}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}|",
            ptick.tapidtstamp,
            ptick.q_pre_closing_price,
            ptick.q_average_price,
            ptick.q_last_price,
            ptick.q_last_qty,
            ptick.q_total_qty,
            ptick.q_total_turnover,
            ptick.q_change_value,
            ptick.q_change_rate,
            ptick.q_entrust_rate,
            ptick.q_vol_ratio,
            ptick.q_turnover_ratio,
            ptick.q_opening_price,
            ptick.q_closing_price,
            ptick.q_high_price,
            ptick.q_low_price
        );

        // write: 1|2|3|4|5|6|, time: 4.5µs  //可避免的堆分配。
        // +: 1|2|3|4|5|6|, time: 14.97µs

        //|流通股|总股本|总市值|市盈率|市净率|币种
        // let stock_info = format!(
        //     "{}|{}|{:.6}|{:.6}|{:.6}|{}|",
        //     ptick.q_circulation_amount, ptick.q_total_shares, ptick.q_market_value, ptick.q_pe_rate, ptick.q_dyn_pb_rate, ptick.q_money_type
        // );
        let _ = write!(
            data,
            "{}|{}|{:.6}|{:.6}|{:.6}|{}|",
            ptick.q_circulation_amount, ptick.q_total_shares, ptick.q_market_value, ptick.q_pe_rate, ptick.q_dyn_pb_rate, ptick.q_money_type
        );

        //买价
        // data = data + &stock_info + "bidprice|";
        let _ = write!(data, "bidprice|");
        for val in ptick.q_bid_price.iter() {
            // data = data + &val.to_string() + "|"
            let _ = write!(data, "{}|", val);
        }

        //买量
        // data += "bidqty|";
        let _ = write!(data, "bidqty|");
        for val in ptick.q_bid_qty.iter() {
            // data = data + &val.to_string() + "|"
            let _ = write!(data, "{}|", val);
        }

        //卖价
        // data += "askprice|";
        let _ = write!(data, "askprice|");
        for val in ptick.q_ask_price.iter() {
            // data = data + &val.trunc().to_string() + "|"
            let _ = write!(data, "{}|", val);
        }
        //卖量
        // data += "askqty|";
        let _ = write!(data, "askqty|");
        for val in ptick.q_ask_qty.iter() {
            // data = data + &val.to_string() + "|"
            let _ = write!(data, "{}|", val);
        }

        data
    }
}
