pub enum CalculateAssetsType {
  Cal_Type_Rate, //汇率变动

  Cal_Type_Assets, //资产变动导致需要重算资产

  Cal_Stock_Margin_Change, //保证金发生变动

  Cal_Type_Position,

  Cal_Type_Price,

  Cal_Type_Marginrate,
}
pub const DEFAULT_ERROR_CODE: &'static str = "9999";
pub const DEFAULT_SUCCESS_CODE: &'static str = "0";
pub const DEFAULT_SUCCESS: &'static str = "success";
pub const DEFAULT_USER_NOT_FOUND: &'static str = "查询用户不存在";

#[derive(PartialEq, Clone, Debug)]
pub enum HttpCode {
  OK = 20000,
  Error = 201,
}
