use fltk::{
    app, 
    button::{Button, self, RadioButton, ToggleButton, RoundButton, LightButton, CheckButton, RepeatButton, RadioLightButton, RadioRoundButton}, 
    frame::{Frame, self}, prelude::*, window::Window, group::{Flex, Pack}, 
    enums::{Mode, self}, 
    image::{Pixmap, self, SvgImage, PngImage}};
use fltk::{enums::*, prelude::*, *};

use std::sync::Mutex;


// 处理事件的方法主要是使用回调（Callback）。但是我们可以根据具体的使用情况选择其他方法，FLTK提供的处理事件的方式有这几种：

// set_callback()方法，在点击按钮时自动触发。
// handle()方法，用于进行细粒度的事件处理。
// emit()方法，接收一个sender和一个message将触发的事件类型发送，之后在event loop中处理事件。
// 我们还可以自定义一个可以在另一个组件的处理方法中被处理的事件


// 你的按钮何时执行回调方法，点击时？还是鼠标松开时？你需要设置触发器来决定何时执行回调，set_callback()方法会设置默认的触发器，
// 不同组件的触发器可能不同。例如按钮组件的触发器便是，当它具有鼠标焦点时的点击或按下回车。 
// 某个组件的触发器是可以通过set_trigger()方法改变的。改变按钮的触发方式可能没有意义，但是对于Input组建来说，
// 触发器可以被设置为CallbackTrigger::Changed，这可以使Input组件在状态改变时就触发回调：
pub fn events_test() {
    // let app = app::App::default();
    // let mut my_window = window::Window::new(100, 100, 400, 300, "My Window");
    // let mut but = button::Button::new(160, 200, 80, 40, "Click me!");
    // my_window.end();
    // my_window.show();
    // but.set_callback(|_| println!("The button was clicked!"));
    // app.run().unwrap();

    // let app = app::App::default();
    // let mut my_window = window::Window::new(100, 100, 400, 300, "My Window");
    // let mut but = button::Button::new(160, 200, 80, 40, "Click me!");
    // my_window.end();
    // my_window.show();
    // but.set_callback(|b| b.set_label("Clicked!"));
    // app.run().unwrap();


    // let a = app::App::default();
    // let mut win = window::Window::default().with_size(400, 300);
    // let mut inp = input::Input::default()
    //     .with_size(160, 30)
    //     .center_of_parent();
    // win.end();
    // win.show();
    // inp.set_trigger(enums::CallbackTrigger::Changed);//改变时就触发回调
    // inp.set_callback(|i| println!("{}", i.value()));
    // a.run().unwrap();

    // let app = app::App::default();
    // let mut my_window = window::Window::new(100, 100, 400, 300, "My Window");
    // let mut but = button::Button::new(160, 200, 80, 40, "Click me!");
    // my_window.end();
    // my_window.show();
    // but.set_callback(move |_| {
    //     my_window.set_label("button was pressed");
    // });
    // app.run().unwrap();


    // let app = app::App::default();
    // let mut my_window = window::Window::new(100, 100, 400, 300, "My Window");
    // let mut but = button::Button::new(160, 200, 80, 40, "Click me!");
    // my_window.end();
    // my_window.show();
    // but.set_callback(button_cb1);
    // app.run().unwrap();


    // let app = app::App::default();
    // let mut my_window = window::Window::new(100, 100, 400, 300, "My Window");
    // let mut but = button::Button::new(160, 200, 80, 40, "Increment!");
    // my_window.end();
    // my_window.show();
    // but.set_callback(button_cb);
    // app.run().unwrap();





    let app = app::App::default();
    let mut my_window = window::Window::new(100, 100, 400, 300, "My Window");
    let mut but = button::Button::new(160, 200, 80, 40, "Click me!");
    my_window.end();
    my_window.show();

    // but.handle(|_, event| {//handle方法接收一个有事件参数的闭包，并在处理后返回一个bool。这个返回值让FLTK知道该事件是否被处理。 它的使用是这样的
    //     println!("The event: {:?}", event);
    //     false
    // });
    but.handle(|_, event| match event {
        Event::Push => {
            println!("I was pushed!");
            true
        },
        _ => false,
    });
    app.run().unwrap();

}


fn button_cb1(w: &mut impl WidgetExt) {
    w.set_label("Clicked");
}


#[derive(Default)]
struct State {
    count: i32,
}

impl State {
    fn increment(&mut self) {
        self.count += 1;
    }
}

lazy_static::lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::default());
}


fn button_cb(_w: &mut button::Button) {
    let mut state = STATE.lock().unwrap();
    state.increment();
}



// 这允许我们创建Channel和Sender Receiver的结构，在触发后发送Message（Message必须是Send + Sync），
// 并在event loop中处理。这样做的好处是，当我们需要将我们的一些量传递到闭包或线程中时，不必再使用智能指针来包装它们。

pub fn channel_eve_test() {
    let app = app::App::default();
    
    let mut my_window = window::Window::new(100, 100, 400, 300, "My Window");
    let mut but = button::Button::new(160, 200, 80, 40, "Click me!");
    my_window.end();
    my_window.show();

    let (s, r) = app::channel();
    
    but.emit(s, true);
    // 相当于 but.set_callback(move |_| s.send(true))

//下面方法效果相同
    // while app.wait() {
    //     if let Some(msg) = r.recv() {
    //         match msg {
    //             true => println!("Clicked"),
    //             false => (), // 什么都不做
    //         }
    //     }
    // }

//Messages 可以在event loop中被接受，另外你也可以在后台线程或app::add_idle()的回调中接收Message
// 这里也不限于使用fltk channel，你可以使用任何channel。例如，这个例子用使用了std channel：
    app::add_idle(move || {
        if let Some(msg) = r.recv() {
            match msg {
                true => println!("Clicked"),
                false => (), // 这里不做任何事
            }
        }
    });
    app.run().unwrap();
}

// 类似于emit()方法，你也可以定义一个适用于所有组件的send()方法，：
use std::sync::mpsc::Sender;

pub trait SenderWidget<W, T>
where
    W: WidgetExt,
    T: Send + Sync + Clone + 'static,
{
    fn send(&mut self, sender: Sender<T>, msg: T);
}

impl<W, T> SenderWidget<W, T> for W
where
    W: WidgetExt,
    T: Send + Sync + Clone + 'static,
{
    fn send(&mut self, sender: Sender<T>, msg: T) {
        self.set_callback(move |_| {
            sender.send(msg.clone()).unwrap();
        });
    }
}

// fn main() {
//     let btn = button::Button::default();
//     let (s, r) = std::sync::mpsc::channel::<Message>();
//     btn.send(s.clone(), Message::SomeMessage);
// }









// 创建自己的事件
// FLTK在enums::Event中预先定义了29个事件。我们还可以使用调用app::handle(impl Into<i32>, window)创建我们自己的事件。
// handle函数以任意一个大于30的i32类型值作为信号标识，最好提前定义好信号标识。我们可以在另一个组件的handle()方法中处理事件，
// 注意这个组件需要放在传递给app::handle的那个窗口内部。 在下面的例子中，我们创建了一个带有Frame和Button的窗口。
// Button的回调函数在执行时，通过app::handle_main函数发送一个CHANGED事件。
// 该CHANGED信号在Frame的handle方法中被接收到并做出处理：

use std::cell::RefCell;
use std::rc::Rc;


pub struct MyEvent;

impl MyEvent {
    const CHANGED: i32 = 40;
}

#[derive(Clone)]
pub struct Counter {
    count: Rc<RefCell<i32>>,
}

impl Counter {
    pub fn new(val: i32) -> Self {
        Counter {
            count: Rc::from(RefCell::from(val)),
        }
    }

    pub fn increment(&mut self) {
        *self.count.borrow_mut() += 1;
        app::handle_main(MyEvent::CHANGED).unwrap();
    }

    pub fn decrement(&mut self) {
        *self.count.borrow_mut() -= 1;
        app::handle_main(MyEvent::CHANGED).unwrap();
    }

    pub fn value(&self) -> i32 {
        *self.count.borrow()
    }
}


pub fn my_even_test()-> Result<(), Box<dyn std::error::Error>>{
    let app = app::App::default();
    let counter = Counter::new(0);
    let mut wind = Window::default().with_size(160, 200).with_label("Counter");
    let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
    pack.set_spacing(10);
    let mut but_inc = Button::default().with_size(0, 40).with_label("+");
    let mut frame = Frame::default()
        .with_size(0, 40)
        .with_label(&counter.clone().value().to_string());
    let mut but_dec = Button::default().with_size(0, 40).with_label("-");
    pack.end();
    wind.end();
    wind.show();

    but_inc.set_callback({
        let mut c = counter.clone();
        move |_| c.increment()
    });

    but_dec.set_callback({
        let mut c = counter.clone();
        move |_| c.decrement()
    });
    
    frame.handle(move |f, ev| {
        if ev == MyEvent::CHANGED.into() {
            f.set_label(&counter.clone().value().to_string());
            true
        } else {
            false
        }
    });

    Ok(app.run()?)
}

// 优点
// 无开销。
// 信号的处理方式与其他任何FLTK事件一样。
// app::handle函数可以返回一个bool，表示该事件是否被处理。
// 允许在事件循环之外处理自定义信号/事件。
// 允许在程序中使用MVC或SVU架构。
// 缺点
// 信号只能在一个组件的处理方法中处理。
// 该信号在事件循环中是不可访问的（为解决，可以使用WidgetExt::emit或本节前面部分描述的channel）。