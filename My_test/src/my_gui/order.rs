use fltk::{
    app, 
    button::{Button, self, RadioButton, ToggleButton, RoundButton, LightButton, CheckButton, RepeatButton, RadioLightButton, RadioRoundButton}, 
    frame::{Frame, self}, prelude::*, window::Window, group::Flex, 
    enums::{Mode, self}, 
    image::{Pixmap, self}};
use fltk::{enums::*, prelude::*, *};




pub fn order() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(800, 700);
    let mut pack = group::Pack::default_fill();
    pack.set_spacing(1);//
    
    // let mut scroll = group::Scroll::default_fill().with_type(group::ScrollType::BothAlways);
    frame::Frame::default().with_size(0, 40).with_label(&format!("订单号"));
    input::Input::default().with_size(0, 40);
    frame::Frame::default().with_size(0, 40).with_label(&format!("成交数量"));
    input::Input::default().with_size(0, 40);
    frame::Frame::default().with_size(0, 40).with_label(&format!("成交价格"));
    input::Input::default().with_size(0, 40);
    frame::Frame::default().with_size(0, 40).with_label(&format!("成交号"));
    input::Input::default().with_size(0, 40);
    frame::Frame::default().with_size(0, 40).with_label(&format!("通道Id"));
    input::Input::default().with_size(0, 40);
    // frame::Frame::default().with_size(0, 40).with_label(&format!("通道类型"));
    // input::Input::default().with_size(0, 40).set_value("1");
    // frame::Frame::default().with_size(0, 40).with_label(&format!("交易方向"));
    // input::Input::default().with_size(0, 40).set_value("1");
    frame::Frame::default().with_size(0, 40).with_label(&format!("通道类型"));
    let mut channel_type_choice = menu::Choice::default().with_size(0, 40).center_of_parent().with_label("通道类型");
    channel_type_choice.add_choice("内盘");
    channel_type_choice.add_choice("外盘");
    channel_type_choice.set_value(1);
    frame::Frame::default().with_size(0, 40).with_label(&format!("交易方向"));
    let mut order_direction_choice = menu::Choice::default().with_size(0, 40).center_of_parent().with_label("交易方向");
    // order_direction_choice.add_choice("买");
    // order_direction_choice.add_choice("卖");
    // order_direction_choice.set_value(1);
    //2方法
    order_direction_choice.add_choice("买|卖");
    order_direction_choice.set_value(0);

    frame::Frame::default().with_size(0, 40); // 占位
    button::Button::default().with_size(0, 40).with_label("提交");















    wind.end();
    wind.show();
    app.run().unwrap();
}

pub fn menus() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(800, 700);
    let mut menubar = menu::MenuBar::new(0, 0, 60, 40, "rew");
    menubar.add("菜单/下单\t", Shortcut::None, menu::MenuFlag::Normal, menu_cb);
    menubar.add("菜单/撤单\t", Shortcut::None, menu::MenuFlag::Normal, menu_cb,);
    let idx = menubar.add("菜单/处理",Shortcut::None,menu::MenuFlag::Submenu, menu_cb,);
    menubar.add("菜单/处理/确认\t",Shortcut::None,menu::MenuFlag::Normal, menu_cb,);
    menubar.add("菜单/处理/成交\t",Shortcut::None,menu::MenuFlag::Normal, menu_cb,);
    menubar.add("菜单/处理/废单\t", Shortcut::None,menu::MenuFlag::Normal, menu_cb,);
    menubar.add("菜单/退出\t", Shortcut::None, menu::MenuFlag::Normal, menu_cb,);

    // let mut clear = button::Button::new(160, 250, 80, 30, "Clear");
    win.end();
    win.show();

    // clear.set_callback(move |_| {
    //     menubar.clear_submenu(idx).unwrap();
    // });

    a.run().unwrap();
}


fn menu_cb(m: &mut impl MenuExt) {
    if let Some(choice) = m.choice() {
        match choice.as_str() {
            "下单\t" => println!("下单"),
            "撤单\t" => println!("撤单"),
            "成交" => println!("成交"),
            "退出\t" => {
                println!("退出");
                app::quit();
            },
            _ => println!("{}", choice),
        }
    }
}