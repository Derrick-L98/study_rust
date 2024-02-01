// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawmed thread!", i);
//             thread::sleep(Duration::from_millis(1));//休眠1毫秒
//         }
//     });
    
//     //handle.join().unwrap();
//     println!("+++++++++++++++++++++");
//     for i in 1..5 {
//         println!("hi number {} from the spawmed thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     handle.join().unwrap();
// }

// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel();//创建通道（多个生产者，一个消费者）
    
//     thread::spawn(move|| {
//         let val = String::from("hi,你好");
//         tx.send(val).unwrap();//发送消息，并做错误处理
//     });
    
//     let received = rx.recv().unwrap();//接收消息
//     println!("Got: {}",received);
    
// }
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     thread::spawn(move|| {
//         let vals = vec![
//         	String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//     for received in rx {
//     	println!("Got: {}",received);
//     }
// }

// use std::sync::Mutex;
// use std::thread;

// fn main() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];
    
//     for _ in 0..10 {
//     //创建10个线程，然后把handle放入vec
//         let handle = thread::spawn(move || {//这里希望把Mutex转移到闭包里，获取互斥锁，然后修改值--------(move报错：原因循环第一次已经转移所有权，第二次将无法使用)
//             let mut num = counter.lock().unwrap();
            
//             *num += 1;
//         });
//         handles.push(handle);
//     }
//     for handle in handles {
//         handle.join().unwrap();
//     }
// 	println!("Result: {}", *counter.lock().unwrap());
// }

use std::sync::{Mutex, Arc};
use std::thread;


fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
    	let counter = Arc::clone(&counter);
    //创建10个线程，然后把handle放入vec
        let handle = thread::spawn(move || {//这里希望把Mutex转移到闭包里，获取互斥锁，然后修改值 
            let mut num = counter.lock().unwrap();
            
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
	println!("Result: {}", *counter.lock().unwrap());
}

