use fltk::{
    app, 
    button::{Button, self, RadioButton, ToggleButton, RoundButton, LightButton, CheckButton, RepeatButton, RadioLightButton, RadioRoundButton}, 
    frame::{Frame, self}, prelude::*, window::Window, group::{Flex, Pack}, 
    enums::{Mode, self}, 
    image::{Pixmap, self, SvgImage, PngImage}};
use fltk::{enums::*, prelude::*, *};


#[derive(Copy, Clone)]
enum Message {
    Inc,
    Dec,
}

struct MyApp {
    app: app::App,
    main_win: window::Window,
    frame: frame::Frame,
    count: i32,
    receiver: app::Receiver<Message>,
}

impl MyApp {
    pub fn new() -> Self {
        let count = 0;
        let app = app::App::default();
        let (s, receiver) = app::channel();
        let mut main_win = window::Window::default().with_size(400, 300);
        let col = group::Flex::default()
            .with_size(100, 200)
            .column()
            .center_of_parent();//分组小部件
        let mut inc = button::Button::default().with_label("+");//创建普通按钮
        inc.emit(s, Message::Inc);//使用发送方在回调时发出消息
        let frame = frame::Frame::default().with_label(&count.to_string());//创建一个新框架
        let mut dec = button::Button::default().with_label("-");
        dec.emit(s, Message::Dec);
        col.end();
        main_win.end();
        main_win.show();
        Self {
            app,
            main_win,
            frame,
            count,
            receiver,
        }
    }

    pub fn run(mut self) {
        while self.app.wait() {
            if let Some(msg) = self.receiver.recv() {
                match msg {
                    Message::Inc => self.count += 1,
                    Message::Dec => self.count -= 1,
                }
                self.frame.set_label(&self.count.to_string());
            }
        }
    }
}

pub fn state_test() {
    let a = MyApp::new();
    a.run();
}