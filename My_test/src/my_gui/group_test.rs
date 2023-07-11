use fltk::{
    app, 
    button::{Button, self, RadioButton, ToggleButton, RoundButton, LightButton, CheckButton, RepeatButton, RadioLightButton, RadioRoundButton}, 
    frame::{Frame, self}, prelude::*, window::Window, group::Flex, 
    enums::{Mode, self}, 
    image::{Pixmap, self}};
use fltk::{enums::*, prelude::*, *};

pub fn test1() {
    // let a = app::App::default();
    // let mut win = Window::default().with_size(400, 300);
    // let _btn = Button::new(160, 200, 80, 30, "Click");
    // win.end();//按钮 "_btn" 的父组件是Window。 在组控件调用end()后创建的其他组件将不被包含在该控件中，即会创建在这个组控件的外面
    // win.show();
    // a.run().unwrap();

    // let a = app::App::default();
    // let mut win = Window::default().with_size(400, 300);
    // win.end();
    // win.show();

    // let btn = Button::new(160, 200, 80, 30, "Click");
    // win.add(&btn);
    // // win.begin();//达到同样的效果
    // // let _btn = Button::new(160, 200, 80, 30, "Click");
    // // win.end();
    
    // a.run().unwrap();


    // let app = app::App::default();
    // let mut wind = window::Window::default().with_size(400, 300);
    // let mut pack = group::Pack::default_fill();
    // pack.set_spacing(5);
    // for i in 0..2 {
    //     frame::Frame::default().with_size(0, 40).with_label(&format!("field {}", i));
    //     input::Input::default().with_size(0, 40);
    // }
    // frame::Frame::default().with_size(0, 40); // 占位
    // button::Button::default().with_size(0, 40).with_label("Submit");
    // pack.end();
    // wind.end();
    // wind.show();

    // app.run().unwrap();

    let app = app::App::default();
    let mut wind = window::Window::default().with_size(300, 100);
    let mut pack = group::Pack::default_fill().with_type(group::PackType::Horizontal);//设置垂直布局
    pack.set_spacing(5);
    for i in 0..2 {
        frame::Frame::default().with_size(40, 0).with_label(&format!("field {}", i));
        input::Input::default().with_size(40, 0);
    }
    frame::Frame::default().with_size(40, 0); // 占位
    button::Button::default().with_size(40, 0).with_label("Submit");
    pack.end();
    wind.end();
    wind.show();

    app.run().unwrap();
}