use fltk::{
    app, 
    button::{Button, self, RadioButton, ToggleButton, RoundButton, LightButton, CheckButton, RepeatButton, RadioLightButton, RadioRoundButton}, 
    frame::{Frame, self}, prelude::*, window::Window, group::Flex, 
    enums::{Mode, self}, 
    image::{Pixmap, self}};
use fltk::{enums::*, prelude::*, *};



//估值器
pub fn valuator_test() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let mut slider = valuator::HorNiceSlider::default().with_size(400, 20).center_of_parent();
    slider.set_minimum(0.);
    slider.set_maximum(100.);
    slider.set_step(1., 1); // 设置步长为10
    slider.set_value(50.); // 设置开始位置
    win.end();
    win.show();

    slider.set_callback(|s| {
        println!("slider at {}", s.value());
    });
    a.run().unwrap();
}