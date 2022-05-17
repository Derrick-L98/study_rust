fn main() {
    let number = 3;
    if number < 5 {
        println!("条件为 true");
    } else {
        println!("条件为 false");
    }
    let a:&str = "abc";
    if a < "ab"{
        println!("条件为 true");
    }else {
        println!("条件为 false");
    }
}