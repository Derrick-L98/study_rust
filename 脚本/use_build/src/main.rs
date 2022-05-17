//构建脚本
//使用包前，先编译build.rs

//1、添加配置：在文件Cargo.toml package部分添加构建内容
//2、在src目录同级创建需要构建的文件
//3、在文件中写入构建的内容
//4、在src目录main.rs中导入




include!(concat!(env!("OUT_DIR"),"/hello.rs"));//使用脚本构建的外部函数
fn main() {
    println!("{}",say_hello());
}
