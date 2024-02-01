// use std::{env, path::PathBuf};
// extern crate protoc_rust;
fn build_grpc() {
    tonic_build::configure()
        .out_dir("src/protofiles")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        // .type_attribute("protos/hqmsg.protos.YsHqInfo", "#[derive(Hash)]")
        // .type_attribute("protos/marketdata.protos.ContractMsg", "#[derive(Hash)]")
        .compile(&["../protoes/ordermsg.proto","../protoes/orderrouter.proto"], &["../protoes"])
        // .compile(&["protos/hqmsg.protos", "protos/marketdata.protos"], &["protos"])
        .unwrap();
}

fn main() {
    build_grpc();
}
