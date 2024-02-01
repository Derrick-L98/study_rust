// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel();
    
//     thread::spawn(move|| {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     });
    
//     let received = rx.recv().unwrap();
//     println!("Got: {}",received);
    
// }
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let tx1 = mpsc::Sender::clone(&tx);
//     thread::spawn(move|| {
//         let vals = vec![
//         	String::from("1: hi"),
//             String::from("1: from"),
//             String::from("1: the"),
//             String::from("1: thread"),
//         ];
//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_millis(200));
//         }
//     });
//     thread::spawn(move|| {
//         let vals = vec![
//         	String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_millis(200));
//         }
//     });
//     for received in rx {
//     	println!("Got: {}",received);
//     }
// }
//---------------------------------------多线程共享mutex
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