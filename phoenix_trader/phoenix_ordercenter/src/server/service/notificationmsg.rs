use anyhow::Result;
use chrono::{Datelike, Timelike};
use messagecenter::notificationclient::NotificationClient;
use messagecenter::protofiles::phoenixnotification::{
    NotificationMessage, 
    NotificationType, 
    NotificationOrderExec, 
    NotificationOrderStatus,
    MessageBody, OrderExecType, NotificationOrderInfo,
};
use rust_decimal::prelude::ToPrimitive;
use crate::protofiles::ExecType;
use crate::common::common::OrderDetail;
use crate::dataservice::entities::prelude::PhoenixOrdStockorder;


pub async fn send_order_exec_to_mq(detail: &OrderDetail, order: &PhoenixOrdStockorder, mq_client: &NotificationClient) -> Result<()> {
    log::info!("send to mq from exec message");
    let mut notification_message = NotificationMessage::default();
    notification_message.msg_type = NotificationType::OrderExecMsg as i32;
    let mut message_body = MessageBody::default();
    let mut notification_order_exec = NotificationOrderExec::default();
    notification_order_exec.unit_id = order.unit_id;
    notification_order_exec.order_id = order.order_no;
    notification_order_exec.stock_id = order.stock_id;
    notification_order_exec.channel_id = detail.channel_id;
    notification_order_exec.channel_type = detail.channel_type;
    if detail.exec_type == ExecType::Confirm{ 
        notification_order_exec.exec_type = OrderExecType::NewOrder as i32;
        notification_order_exec.order_quantity = order.order_amount;
    } else if detail.exec_type == ExecType::Filled {
        notification_order_exec.order_quantity = detail.deal_amount;
        notification_order_exec.deal_no = detail.deal_id;
        notification_order_exec.order_price = detail.deal_price.to_f64().unwrap_or_default();
        notification_order_exec.exec_type = OrderExecType::OrderFill as i32;
    } else if detail.exec_type == ExecType::Canceled || detail.exec_type == ExecType::Rejected {
        notification_order_exec.order_quantity = detail.cancel_amount;
        notification_order_exec.exec_type = OrderExecType::OrderCancelled as i32;
    }
    notification_order_exec.order_direction = order.order_direction;
    notification_order_exec.memo = detail.memo.to_owned();
    notification_order_exec.msg_id = detail.msg_id;

    message_body.msg_orderexec = Some(notification_order_exec);
    notification_message.msg_body = Some(message_body);
    let key = format!("notification.order.exec.{}",order.order_no);
    // log::info!("消息推送: {:?}", notification_message);
    if let Err(err) = mq_client.try_publish(&key, &notification_message).await {
        log::error!("{}", err);
    }
    Ok(())
}

pub async fn send_order_status_to_mq(detail: &OrderDetail, order: &PhoenixOrdStockorder, mq_client: &NotificationClient) -> Result<()> {
    log::info!("send to mq from order status message");
    let mut notification_message = NotificationMessage::default();
    notification_message.msg_type = NotificationType::OrderStatusChanged as i32;
    let mut message_body = MessageBody::default();
    let mut notification_order_status = NotificationOrderStatus::default();
    notification_order_status.unit_id = order.unit_id;
    notification_order_status.order_id = order.order_no;

    if detail.exec_type == ExecType::Confirm{
        notification_order_status.order_action = "1".to_owned();// 1:新订单  2:成交  3:撤单
    } else if detail.exec_type == ExecType::Filled {
        notification_order_status.order_quantity = detail.deal_amount;
        notification_order_status.deal_no = detail.deal_id;
        notification_order_status.order_action = "2".to_owned();
    } else if detail.exec_type == ExecType::Canceled || detail.exec_type == ExecType::Rejected {
        notification_order_status.order_quantity = detail.cancel_amount;
        notification_order_status.order_action = "3".to_owned();
    }
    notification_order_status.order_price = detail.deal_price.to_f64().unwrap_or_default();
    notification_order_status.order_status = order.order_status;
    notification_order_status.memo = detail.memo.to_owned();

    message_body.msg_orderstatus = Some(notification_order_status);
    notification_message.msg_body = Some(message_body);
    let key = format!("notification.order.status.{}",order.order_no);
    // log::info!("消息推送: {:?}", notification_message);
    if let Err(err) = mq_client.try_publish(&key, &notification_message).await {
        log::error!("{}", err);
    }
    Ok(())
}

pub async fn send_order_info_to_mq(detail: &OrderDetail, order: &PhoenixOrdStockorder, mq_client: &NotificationClient) -> Result<()> {
    let mut notification_message = NotificationMessage::default();
    notification_message.msg_type = NotificationType::UserOrderInfo as i32;
    let mut message_body = MessageBody::default();
    let mut msg_orderinfo = NotificationOrderInfo::default();
    msg_orderinfo.unit_id = order.unit_id;
    msg_orderinfo.order_direction = order.order_direction;
    msg_orderinfo.stock_code = order.stock_code.to_owned();
    msg_orderinfo.order_price = order.order_price.to_f64().unwrap_or_default();
    msg_orderinfo.order_no = order.order_no;
    msg_orderinfo.order_type = order.order_type;
    msg_orderinfo.deal_price = detail.deal_price.to_f64().unwrap_or_default();
    msg_orderinfo.order_amount = order.order_amount;
    msg_orderinfo.deal_amount = order.deal_amount;
    msg_orderinfo.canceled_amount = order.cancel_amount;
    //2023-02-01 14:35:25
    let create_time = utility::timeutil::from_timestamp_to_naive_date_time(order.create_time);
    msg_orderinfo.create_time = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", create_time.year(), create_time.month(), create_time.day(), create_time.hour(), create_time.minute(), create_time.second());
    msg_orderinfo.last_deal_time = detail.deal_time.to_owned();
    msg_orderinfo.order_status = order.order_status;
    msg_orderinfo.stock_id = order.stock_id;
    msg_orderinfo.order_memo = order.order_memo.to_owned();

    message_body.msg_orderinfo = Some(msg_orderinfo);
    notification_message.msg_body = Some(message_body);
    let key = format!("notification.order.info.{}",order.unit_id);
    // log::info!("消息推送: {:?}", notification_message);
    if let Err(err) = mq_client.try_publish(&key, &notification_message).await {
        log::error!("{}", err);
    }

    Ok(())
}

pub async fn notificationmsg(detail: &OrderDetail, order: &PhoenixOrdStockorder, mq_client: &NotificationClient) {
    let _ = send_order_info_to_mq(&detail, &order, mq_client).await;
    let _ = send_order_status_to_mq(&detail, &order, mq_client).await;
    let _ = send_order_exec_to_mq(&detail, &order, mq_client).await;
}