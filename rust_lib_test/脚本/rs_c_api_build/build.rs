extern crate cc;//编译工具导入

fn main() {
    cc::Build::new().file("src/hello.c").compile("hello");
}