#[derive(Debug, Clone, Copy)]
pub enum KLineType
//行情类型
{
    Hq1Kline = 1,
    Hq5Kline = 5,
    Hq10Kline = 10,
    Hq30Kline = 30,
    Hq60Kline = 60,
    Hq24Kline = 1440,
    // HqWeekKline = 168,
    // HqMonthKline = 720
}

#[derive(Debug, Default, Clone)]
pub struct KLineData {
    pub stock_code: String,           //股票代码
    pub period: i32,                  //周期
    pub begin_minutes: i32,           //开始分钟
    pub end_minutes: i32,             //结束分钟
    pub prev_minutes: i32,            //记录K线上一个周期在当天分钟数, 即最新的已形成的周期=====> 用来更新K线
    pub tick_time: String,            //最新一条tick的时间, 来一条tick就更新此值, 和prev_minutes对应
    pub open_price: f64,              //开
    pub close_price: f64,             //收
    pub high_price: f64,              //高
    pub low_price: f64,               //低
    pub last_price: f64,              //新
    pub average_price: f64,           //均价
    pub pre_close_price: f64,         //昨收价
    pub current_period_volume: f64,   //本周期最新总成交量
    pub current_period_turnover: f64, //本周期最新总成交额
    pub is_fake: bool,                //是否是补数据更新的
}

impl KLineData {
    pub async fn new() -> Self {
        KLineData { ..Default::default() }
    }
}


// #[derive(Debug, Default, Clone)]
// pub struct KLineData {
//     pub stock_code: String,           //股票代码
//     pub period: i32,                  //周期
//     pub begin_minutes: i32,           //开始分钟
//     pub end_minutes: i32,             //结束分钟
//     pub prev_minutes: i32,            //记录K线上一个周期在当天分钟数, 即最新的已形成的周期
//     pub latest_tick_time: i64,        //最新一条tick的时间戳, 来一条tick就更新此值, 和prev_minutes对应
//     pub trade_day: String,            //交易日, yyyymmdd
//     pub utc_trade_day: String,        //UTC交易日, yyyymmdd
//     pub open_price: f64,              //开
//     pub close_price: f64,             //收
//     pub high_price: f64,              //高
//     pub low_price: f64,               //低
//     pub last_price: f64,              //新
//     pub average_price: f64,           //均价
//     pub pre_close_price: f64,         //昨收价
//     pub prev_volume: f64,             //上一周期最新总成交量
//     pub prev_turnover: f64,           //上一周期最新总交易额
//     pub current_period_volume: f64,   //本周期最新总成交量
//     pub current_period_turnover: f64, //本周期最新总成交额
//     pub is_fake: bool,                //是否是补数据更新的
// }

// impl KLineData {
//     pub async fn new() -> Self {
//         KLineData { ..Default::default() }
//     }
// }
