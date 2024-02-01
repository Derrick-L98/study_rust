use std::error::Error;
use std::time::Duration;

// use futures::stream;
// use rand::rngs::ThreadRng;
// use rand::Rng;
use tokio::time;
use tokio_stream::StreamExt;
use tonic::transport::Channel;

mod config;
mod protofiles;
mod server;
use crate::protofiles::RouterMsg;
use crate::protofiles::orderrouter::order_router_service_client::OrderRouterServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 单向流
    // let mut client = OrderRouterServiceClient::connect("http://[::1]:50051").await?;
    // let request = tonic::Request::new(RouterMsg { 
    //     msg_type: todo!(), 
    //     msg: todo!(), 
    //     msg_id: todo!(), 
    //     msg_time: todo!() 
    // });
    // let response = client.order_transfer(request).await?;
    // println!("RESPONSE={:?}", response);


    //双向流
    let mut client = OrderRouterServiceClient::connect("http://127.0.0.1:8765").await?;
    let _ret = order_transfer(&mut client).await;
    let _ret = order_transfer_2(&mut client).await;
    Ok(())
}

//方法1
async fn order_transfer(client: &mut OrderRouterServiceClient<Channel>) -> Result<(), Box<dyn Error>> {
    let start = time::Instant::now();

    let outbound = async_stream::stream! {
        //定时器
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            let time = interval.tick().await;
            let elapsed = time.duration_since(start);
            let note = RouterMsg { 
                    msg_type: 0, 
                    msg: None, 
                    msg_id: 1, 
                    msg_time: 1 
                };
            yield note;
        }
    };

    let response = client.order_transfer(tonic::Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    while let Some(note) = inbound.message().await? {
        println!("NOTE = {:#?}", note);
    }

    Ok(())
}

//方法2
async fn order_transfer_2(client: &mut OrderRouterServiceClient<Channel>) -> Result<(), Box<dyn Error>> {
    let req_data = tokio_stream::iter(1..usize::MAX).map(move |_i| RouterMsg { 
        msg_type: 1, 
        msg: None, 
        msg_id: 1, 
        msg_time: 2 
    });
    let in_stream = req_data.take(10); //创建一个新流，该流最多包含n个基础流项。
    let sub_ret = client.order_transfer(in_stream).await;
    if sub_ret.as_ref().is_err() {
        log::error!("subscribe error:{:?}", sub_ret.as_ref().err());
    }
    let response = sub_ret.unwrap();

    let mut inbound = response.into_inner();
    while let Ok(inbound_data) = inbound.message().await {
        if inbound_data.is_some() {
            let rep_data = inbound_data.unwrap();
            println!("{:#?}", rep_data);
        } else {
            println!("inbound data empty");
        }
    }
    Ok(())
}
