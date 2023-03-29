// use std::{env, path::PathBuf};
// extern crate protoc_rust;
fn build_grpc() {
    tonic_build::configure()
        .out_dir("src/protofiles")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        // .type_attribute("protos/hqmsg.proto.YsHqInfo", "#[derive(Hash)]")
        // .type_attribute("protos/marketdata.proto.ContractMsg", "#[derive(Hash)]")
        // .compile(&["protos/hqmsg.proto", "protos/marketdata.proto", "protos/phoenixtickcenter.proto"], &["protos"])
        .compile(&["protos/hqmsg.proto", "protos/phoenixtickcenter.proto"], &["protos"])
        .unwrap();
}

fn main() {
    build_grpc();
}
