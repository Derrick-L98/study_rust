fn main(){
    let num:i32 = 10;
    println!("Hello,world!");
    println!("num = {}",num);
    let n = &num;
    println!("n = {}",n);
    println!("num = {}",num);
    //let a = 13;
    print!("你好！");
    println!("n is {0}, n again is {0}", n); 
   let  a = "abc";
    //a = "NIf";//a是不可变变量，所以这里会错误
    println!("{}",a);
    let b = 10;
    let b = 20;
    println!("{}",b);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    another_function();
}

fn another_function() {
    println!("Hello, runoob!");
}