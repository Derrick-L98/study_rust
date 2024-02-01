/* 错误说明
错误代码总长度是6为，其中前两位分类
E0 系统类
E1 账号类
E2 品种类
E3 通道类
E4 交易类
E9 其它
*/
pub enum ErrorCode {
  CodeOk,                            /*E00000 正常*/
  CodeSystemErrRequest,              /*E00001 请求参数不完整*/
  CodeSystemNotPermitted,            /*E00002 操作不允许*/
  CodeAccountBuyClosed,              /*E10001 不能下买单*/
  CodeAccountNottradable,            /*E10002 账号禁止交易*/
  CodeAccountNotexist,               /*E10003 账号不存在*/
  CodeAccountNoStockPosition,        /*E10004 没有找到持仓数据*/
  CodeAccountExceedPosition,         /*E10005 下单量超过可用交易量*/
  CodeAgentAccountError,             /*E10006 账号权限不足*/
  CodeAccountRiskRateHigh,           /*E10007 账户风险率已达到预警线*/
  CodeCommidityNotExist,             /*E20001 交易品种不存在!*/
  CodeCommidityChannelNotConfigured, /*E20002 暂不支持该品种交易*/
  CodeMarketClosed,                  /*E20003 当前市场非交易日*/
  CodeChannelClosed,                 /*E30001 交易通道暂时关闭*/
  CodeChannelPartialClosed,          /*E30002 部分交易通道暂时关闭，请减少报单数量再试*/
  CodeFundCurrencyError,             /*E40001 资金类型不对 */
  CodeFundAccountError,              /*E40002 资金账号不对 */
  CodeRealCashError,                 /*E40004 可用资金不足*/
  CodeGemPositionError,              /*E40003 本次交易已达创业板上限*/
  CodeOrderExceedMaxNumber,          /*E50001 单笔最大数量超过限制*/
  CodeOrderExceedMinNumber,          /*E50002 单笔最小数量不够*/
  CodeOrderExceedMaxMoney,           /*E50003 单笔最大金额超过限制*/
  CodeOrderErrorNumber,              /*E50004 下单数量错误 */
  CodeMinTradeUnitError,             /*E50005 最小变动单位错误 */
  CodeNotFound,                      /*E90000 NOT FOUND*/
  CodeUnknown,                       /*未知*/
}

pub struct PhoenixError(pub String, pub String);

pub fn get_error_code(code: ErrorCode) -> PhoenixError {
  match code {
    ErrorCode::CodeOk => PhoenixError(String::from("E00000"), String::from("")),
    ErrorCode::CodeSystemErrRequest => PhoenixError(String::from("E00001"), String::from("请求参数不完整")),
    ErrorCode::CodeSystemNotPermitted => PhoenixError(String::from("E00002"), String::from("操作不允许")),
    ErrorCode::CodeAccountBuyClosed => PhoenixError(String::from("E10001"), String::from("账号不能下买单")),
    ErrorCode::CodeAccountNottradable => PhoenixError(String::from("E10002"), String::from("账号禁止交易")),
    ErrorCode::CodeAccountNotexist => PhoenixError(String::from("E10003"), String::from("账号不存在")),
    ErrorCode::CodeAccountNoStockPosition => PhoenixError(String::from("E10004"), String::from("没有找到持仓数据")),
    ErrorCode::CodeAccountExceedPosition => PhoenixError(String::from("E10005"), String::from("下单量超过可用交易量")),
    ErrorCode::CodeAgentAccountError => PhoenixError(String::from("E10006"), String::from("账号权限不足")),
    ErrorCode::CodeAccountRiskRateHigh => PhoenixError(String::from("E10007"), String::from("账户风险率已达到预警线")),
    ErrorCode::CodeCommidityNotExist => PhoenixError(String::from("E20001"), String::from("交易品种不存在")),
    ErrorCode::CodeCommidityChannelNotConfigured => PhoenixError(String::from("E20002"), String::from("暂不支持该品种交易")),
    ErrorCode::CodeMarketClosed => PhoenixError(String::from("E20003"), String::from("当前市场非交易日")),
    ErrorCode::CodeChannelClosed => PhoenixError(String::from("E30001"), String::from("交易通道暂时关闭")),
    ErrorCode::CodeChannelPartialClosed => PhoenixError(String::from("E30002"), String::from("部分交易通道暂时关闭，请减少报单数量再试")),
    ErrorCode::CodeFundCurrencyError => PhoenixError(String::from("E40001"), String::from("资金类型不对")),
    ErrorCode::CodeFundAccountError => PhoenixError(String::from("E40002"), String::from("资金账号不对")),
    ErrorCode::CodeGemPositionError => PhoenixError(String::from("E40003"), String::from("本次交易已达创业板上限")),
    ErrorCode::CodeRealCashError => PhoenixError(String::from("E40004"), String::from("可用资金不足")),
    ErrorCode::CodeOrderExceedMaxNumber => PhoenixError(String::from("E50001"), String::from("单笔最大数量超过限制")),
    ErrorCode::CodeOrderExceedMinNumber => PhoenixError(String::from("E50002"), String::from("单笔最小数量不够")),
    ErrorCode::CodeOrderExceedMaxMoney => PhoenixError(String::from("E50003"), String::from("单笔最大金额超过限制")),
    ErrorCode::CodeOrderErrorNumber => PhoenixError(String::from("E50004"), String::from("下单数量错误")),
    ErrorCode::CodeMinTradeUnitError => PhoenixError(String::from("E50005"), String::from("最小变动单位错误")),
    ErrorCode::CodeNotFound => PhoenixError(String::from("E90000"), String::from("没有找到相应的信息")),
    _ => PhoenixError(String::from("E99999"), String::from("其他错误")),
  }
}
