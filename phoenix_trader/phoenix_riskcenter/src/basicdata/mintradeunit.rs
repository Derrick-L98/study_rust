use common::constant;

const AllTradeUnit: [f64; 11] = [0.001, 0.005, 0.01, 0.02, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0];

#[derive(Debug, Clone, Default)]
pub struct Mintradeunit {}

#[derive(Debug, Clone, Default)]
pub struct PriceByUnit {
  pub dLastMinPrice: f64,
  pub dLastMaxPrice: f64,
}
impl Mintradeunit {
  fn GetMinTradeUnit(dLastPrice: f64, iStockType: i32) -> f64 {
    let mut TradeUnit = 0.0;
    if iStockType == constant::StockType::EMETF as i32 {
      TradeUnit = AllTradeUnit[0];
    } else {
      if (0.01 < dLastPrice && dLastPrice < 0.25) || (dLastPrice == 0.25) {
        TradeUnit = AllTradeUnit[0];
      } else if ((0.25 < dLastPrice) && (dLastPrice < 0.5)) || (dLastPrice == 0.5) {
        TradeUnit = AllTradeUnit[1];
      } else if ((0.5 < dLastPrice) && (dLastPrice < 10.0)) || (dLastPrice == 10.0) {
        TradeUnit = AllTradeUnit[2];
      } else if ((10.0 < dLastPrice) && (dLastPrice < 20.0)) || (dLastPrice == 20.0) {
        TradeUnit = AllTradeUnit[3];
      } else if ((20.0 < dLastPrice) && (dLastPrice < 100.0)) || (dLastPrice == 100.0) {
        TradeUnit = AllTradeUnit[4];
      } else if ((100.0 < dLastPrice) && (dLastPrice < 200.0)) || (dLastPrice == 200.0) {
        TradeUnit = AllTradeUnit[5];
      } else if ((200.0 < dLastPrice) && (dLastPrice < 500.0)) || (dLastPrice == 500.0) {
        TradeUnit = AllTradeUnit[6];
      } else if ((500.0 < dLastPrice) && (dLastPrice < 1000.0)) || (dLastPrice == 1000.0) {
        TradeUnit = AllTradeUnit[7];
      } else if ((1000.0 < dLastPrice) && (dLastPrice < 2000.0)) || (dLastPrice == 2000.0) {
        TradeUnit = AllTradeUnit[8];
      } else if ((2000.0 < dLastPrice) && (dLastPrice < 2000.0)) || (dLastPrice == 2000.0) {
        TradeUnit = AllTradeUnit[9];
      } else if ((5000.0 < dLastPrice) && (dLastPrice < 9995.0)) || (dLastPrice == 9995.0) {
        TradeUnit = AllTradeUnit[10];
      } else {
        log::error!("GetMinTradeUnit 价格不在区间");
      }
    }
    log::info!("GetMinTradeUnit TradeUnit:{}", TradeUnit);
    return TradeUnit;
  }

  fn GetMinMaxPriceByUnit(dLastPriceUnit: f64,ret:&mut PriceByUnit, iStockType: i32)  {
    if iStockType == constant::StockType::EMETF as i32 {
      ret.dLastMinPrice = 0.01;
      ret.dLastMaxPrice = 0.25;
    } else {
      if dLastPriceUnit == AllTradeUnit[0] {
        ret.dLastMinPrice = 0.01;
        ret.dLastMaxPrice = 0.25;
      } else if dLastPriceUnit == AllTradeUnit[1] {
        ret.dLastMinPrice = 0.25;
        ret.dLastMaxPrice = 0.5;
      } else if dLastPriceUnit == AllTradeUnit[2] {
        ret.dLastMinPrice = 0.5;
        ret.dLastMaxPrice = 10.0;
      } else if dLastPriceUnit == AllTradeUnit[3] {
        ret.dLastMinPrice = 10.0;
        ret.dLastMaxPrice = 20.0;
      } else if dLastPriceUnit == AllTradeUnit[4] {
        ret.dLastMinPrice = 20.0;
        ret.dLastMaxPrice = 100.0;
      } else if dLastPriceUnit == AllTradeUnit[5] {
        ret.dLastMinPrice = 100.0;
        ret.dLastMaxPrice = 200.0;
      } else if dLastPriceUnit == AllTradeUnit[6] {
        ret.dLastMinPrice = 200.0;
        ret.dLastMaxPrice = 500.0;
      } else if dLastPriceUnit == AllTradeUnit[7] {
        ret.dLastMinPrice = 500.0;
        ret.dLastMaxPrice = 1000.0;
      } else if dLastPriceUnit == AllTradeUnit[8] {
        ret.dLastMinPrice = 1000.0;
        ret.dLastMaxPrice = 2000.0;
      } else if dLastPriceUnit == AllTradeUnit[9] {
        ret.dLastMinPrice = 2000.0;
        ret.dLastMaxPrice = 2000.0;
      } else if dLastPriceUnit == AllTradeUnit[10] {
        ret.dLastMinPrice = 5000.0;
        ret.dLastMaxPrice = 9995.0;
      } else {
        log::error!("GetMinMaxPriceByUnit 区间的价格不准确");
      }
    }
    
    log::info!("GetMinMaxPriceByUnit MinMaxPriceByUnit{:#?}",ret);
  }

  pub fn IsCorrespondMinTradeUnit(dLastPrice: f64, dOrderPrice: f64, iStockType: i32) -> bool {
    log::info!("IsCorrespondMinTradeUnit dOrderPrice={}", dOrderPrice);
    let mut dOrderPriceUnit = 0.00; //报单价价格区间的变动单位
    let mut dLastPriceUnit = 0.00; //最新价价格区间的变动单位
    let mut iOrderPrice = 0;
    let mut iOrderPriceUnit = 0;
    let mut iLastPrice = 0;
    let mut iLastPriceUnit = 0;

    dOrderPriceUnit = Mintradeunit::GetMinTradeUnit(dOrderPrice, iStockType);
    if dOrderPriceUnit == 0.00 {
      log::error!("IsCorrespondMinTradeUnit dOrderPrice get min trade unit failed");
      return false;
    }

    dLastPriceUnit = Mintradeunit::GetMinTradeUnit(dLastPrice, iStockType);
    if dLastPriceUnit == 0.00 {
      log::error!("IsCorrespondMinTradeUnit dLastPrice get min trade unit failed");
      return false;
    }

    iOrderPrice = (dOrderPrice * 1000.0 + 0.1) as i32;
    iOrderPriceUnit = (dOrderPriceUnit * 1000.0 + 0.1) as i32;
    iLastPrice = (dLastPrice * 1000.0 + 0.1) as i32;
    iLastPriceUnit = (dLastPriceUnit * 1000.0 + 0.1) as i32;

    log::info!("IsCorrespondMinTradeUnit iOrderPrice={}, iOrderPriceUnit={}", iOrderPrice, iOrderPriceUnit);
    log::info!("IsCorrespondMinTradeUnit iLastPrice={}, iLastPriceUnit={}", iLastPrice, iLastPriceUnit);

    if iOrderPriceUnit == iLastPriceUnit {
      if ((iOrderPrice - iLastPrice) % iOrderPriceUnit) != 0 {
        log::error!("(iOrderPrice - iLastPrice) mod iOrderPriceUnit != 0");
        return false;
      }
    } else {
      let mut dLastMinPrice = 0.00;
      let mut dLastMaxPrice = 0.00; //最新价价格区间最小/最大值
      let mut iCheckPrice = 0;
      let mut ret = PriceByUnit::default();
      let _ =Mintradeunit::GetMinMaxPriceByUnit(dLastPriceUnit, &mut ret,iStockType);
      dLastMinPrice = ret.dLastMinPrice;
      dLastMaxPrice = ret.dLastMaxPrice;
      if dLastMinPrice == 0.00 || dLastMaxPrice == 0.00 {
        log::error!("IsCorrespondMinTradeUnit dLastPriceUnit get max or min price unit failed");
        return false;
      }

      iCheckPrice = iLastPrice;

      if iOrderPriceUnit < iLastPriceUnit {
        //报单价区间变动单位 < 最新价区间变动单位  如：(0.5, 10]-0.01    (10, 20]-0.02
        let iLastMinPrice = dLastMinPrice as i32 * 1000;
        log::info!("IsCorrespondMinTradeUnit iLastMinPrice={}", iLastMinPrice);

        //最新价逐渐减变动单位，直到超出范围最低值
        let iLastRemind = (iCheckPrice - iLastMinPrice) % iLastPriceUnit;
        iCheckPrice = iLastMinPrice + iLastRemind;
        if iCheckPrice > iLastMinPrice {
          iCheckPrice -= iLastPriceUnit;

          log::info!("IsCorrespondMinTradeUnit iOrderPriceUnit < iLastPriceUnit, subtract iLastPriceUnit, iCheckPrice={}", iCheckPrice);
        }
        //达到报单价格区间，逐渐减变动单位，直到不大于报单价
        let iOrderRemind = (iCheckPrice - iOrderPrice) % iOrderPriceUnit;
        if iOrderRemind > 0 {
          log::error!("(iCheckPrice - iOrderPrice) mod iOrderPriceUnit != 0");
          return false;
        }
      } else {
        //报单价区间变动单位 > 最新价区间变动单位  如：(10, 20]-0.02    (0.5, 10]-0.01
        let iLastMaxPrice = dLastMaxPrice as i32 * 1000;
        log::error!("IsCorrespondMinTradeUnit iLastMaxPrice={}", iLastMaxPrice);

        //最新价逐渐加变动单位，直到超出范围最高值
        let iLastRemind = (iLastMaxPrice - iCheckPrice) % iLastPriceUnit;
        iCheckPrice = iLastMaxPrice - iLastRemind;
        if iCheckPrice <= iLastMaxPrice {
          iCheckPrice += iLastPriceUnit;

          log::info!("IsCorrespondMinTradeUnit iOrderPriceUnit > iLastPriceUnit, append iLastPriceUnit, iCheckPrice={}", iCheckPrice);
        }
        //达到报单价格区间，逐渐加变动单位，直到不小于报单价
        let iOrderRemind = (iOrderPrice - iCheckPrice) % iOrderPriceUnit;
        if iOrderRemind > 0 {
          log::error!("(iOrderPrice - iCheckPrice) mod iOrderPriceUnit != 0");
          return false;
        }
      }
    }
    log::info!("IsCorrespondMinTradeUnit return true");
    return true;
  }
}
