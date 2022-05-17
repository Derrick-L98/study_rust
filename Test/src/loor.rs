// fn main() {
//     let s = ['R', 'U', 'N', 'O', 'O', 'B'];
//     let mut i = 0;
//     loop {
//         let ch = s[i];
//         if ch == 'O' {
//             break;
//         }
//         println!("\'{}\'", ch);
//         i += 1;
//     }
//     let s1 = String::from("hello");
//     let s2 = s1;//将s1的值移动给s2，s1将被释放，不可用
//     println!("{}", s2);

//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("s1 = {}, s2 = {}", s1, s2);

//     let s1 = String::from("hello");
//     let s2 = &s1;
//     println!("s1 is {}, s2 is {}", s1, s2);
// }
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }
}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 40, height: 20 };
    println!("{}", rect1.wider(&rect2));
}