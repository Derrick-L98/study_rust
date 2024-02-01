use std::fs;

fn main() {
  let outdir = "src/protofiles";
  fs::create_dir_all(outdir).unwrap();
  tonic_build::configure()
    .out_dir(outdir)
    .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .compile(&["../protoes/logcenter.proto", "../protoes/akacenter.proto"], &["../protoes"])
    .unwrap();
}
