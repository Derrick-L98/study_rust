


//错误码


pub const DATA_ERROR :&'static str="数据错误";
pub const PARAM_USER_EXISTS:&'static str="用户已存在";



//缓存超时时间常量
pub const EXPARE_TIME_8_HOUR:i64=8*60*60;
pub const EXPARE_TIME_1_HOUR:i64=1*60*60;
pub const EXPARE_TIME_1_DAY:i64=24*60*60;

//rediskey前缀

pub const USER_ASSETS_REDIS_KEY:&'static str ="assets_user_";  //用户资产信息
pub const LOCK_USER_REDIS_KEY:&'static str = "lock_assets_user_"; //用户资产信息锁

//持仓key 形式  user_1_stock_1
pub const USER_POSITION_KEY:&'static str="user_{}_stock_{}"; //用户持仓
pub const LOCK_USER_POSITION_KEY:&'static str="lock_user_{}_stock_{}"; //用户持仓锁

pub const SYSTEM_DATE_KEY:&'static str ="system_current_date_"; //系统当前日期
pub const COMMODITY_KEY:&'static str ="commodity_";  //品种key
pub const RATE_CNY_HKD_STOCK_BUY:&'static str ="RATE_CNY_HKD_STOCK_BUY"; //人民币兑港币买汇率
pub const RATE_CNY_HKD_STOCK_SELL:&'static str ="RATE_CNY_HKD_STOCK_SELL"; //人民币兑港币卖汇率
//币种
pub const CURRENCY_CNY:&'static str="CNY";
pub const CURRENCY_HKD:&'static str="HKD";