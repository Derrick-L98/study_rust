//指定订阅的数据


// use std::collections::HashMap;

// type EventCallback = Box<dyn Fn()>;

// struct EventEmitter {
//     /// 事件-监听器数组容器
//     events: HashMap<String, Vec<EventCallback>>,
// }

// impl EventEmitter {
//     fn new() -> EventEmitter {
//         EventEmitter {
//             events: HashMap::new(),
//         }
//     }

// 	/// 添加事件监听器，监听器是一个回调函数，表示用户订阅的具体服务
//     fn add_listener(&mut self, event: &str, callback: EventCallback) {
//         let callbacks = self.events.entry(event.to_string()).or_insert(Vec::new());
//         callbacks.push(callback);
//     }

// 	/// 移除事件监听器：相当于用户取消订阅
//     fn remove_listener(&mut self, event: &str, callback: &EventCallback) {
//         if let Some(callbacks) = self.events.get_mut(event) {
//             callbacks.retain(|cb| cb() != callback());
//         }
//     }

// 	/// 触发事件：相当于发布消息或服务，也就是事件发生时，将订阅者订阅的服务一一为订阅者执行
//     fn emit(&self, event: &str) {
//         if let Some(callbacks) = self.events.get(event) {
//             for callback in callbacks {
//                 callback();
//             }
//         }
//     }
// }


// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};

// type EventCallback = Arc<dyn Fn() + Send + Sync>;

// pub struct EventEmitter {
//     events: Mutex<HashMap<String, Vec<EventCallback>>>,
//     max_listeners: usize,
// }

// impl EventEmitter {
//     pub fn new() -> Self {
//         EventEmitter {
//             events: Mutex::new(HashMap::new()),
//             max_listeners: usize::MAX,
//         }
//     }

//     /// 设置最大监听器数量
//     /// Set the maximum number of listeners
//     pub fn set_max_listeners(&mut self, max_listeners: usize) {
//         self.max_listeners = max_listeners;
//     }

//     /// 获取最大监听器数量
//     pub fn get_max_listeners(&self) -> usize {
//         self.max_listeners
//     }

//     /// 添加事件监听器
//     pub fn add_listener(&self, event: &str, callback: EventCallback) {
//         let mut events = self.events.lock().unwrap();
//         let callbacks = events.entry(event.to_string()).or_insert(Vec::new());
//         callbacks.push(callback);
//     }


// 	/// 移除事件监听器
//     pub fn remove_listener(&self, event: &str, callback: &EventCallback) {
//         let mut events = self.events.lock().unwrap();
//         if let Some(callbacks) = events.get_mut(event) {
//             callbacks.retain(|cb| !Arc::ptr_eq(cb, callback));
//         }
//     }
//     /// 触发事件
//     pub fn emit(&self, event: &str) {
//         let events = self.events.lock().unwrap();
//         if let Some(callbacks) = events.get(event) {
//             for callback in callbacks {
//                 let callback_clone = callback.clone();
//                 // Spawn a new thread to run each callback asynchronously
//                 std::thread::spawn(move || {
//                     (*callback_clone)();
//                 });
//             }
//         }
//     }
// }

use std::sync::{Arc};
use EventEmitter::EventEmitter;

pub fn sock2() {
    let emitter = EventEmitter::new();

    let callback1 = Arc::new(|| println!("[event1 emitted]: The first callback of event1 has been called."));
    let callback2 = Arc::new(|| println!("[event1 emitted]: The second callback of event1 has been called."));
    let callback3 = Arc::new(|| println!("[event2 emitted]: The only one callbask of event2 has been called."));

    // Add event listener
    emitter.on("event1", callback1);
    emitter.on("event1", callback2);
    emitter.on("event2", callback3);

    let ct1 = emitter.listener_count("event1");
    let ct2 = emitter.listener_count("event2");
    println!("Number of Listeners for event1 is: {ct1}, \nNumber of Listeners for event2 is: {ct2}");

    emitter.emit("event1"); // Emit event1
    emitter.emit("event2"); // Emit event1
}
