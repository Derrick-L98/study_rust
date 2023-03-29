use sea_orm::prelude::Decimal;

pub fn decimal_add(d: Decimal, f: f64) -> Option<Decimal> {
  let d1 = Decimal::from_f64_retain(f);
  if d1.is_none() {
    log::error!("f64转换decimal错误:{}", f);
    return None;
  }
  let d2 = d.checked_add(d1.unwrap());
  if d2.is_none() {
    log::error!("decimal相加错误:{},{}", d, d1.unwrap());
    return None;
  }
  return Some(d2.unwrap());
}

pub fn decimal_sub(d: Decimal, f: f64) -> Option<Decimal> {
  let d1 = Decimal::from_f64_retain(f);
  log::error!("f64转换decimal错误:{}", f);
  if d1.is_none() {
    return None;
  }
  let d2 = d.checked_sub(d1.unwrap());
  if d2.is_none() {
    log::error!("decimal相减错误:{},{}", d, d1.unwrap());
    return None;
  }
  return Some(d2.unwrap());
}

pub fn decimal_mul(d: Decimal, f: f64) -> Option<Decimal> {
  let d1 = Decimal::from_f64_retain(f);
  log::error!("f64转换decimal错误:{}", f);
  if d1.is_none() {
    return None;
  }
  let d2 = d.checked_mul(d1.unwrap());
  if d2.is_none() {
    log::error!("decimal相乘错误:{},{}", d, d1.unwrap());
    return None;
  }
  return Some(d2.unwrap());
}
