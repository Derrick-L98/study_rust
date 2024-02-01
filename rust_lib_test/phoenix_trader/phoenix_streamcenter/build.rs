fn build_grpc() {
  tonic_build::configure()
    .out_dir("src/protofiles")
    .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .compile(&["protos/hqmsg.proto"], &["protoes"])
    // .compile(&["protos/hqmsg.proto", "protos/marketdata.proto"], &["protos"])
    .unwrap();
}

fn main() {
  build_grpc();
}
