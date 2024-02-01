fn build_grpc() {
  tonic_build::configure()
    .out_dir("src/protofiles")
    .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .compile(&["../protoes/assetscenter.proto", "../protoes/accountriskcenter.proto", "../protoes/akacenter.proto"], &["../protoes"])
    .unwrap();
}
fn main() {
  build_grpc();
}
