use std::env;//获取环境变量
use std::fs::File;
use std::io::Write;
use std::path::Path;

//实际上是在编译包之前，先编译build.rs
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();             //文件目录
    let dest_path = Path::new(&out_dir).join("hello.rs");   //文件路径
    let mut f = File::create(&dest_path).unwrap();          //创建文件

    f.write_all(b"
        pub fn say_hello() -> &'static str {
            \"hello\"
        }
    ").unwrap();

}