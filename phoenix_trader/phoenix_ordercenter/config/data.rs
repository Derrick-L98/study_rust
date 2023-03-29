RouterMsg {
    msg_type: Order,
    msg_content: Some(
        MsgContent {
            register_req: None,
            order_msg: Some(
                OrderMsg {
                    order_type: Place,
                    order_id: 7000641805742120955,
                    brk_order_id: "",
                    stock_code: "600000",
                    exchange_id: 101,
                    order_direction: 1,
                    order_qty: 2000,
                    price_type: 1,
                    order_price: 6.88,
                    currency_no: "CNY",
                    channel_type: 1,
                    channel_id: 12,
                    cancel_id: 0,
                },
            ),
            exec_msg: None,
            resp: None,
        },
    ),
    msg_id: 7000641805742120954,
    msg_time: 101112,
}

//行情
YsHqInfo {
    exchange_id: "XSHG",
    commodity_no: "",
    contract_no1: "600000_XSHG",
    currency_no: "",
    tapidtstamp: "2022-11-22 10:11:51.410",
    q_pre_settle_price: 6.88,
    q_pre_position_qty: 0,
    q_opening_price: 6.89,
    q_last_price: 6.9,
    q_high_price: 6.93,
    q_low_price: 6.89,
    q_limit_up_price: 7.57,
    q_limit_down_price: 6.19,
    q_total_qty: 48769,
    q_total_turnover: 33711122.0,
    q_position_qty: 0,
    q_average_price: 6.913,
    q_closing_price: 0.0,
    q_last_qty: 0.0,
    q_bid_price: [
        6.9,
        6.89,
        6.88,
        6.87,
        6.86,
    ],
    q_bid_qty: [
        1381,
        2037,
        2649,
        741,
        1332,
    ],
    q_ask_price: [
        6.91,
        6.92,
        6.93,
        6.94,
        6.95,
    ],
    q_ask_qty: [
        3310,
        3193,
        3888,
        2548,
        3325,
    ],
    q_change_rate: 0.29,
    q_change_value: 0.02,
    q_pre_closing_price: 6.88,
    q_total_bid_qty: 0,
    q_total_ask_qty: 0,
    q_turnover_ratio: 0.02,
    q_amplitude: 0.58,
    q_pe_rate: 3.754,
    q_dyn_pb_rate: 0.35,
    q_vol_ratio: 1.27,
    q_circulation_amount: 29352173656,
    q_total_shares: 29352173656,
    q_market_value: 202529998226.0,
    q_money_type: "CNY",
    q_industry_info: "",
    q_last_turnover: 0.0,
    q_entrust_rate: -33.29,
    q_bid_qty2: [],
    q_ask_qty2: [],
    q_total_qty2: 0.0,
}

