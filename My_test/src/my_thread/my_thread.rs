use std::thread;
use tokio::sync::broadcast;
use thread_id;

pub async fn thread() {
    let handler = thread::Builder::new()
    .name("named thread".into())
    .spawn(|| {
        let handle = thread::current();
        // assert_eq!(handle.name(), Some("named thread"));
        println!("{:#?}, {:#?}", handle.name(), handle.id());
        println!("spawned thread has id {}", thread_id::get());
    })
    .unwrap();

    handler.join().unwrap();
    let handle = thread::current();
    println!("{:#?}, {:#?}", handle.name(), handle.id());
    println!("main thread has id {}", thread_id::get());
}



pub async fn th() {
    let (tx1, mut rx1) = broadcast::channel::<&str>(5);
    let (tx2, mut rx2) = broadcast::channel::<&str>(5);

    tokio::spawn(async move {
        let sec = std::time::Duration::from_secs(8);
        loop {
            let _ = tx1.send("one");
            thread::sleep(sec);
        }
    });

    tokio::spawn(async move {
        let sec = std::time::Duration::from_secs(1);
        loop {
            thread::sleep(sec);
            let _ = tx2.send("two");
        }
    });

    loop {
        tokio::select! {
            val = rx1.recv() => {
                tokio::spawn(async move {
                    println!(">>>>>>>>>>>>>>rx1 begin first!>>>>>>>>>>>>>>");
                    let five = std::time::Duration::from_secs(15);
                    thread::sleep(five);
                    println!("<<<<<<<<<<<<<<rx1 completed first with {:?}<<<<<<<<<<<<<<", val)
                });
            }
            val = rx2.recv()  => {
                println!("rx2 completed first with {:?}", val);
            }
        }
    }
}