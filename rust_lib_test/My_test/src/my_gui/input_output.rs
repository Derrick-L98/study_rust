use fltk::{
    app, 
    button::{Button, self, RadioButton, ToggleButton, RoundButton, LightButton, CheckButton, RepeatButton, RadioLightButton, RadioRoundButton}, 
    frame::{Frame, self}, prelude::*, window::Window, group::Flex, 
    enums::{Mode, self}, 
    image::{Pixmap, self}};
use fltk::{enums::*, prelude::*, *};


pub fn input_test() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let flex = group::Flex::default().with_size(100, 100).column().center_of_parent();
    let label = frame::Frame::default().with_label("Enter age");
    let input = input::IntInput::default();
    let mut btn = button::Button::default().with_label("Submit");
    flex.end();
    win.end();
    win.show();

    btn.set_callback(move |btn| {
        println!("your age is {}", input.value());
    });

    a.run().unwrap();
}

pub fn output_test() {

    // let a = app::App::default();
    // let mut win = window::Window::default().with_size(400, 300);
    // let flex = group::Flex::default().with_size(200, 50).column().center_of_parent();
    // let label = frame::Frame::default().with_label("Check this text:");
    // let mut output = output::Output::default();
    // output.set_value("You can't edit this!");
    // flex.end();
    // win.end();
    // win.show();
    // a.run().unwrap();

    // 使用InputExt::set_readonly(bool)方法将Input组件设置为只读：

    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let flex = group::Flex::default().with_size(100, 100).column().center_of_parent();
    let label = frame::Frame::default().with_label("Enter age");
    let mut input = input::IntInput::default();
    let mut btn = button::Button::default().with_label("Submit");
    flex.end();
    win.end();
    win.show();

    btn.set_callback(move |btn| {
        println!("your age is {}", input.value());
        input.set_readonly(true);//用户输入内容并按下按钮后让文本框不可修改。
    });

    a.run().unwrap();

}
