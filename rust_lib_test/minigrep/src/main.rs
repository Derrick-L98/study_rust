use minigrep::Config;
use std::env;//args()返回一个迭代器，调用collect方法
use std::process;


fn main() {
    //let args : Vec<String> = env::args().collect();//用于接收命令行参数
    //let config = Config::new(&args).unwrap_or_else(|err| { 
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parssing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}