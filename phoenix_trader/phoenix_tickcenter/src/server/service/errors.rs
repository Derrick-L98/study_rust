/* 错误说明
错误代码总长度是6为，其中前两位分类
0 系统类
1 账号类
2 品种类
9 其它
*/
pub enum ErrorCode {
    CodeOk,                 /*0 正常*/
    CodeNoData,             /*1 未查到相关信息*/
    CodeSystemErrRequest,   /*2 请求参数不完整*/
    CodeContractNotexist,   /*3 合约编号不存在*/
    CodeRedisFlagErr,       /*4 Redis配置flag错误*/
    CodeGoodsNotexist,      /*5 品种代码不存在!*/
    CodeExchangeNONotexist, /*6 市场代码不存在 */
    CodeNotFound,           /*7 NOT FOUND*/
    CodeUnknown,            /*未知*/
}

pub struct PhoenixtickError(pub String, pub i32);

pub fn get_error_code(code: ErrorCode) -> PhoenixtickError {
    match code {
        ErrorCode::CodeOk => PhoenixtickError(String::from("Ok"), 0),
        ErrorCode::CodeNoData => PhoenixtickError(String::from("未查到相关信息"), 1),
        ErrorCode::CodeSystemErrRequest => PhoenixtickError(String::from("请求参数不完整"), 2),
        ErrorCode::CodeContractNotexist => PhoenixtickError(String::from("合约编号不存在"), 3),
        ErrorCode::CodeRedisFlagErr => PhoenixtickError(String::from("Redis配置flag错误"), 4),
        ErrorCode::CodeGoodsNotexist => PhoenixtickError(String::from("品种代码不存在"), 5),
        ErrorCode::CodeExchangeNONotexist => PhoenixtickError(String::from("市场代码不存在"), 6),
        ErrorCode::CodeNotFound => PhoenixtickError(String::from("没有找到相应的信息"), 7),
        _ => PhoenixtickError(String::from("其他错误"), 999),
    }
}
