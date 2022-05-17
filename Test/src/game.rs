use std::io;//导入标准库


fn main(){

    println!("猜数！");
    println!("猜测一个数！");
    
    let mut guess = String::new();//声明一个可以修改的空的字符串
    
    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜的数是：{}",guess);



}