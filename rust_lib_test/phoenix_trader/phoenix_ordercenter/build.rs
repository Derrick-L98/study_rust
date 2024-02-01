fn build_grpc() {
    tonic_build::configure()
        .out_dir("src/protofiles")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &[
                "../protoes/ordercenter.proto", 
                "../protoes/ordermsg.proto", 
                "../protoes/orderrouter.proto", 
                "../protoes/assetscenter.proto", 
                "../protoes/riskcenter.proto",
                "../protoes/accountriskcenter.proto",
                // "../protoes/OrderDeal.proto",
                // "../protoes/OrderProcess.proto",
                // "../protoes/akacenter.proto",
                // "../protoes/SubscribeHqMsg.proto",
                // "../protoes/HqMsg.proto"
                ], &["../protoes"])
        .unwrap();
}

fn main() {
  build_grpc();
}
