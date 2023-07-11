use fltk::{prelude::*, *};
use fltk::{
    app, button,
    enums::{Color, Font, FrameType},
    frame, group, input,
    window,
};

pub fn file_test() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 300);

    let mut btn = button::Button::default()
        .with_size(80, 30)
        .with_label("Select file")
        .center_of_parent();

    wind.end();
    wind.show();

    btn.set_callback(|_| {
        let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
        // dialog.set_filter("*.{txt,rs,toml}");//添加过滤器来限制文件类型,让对话框只能选取.txt、.rs和.toml文件。
        dialog.show();
        println!("{:?}", dialog.filename());
    });

    app.run().unwrap();
}

pub fn my_fltk_file() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(800, 700);

    let mut btn = button::Button::default()
        .with_size(100, 30)
        .with_label("Select file")
        .center_of_parent();

    wind.end();
    wind.show();

    // btn.set_callback(|_| {
    //     let mut dialog = dialog::FileChooser::new(
    //         ".",                            /*对话框弹出时所在目录*/
    //         "*.{txt,rs,toml}",              /*文件类型限定*/
    //         dialog::FileChooserType::Multi, /*对话框类型*/
    //         "Select file:",                 /*对话框标题*/
    //     );
    //     dialog.show();
    //     while dialog.shown() {
    //         app::wait();
    //     }
    //     if dialog.count() > 1 {
    //         for i in 1..=dialog.count() { // values start at 1
    //             println!(" VALUE[{}]: '{}'", i, dialog.value(i).unwrap());
    //         }
    //     }
    // });


    btn.set_callback(|_| {
        let file = dialog::file_chooser(
            "Choose File",
            "*.rs",
            /*start dir*/ ".",
            /*relative*/ true,
        );
        if let Some(file) = file {
            println!("{}", file);
        }
    });

    app.run().unwrap();
}



//这些函数都有相对的变体，一个有_default()后缀，不需要设置坐标，另一个没有后缀，需要手动输入坐标。 有些对话框会返回一个值，
// 比如choice，input 和 password。input和password返回用户输入的文本，而choice则返回选择值的索引
pub fn help_dialog_test() {
    // let app = app::App::default();
    // let mut wind = window::Window::default().with_size(400, 300);

    // let mut btn = button::Button::default()
    //     .with_size(100, 30)
    //     .with_label("Show dialog")
    //     .center_of_parent();

    // wind.end();
    // wind.show();

    // btn.set_callback(|_| {
    //     let mut help = dialog::HelpDialog::new(100, 100, 400, 300);
    //     help.set_value("<h2>Hello world</h2>"); // this takes html
    //     help.show();
    //     while help.shown() {
    //         app::wait();
    //     }
    // });

    // app.run().unwrap();


    // let app = app::App::default();
    // let mut wind = window::Window::default().with_size(400, 300);

    // let mut btn = button::Button::default()
    //     .with_size(100, 30)
    //     .with_label("Show dialog")
    //     .center_of_parent();

    // wind.end();
    // wind.show();

    // btn.set_callback(|_| {
    //     // dialog::message_default("Message");
    //     dialog::message(100, 100, "Message");//将在默认位置（大致在鼠标位置）显示一个message对话框。如果你想手动设置显示的坐标，可以使用message()函数：
    // });

    // app.run().unwrap();


    let app = app::App::default();
    dialog::message_title_default("My App!");//设置对话框标题
    let mut wind = window::Window::default().with_size(400, 300);

    let mut btn = button::Button::default()
        .with_size(100, 30)
        .with_label("Show dialog")
        .center_of_parent();

    wind.end();
    wind.show();

    //选择No将打印0，Yes将打印1，Cancel将打印2
    btn.set_callback(|_| {
        let choice = dialog::choice_default("Would you like to save", "No", "Yes", "Cancel");
        println!("{}", choice);
    });

    app.run().unwrap();



}

pub fn input_password_test() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 300);

    let mut btn = button::Button::default()
        .with_size(100, 30)
        .with_label("Show dialog")
        .center_of_parent();

    wind.end();
    wind.show();

    btn.set_callback(|_| {
        // password() 和 input() 需要第二个参数来表示默认显示的值
        let pass = dialog::password_default("Enter password:", "");
        if let Some(pass) = pass {
            println!("{}", pass);
        }
    });

    app.run().unwrap();
}














//自定义对话框

fn style_button(btn: &mut button::Button) {
    btn.set_color(Color::Cyan);
    btn.set_frame(FrameType::RFlatBox);
    btn.clear_visible_focus();
}

pub fn show_dialog() -> MyDialog {
    MyDialog::default()
}

pub struct MyDialog {
    inp: input::Input,
}

impl MyDialog {
    pub fn default() -> Self {
        let mut win = window::Window::default()
            .with_size(400, 100)
            .with_label("My Dialog");
        win.set_color(Color::from_rgb(240, 240, 240));
        let mut pack = group::Pack::default()
            .with_size(300, 30)
            .center_of_parent()
            .with_type(group::PackType::Horizontal);
        pack.set_spacing(20);
        frame::Frame::default()
            .with_size(80, 0)
            .with_label("Enter name:");
        let mut inp = input::Input::default().with_size(100, 0);
        inp.set_frame(FrameType::FlatBox);
        let mut ok = button::Button::default().with_size(80, 0).with_label("Ok");
        style_button(&mut ok);
        pack.end();
        win.end();
        win.make_modal(true);
        win.show();
        ok.set_callback({
            let mut win = win.clone();
            move |_| {
                win.hide();
            }
        });
        while win.shown() {
            app::wait();
        }
        Self { inp }
    }
    pub fn value(&self) -> String {
        self.inp.value()
    }
}

pub fn custom_dialog_test() {
    let a = app::App::default();
    app::set_font(Font::Times);
    let mut win = window::Window::default().with_size(600, 400);
    win.set_color(Color::from_rgb(240, 240, 240));
    let mut btn = button::Button::default()
        .with_size(80, 30)
        .with_label("Click")
        .center_of_parent();
    style_button(&mut btn);
    let mut frame = frame::Frame::new(btn.x() - 40, btn.y() - 100, btn.w() + 80, 30, None);
    frame.set_frame(FrameType::BorderBox);
    frame.set_color(Color::Red.inactive());
    win.end();
    win.show();
    btn.set_callback(move |_| {
        let d = show_dialog();
        frame.set_label(&d.value());
    });
    a.run().unwrap();
}


pub fn print_test() {//打印对话框， 会调用你的系统平台的本地打印机对话框：
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 300);
    let mut but = button::Button::default()
    .with_size(80, 30)
    .with_label("Click")
    .center_of_parent();

    wind.make_resizable(true);
    wind.end();
    wind.show();
    but.set_callback(|widget| {
        let mut printer = printer::Printer::default();
        if printer.begin_job(1).is_ok() {
            printer.begin_page().ok();
            let (width, height) = printer.printable_rect();
            draw::set_draw_color(enums::Color::Black);
            draw::set_line_style(draw::LineStyle::Solid, 2);
            draw::draw_rect(0, 0, width, height);
            draw::set_font(enums::Font::Courier, 12);
            printer.set_origin(width / 2, height / 2);
            printer.print_widget(widget, -widget.width() / 2, -widget.height() / 2);
            printer.end_page().ok();
            printer.end_job();
        }
    });

    app.run().unwrap();
}