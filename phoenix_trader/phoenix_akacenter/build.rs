fn build_grpc() {
  tonic_build::configure()
    .out_dir("src/protofiles")
    .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .compile(&["../protoes/riskcenter.proto"], &["../protoes"])
    .unwrap();
}

fn main() {
  build_grpc();
}
