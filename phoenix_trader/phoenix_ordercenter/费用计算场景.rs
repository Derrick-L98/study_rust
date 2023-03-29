
买
1.
交易:HKD   费用:CNY
fee_ratio: 0.00027, maximum_fee: 100.0, minimum_fee: 2.0, currency_type: Cny,
fee_type: 6, amount: 100, price: 126.3, fee_ratio: 0.00027
item_fee: 3.4101
currency_direction: CNY2HKD  ====> RATE_CNY_HKD_STOCK_SELL
汇率: 1.3500
if fee.minimum_fee > 0.001 && item_fee < fee.minimum_fee * rate {
    item_fee = fee.minimum_fee * rate;
}
if fee.maximum_fee > 0.001 && item_fee > fee.maximum_fee * rate {
    item_fee = fee.maximum_fee * rate;
}
item_fee: 3.4101

2.
交易:HKD   费用:HKD
fee_ratio: 0.0005, maximum_fee: 100.0, minimum_fee: 2.0, currency_type: Hkd
fee_type: 5, amount: 100, price: 126.3, fee_ratio: 0.0005
item_fee: 6.315
汇率: 1
if fee.minimum_fee > 0.001 && item_fee < fee.minimum_fee * rate {
    item_fee = fee.minimum_fee * rate;
}
if fee.maximum_fee > 0.001 && item_fee > fee.maximum_fee * rate {
    item_fee = fee.maximum_fee * rate;
}
item_fee: 6.315

3.
交易:CNY   费用:HKD
fee_ratio: 2e-5, maximum_fee: 100.0, minimum_fee: 2.0, currency_type: Hkd
fee_type: b, amount: 100, price: 14.59, fee_ratio: 0.00002
item_fee: 0.0291815
urrency_direction: HKD2CNY  ====> RATE_CNY_HKD_STOCK_BUY
汇率: 1.4320
if fee.minimum_fee > 0.001 && item_fee < fee.minimum_fee * rate {
    item_fee = fee.minimum_fee * rate;
}
if fee.maximum_fee > 0.001 && item_fee > fee.maximum_fee * rate {
    item_fee = fee.maximum_fee * rate;
}
item_fee: 2.86


3.
交易:CNY   费用:CNY
fee_ratio: 2e-5, maximum_fee: 0.0, minimum_fee: 0.0, currency_type: Cny
fee_type: 3, amount: 100, price: 14.59, fee_ratio: 0.00002
item_fee: 0.02918
汇率: 1
if fee.minimum_fee > 0.001 && item_fee < fee.minimum_fee * rate {
    item_fee = fee.minimum_fee * rate;
}
if fee.maximum_fee > 0.001 && item_fee > fee.maximum_fee * rate {
    item_fee = fee.maximum_fee * rate;
}
item_fee: 0.02918
