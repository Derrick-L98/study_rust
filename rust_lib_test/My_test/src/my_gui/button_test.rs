use fltk::{
    app, 
    button::{Button, self, RadioButton, ToggleButton, RoundButton, LightButton, CheckButton, RepeatButton, RadioLightButton, RadioRoundButton}, 
    frame::{Frame, self}, prelude::*, window::Window, group::Flex, 
    enums::{Mode, self}, 
    image::{Pixmap, self}};
use fltk::{enums::*, prelude::*, *};


pub fn RadioRoundButton_test() {
    // let a = app::App::default();
    // let mut win = window::Window::default().with_size(400, 300);
    // let flex = group::Flex::default().with_size(100, 200).column().center_of_parent();
    // // 用户同一时间只能选中一个按钮，选中后，另一个会被取消选中
    // let btn1 = button::RadioRoundButton::default().with_label("Option 1");
    // let btn2 = button::RadioRoundButton::default().with_label("Option 2"); 
    // flex.end();
    // win.end();
    // win.show();
    // a.run().unwrap();


    // let a = app::App::default();
    // let mut win = window::Window::default().with_size(400, 300);
    // let flex = group::Flex::default().with_size(100, 200).column().center_of_parent();
    // let btn1 = button::CheckButton::default().with_label("Option 1");
    // let btn2 = button::CheckButton::default().with_label("Option 2");
    // let mut btn3 = button::Button::default().with_label("Submit");
    // flex.end();
    // win.end();
    // win.show();

    // btn3.set_callback(move |btn3| {
    //     if btn1.value() {
    //         println!("btn1 is checked");
    //     }
    //     if btn2.value() {
    //         println!("btn2 is checked");
    //     }
    // });
    // a.run().unwrap();


//默认选中一个按钮
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let flex = group::Flex::default().with_size(100, 200).column().center_of_parent();
    let mut btn1 = button::CheckButton::default().with_label("Option 1");
    btn1.set_value(true);
    // 同样可以使用 btn1.set_checked(true)
    let btn2 = button::CheckButton::default().with_label("Option 2");
    let mut btn3 = button::Button::default().with_label("Submit");
    flex.end();
    win.end();
    win.show();

    btn3.set_callback(move |btn3| {
        if btn1.value() {
            println!("btn1 is checked");
        }
        if btn2.value() {
            println!("btn2 is checked");
        }
    });

    a.run().unwrap();
}

pub fn wind_test() {
    let app = app::App::default().with_scheme(app::Scheme::Oxy).load_system_fonts();
    let mut wind = Window::new(200, 200, 400, 300, "手动成交APP");
    wind.end();
    wind.show();
    app.run().unwrap();
}

pub fn button_test() {
    let app = app::App::default();
    let mut my_window = window::Window::new(600, 300, 600, 500, "My Window");
    // 创建普通按钮
    let mut but = Button::new(260, 400, 80, 40, "Ok!");
    let mut frame = Frame::new(260, 200, 80, 40, "");
    // // 创建一个单选按钮。无线电，意味着在同一组中只能切换一个
    // let a = RadioButton::new(50, 50, 80, 40, "a!");
    // // 创建切换按钮
    // let b = ToggleButton::new(100, 100, 80, 40, "b!");
    // // 创建圆形按钮
    // let c = RoundButton::new(150, 150, 80, 40, "c!");
    // // 创建复选按钮
    // let d = CheckButton::new(200, 200, 80, 40, "d!");
    // // 创建灯光按钮
    // let e = LightButton::new(250, 250, 80, 40, "e!");
    // // 创建重复按钮
    // let f = RepeatButton::new(300, 300, 80, 40, "f!");
    // // 创建一个单选按钮。无线电，意味着在同一组中只能切换一个
    // let g = RadioLightButton::new(350, 350, 80, 40, "g!");
    // // 创建一个单选圆形按钮。无线电，意味着在同一组中只能切换一个
    // let h = RadioRoundButton::new(400, 400, 80, 40, "h!");

    my_window.end();
    my_window.show();
    but.set_callback(move |_| frame.set_label("Hello World!")); // the closure capture is mutable borrow to our button
    app.run().unwrap();
}


