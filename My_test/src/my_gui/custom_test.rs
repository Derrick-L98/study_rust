use fltk::{prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;

struct MyCustomButton {
    inner: widget::Widget,
    num_clicks: Rc<RefCell<i32>>,
}


impl MyCustomButton {

    pub fn new(radius: i32, label: &str) -> Self {
        let mut inner = widget::Widget::default()
            .with_size(radius * 2, radius * 2)
            .with_label(label)
            .center_of_parent();
        inner.set_frame(enums::FrameType::OFlatBox);
        let num_clicks = 0;
        let num_clicks = Rc::from(RefCell::from(num_clicks));
        let clicks = num_clicks.clone();
        inner.draw(|i| { 
            // 我们需要一个绘制的方法
            draw::draw_box(i.frame(), i.x(), i.y(), i.w(), i.h(), i.color());
            draw::set_draw_color(enums::Color::Black); // 设置文字颜色
            draw::set_font(enums::Font::Helvetica, app::font_size());
            draw::draw_text2(&i.label(), i.x(), i.y(), i.w(), i.h(), i.align());
        });
        inner.handle(move |i, ev| match ev {
            enums::Event::Push => {
                *clicks.borrow_mut() += 1;
                // 使 num_clicks 在点击时递增
                i.do_callback(); 
                // 执行我们使用 set_callback() 设置的回调方法
                true
            }
            _ => false,
        });
        Self {
            inner,
            num_clicks,
        }
    }

    // 获得我们的按钮被点击的次数
    pub fn num_clicks(&self) -> i32 {
        *self.num_clicks.borrow()
    }
}

// 通过宏扩展widget::Widget
widget_extends!(MyCustomButton, widget::Widget, inner);


pub fn custom_test() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    // 设置背景为白色
    app::background(255, 255, 255);
    let mut wind = window::Window::new(100, 100, 400, 300, "Hello from rust");
    let mut btn = MyCustomButton::new(50, "Click");
    btn.set_color(enums::Color::Cyan);
    btn.set_callback(|_| println!("Clicked"));
    wind.end();
    wind.show();

    app.run().unwrap();
    
    // 打印我们的按钮被点击的数字，退出
    println!("Our button was clicked {} times", btn.num_clicks());
}