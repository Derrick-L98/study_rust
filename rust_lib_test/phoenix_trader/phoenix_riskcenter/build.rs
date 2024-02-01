// use std::{env, path::PathBuf};

fn build_grpc() {
    tonic_build::configure()
        .out_dir("src/protofiles")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["protos/riskcenter.proto","protos/accountriskcenter.proto","protos/SubscribeHqMsg.proto"], &["protos"])
        .unwrap();
}

// fn build_proto() {
//     // add project-specific build logic here...
//     protoc_rust::Codegen::new()
//         .out_dir("src/protofiles")
//         .inputs(&["protos/HqMsg.proto", "protos/OmsMessage.proto"])
//         .include("protos")
//         .run()
//         .expect("protoc");
// }

fn main() {
    build_grpc();
}
