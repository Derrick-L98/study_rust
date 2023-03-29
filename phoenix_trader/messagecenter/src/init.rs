use super::{notificationclient::NotificationClient, quotationclient::QuotationClient};
use std::time::Duration;

pub async fn init_quotation_listen(mut client: QuotationClient) {
  let mut retry_interval = tokio::time::interval(Duration::from_secs(5));

  tokio::spawn(async move {
    // retry_interval.tick().await;
    loop {
      tokio::select! {
         _ = retry_interval.tick() => {
             log::info!("trying quotation client consume in init......");
             if let Err(err) = client.try_consume().await{
                 log::error!("quotation client consume error: {:?}. start to re-connecting", err);
                 let _ = client.retry_consume().await;
             }
         }
      }
    }
  });
}

pub async fn init_notification_client(mut client: NotificationClient) {
  let mut retry_interval = tokio::time::interval(Duration::from_secs(5));

  tokio::spawn(async move {
    // retry_interval.tick().await;
    loop {
      tokio::select! {
         _ = retry_interval.tick() => {
             log::info!("trying client connection in init......");
             if let Err(err) = client.try_connect().await{
                 log::error!("client connection error: {:?}. start to re-connecting", err);
                 let _ = client.retry_connect().await;
             }

         }
      }
    }
  });
  // Ok(())
}
