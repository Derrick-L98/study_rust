#[derive(Debug, Default, Clone)]
pub struct SummarizedTickData {
    pub stock_code: String,  //股票代码
    pub tick_time: i32,      //tick时间(后面) 20220510150000
    pub open_price: f64,     //开盘价
    pub close_price: f64,    //收盘价
    pub high_price: f64,     //最高价
    pub low_price: f64,      //最低价
    pub last_price: f64,     //最新价
    pub average_price: f64,  //均价
    pub total_volume: f64,   //本周期总成交量
    pub total_turnover: f64, //本周期最新总成交额
}

impl SummarizedTickData {
    pub fn new() -> Self {
        SummarizedTickData { ..Default::default() }
    }
}