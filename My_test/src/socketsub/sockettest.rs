use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Subscriber {
    name: String,
}

struct Publisher {
    //订阅者名称 ?? 改记录时间
    //订阅者想要订阅的数据
    //订阅什么事件 ---- 对应的消息
    subscribers: HashSet<Subscriber>,
    name: String,
}

impl Publisher {
    fn new(name: &str) -> Publisher {
        Publisher {
            subscribers: HashSet::new(),
            name: name.to_string(),
        }
    }

    fn add_subscriber(&mut self, subscriber: &Subscriber) {
        self.subscribers.insert(subscriber.clone());
    }

    fn remove_subscriber(&mut self, subscriber: &Subscriber) {
        self.subscribers.remove(subscriber);
        println!("\n=> {} 已取消订阅。\n", subscriber.name);
    }

    fn notify_all(&self, arg: &str) {
        for subscriber in &self.subscribers {
            subscriber.notify(self, arg);
        }
    }
}

impl Subscriber {
    fn new(name: &str) -> Subscriber {
        Subscriber {
            name: name.to_string(),
        }
    }

    fn notify(&self, publisher: &Publisher, arg: &str) {
        println!(
            "\"{}\"（订阅者） 收到的通知来自 \"{}\"（发布者）的通知: {}",
            self.name, publisher.name, arg
        );
    }

    fn update(&self) {

    }
}

pub fn sock() {
    let mut publisher = Publisher::new("政务服务中心");
    let jack_lee = Subscriber::new("jackLee");
    let jack_ma = Subscriber::new("jackMa");

    publisher.add_subscriber(&jack_lee);
    publisher.add_subscriber(&jack_ma);

    println!("------- 第一次发布消息 -------");
    publisher.notify_all("[通知] 恢复证件办理!");

    // 用户 jackMa 取消订阅
    publisher.remove_subscriber(&jack_ma);

    println!("------- 第二次发布消息 -------");
    publisher.notify_all("[通知] 恢复证件办理!");
}
