// 报盘消息格式

message RouterMsg {
    MsgType    msg_type = 1;          // 消息类型:见MsgType
    MsgContent msg      = 2;          // 与msg_type对应
    int64      msg_id   = 3;          // 消息ID
    int64      msg_time = 4;          // 消息创建时间
}

// 空消息体:
RouterMsg {
    msg_type: MsgType::Register,//注册 0
    msg: None,
    msg_id: 0,
    msg_time: 0,
}

// 注册:
RouterMsg {
    msg_type: MsgType::Register,//注册 0
    msg: Some(
        MsgContent {
            register_req: Some( //注册消息
                RegisterReq {
                    channel_id: [
                        12, 
                    ],
                },
            ),
            order_msg: None,
            exec_msg: None,
            resp: None,
        },
    ),
    msg_id: 1,
    msg_time: 20221024,
}

// 委托(下单.撤单):
RouterMsg {
    msg_type: MsgType::Order,    //订单消息 1
    msg: Some(
        MsgContent {
            register_req: None, 
            order_msg: Some( //下单委托消息
                OrderMsg {
                    //委托下单: 委托号.证券代码.市场xshg.委托方向.委托数量.委托价格.价格类型.币种.通道类型.通道id ?[委托确认号.关联委托]
                    order_type: OrderType::PlaceOrder, // 下单委托 1
                    order_id: ,             // 委托id
                    brk_order_id: ,         // 委托确认号
                    stock_code: ,           // 证券代码
                    exchange_code: ,        // 市场xshg
                    order_side: ,           // 委托方向  1=buy  2=sell
                    order_qty: ,            // 订单数量或撤单数量
                    price_type: ,           // 价格类型(市价限价)
                    order_price: ,          // 委托价格
                    currency_no: ,          // 币种
                    channel_type: ,         // 通道类型 1 外盘, 2 内盘
                    channel_id: ,           // 通道id
                    cancel_id: ,            // 撤单id
                }
            ),
            exec_msg: None,
            resp: None,
        },
    ),
    msg_id: 1,
    msg_time: 20221024,
}

RouterMsg {
    msg_type: MsgType::Order,    //订单消息 1
    msg: Some(
        MsgContent {
            register_req: None, 
            order_msg: Some( //撤单委托消息
                OrderMsg {
                    //撤单委托: 委托号.确认号.撤单ID.证券代码.市场xshg.委托方向.撤单数量.价格类型.通道类型.通道id 
                    order_type: OrderType::CancelOrder,// 撤单委托 2
                    order_id: ,             // 委托id
                    brk_order_id: ,         // 委托确认号
                    stock_code: ,           // 证券代码
                    exchange_code: ,        // 市场xshg
                    order_side: ,           // 委托方向  1=buy  2=sell
                    order_qty: ,            // 订单数量或撤单数量
                    price_type: ,           // 价格类型(市价限价)
                    order_price: ,          // 委托价格
                    currency_no: ,          // 币种
                    channel_type: ,         // 通道类型 1 外盘, 2 内盘
                    channel_id: ,           // 通道id
                    cancel_id: ,            // 撤单id
                }
            ),
            exec_msg: None,
            resp: None,
        },
    ),
    msg_id: 1,
    msg_time: 20221024,
}

// 回报(确认.成交.撤单):
RouterMsg {
    msg_type: MsgType::Exec,//订单执行回报 2
    msg: Some(
        MsgContent {
            register_req: None,
            order_msg: None,
            exec_msg: Some( //确认消息
                ExecMsg {
                    //确认: 委托号.确认号.通道类型.通道ID.状态[2]
                    exec_type: ExecType::Confirm,     // 确认回报 1
                    order_id: ,             // 委托序号
                    exec_qty: ,             // 本次成交数量或撤单数量
                    exec_price: ,           // 本次成交价格
                    exec_id: ,              // 成交编号
                    exec_time: ,            // 发生时间  2019-12-19 11:21:19
                    order_side: ,           // 订单方向  1=Buy, 2=Sell
                    brk_order_id: ,         // 委托确认号
                    channel_id: ,           // 通道id
                    channel_type: ,         // 1 外盘, 2 内盘
                    exec_memo: ,            // 备注
                }
            ),
            resp: None,
        },
    ),
    msg_id: 1,
    msg_time: 20221024,
}

RouterMsg {
    msg_type: MsgType::Exec,//订单执行回报 2
    msg: Some(
        MsgContent {
            register_req: None,
            order_msg: None,
            exec_msg: Some( //成交消息
                ExecMsg {
                    // 成交: 委托号.成交数量.成交价格.成交编号.成交时间.委托方向.通道类型.通道ID
                    exec_type: ExecType::Filled,      // 成交回报 2
                    order_id: ,             // 委托序号
                    exec_qty: ,             // 本次成交数量
                    exec_price: ,           // 本次成交价格
                    exec_id: ,              // 成交编号
                    exec_time: ,            // 发生时间  2019-12-19 11:21:19
                    order_side: ,           // 订单方向  1=Buy, 2=Sell
                    brk_order_id: ,         // 委托确认号
                    channel_id: ,           // 通道id
                    channel_type: ,         // 1 外盘, 2 内盘
                    exec_memo: ,            // 备注
                }
            ),
            resp: None,
        },
    ),
    msg_id: 1,
    msg_time: 20221024,
}

RouterMsg {
    msg_type: MsgType::Exec,//订单执行回报 2
    msg: Some(
        MsgContent {
            register_req: None,
            order_msg: None,
            exec_msg: Some( //撤单回报消息
                ExecMsg {
                    //撤单: 委托号.[W-撤成、R-废单].备注.撤单数量.通道类型.通道ID
                    exec_type: ExecType::Canceled,    // 撤单回报 3
                    order_id: ,             // 委托序号
                    exec_qty: ,             // 撤单数量
                    exec_price: ,           // 本次成交价格
                    exec_id: ,              // 成交编号
                    exec_time: ,            // 发生时间  2019-12-19 11:21:19
                    order_side: ,           // 订单方向  1=Buy, 2=Sell
                    brk_order_id: ,         // 委托确认号
                    channel_id: ,           // 通道id
                    channel_type: ,         // 1 外盘, 2 内盘
                    exec_memo: ,            // 备注
                }
            ),
            resp: None,
        },
    ),
    msg_id: 1,
    msg_time: 20221024,
}