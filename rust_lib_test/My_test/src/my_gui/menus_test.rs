use fltk::{
    app, 
    button::{Button, self, RadioButton, ToggleButton, RoundButton, LightButton, CheckButton, RepeatButton, RadioLightButton, RadioRoundButton}, 
    frame::{Frame, self}, prelude::*, window::Window, group::Flex, 
    enums::{Mode, self}, 
    image::{Pixmap, self}};
use fltk::{enums::*, prelude::*, *};


#[derive(Clone)]
enum Msg {
    New, 
    Open,
    Save,
    SaveAs, 
    Print, 
    Quit, 
    Cut,
    Copy,
    Paste,
    About,
}

pub fn menus_test() {
    // let app = app::App::default();
    // let mut wind = window::Window::default().with_size(400, 300);
    // let mut choice = menu::Choice::default().with_size(80, 30).center_of_parent().with_label("Select item");
    // choice.add_choice("Choice 1");
    // choice.add_choice("Choice 2");
    // choice.add_choice("Choice 3");
    // // 也可以直接输入 choice.add_choice("Choice 1|Choice 2|Choice 3");
    // wind.end();
    // wind.show();

    // choice.set_callback(|c| {
    //     match c.value() {
    //         0 => println!("choice 1 selected"),
    //         1 => println!("choice 2 selected"),
    //         2 => println!("choice 3 selected"),
    //         _ => unreachable!(),
    //     }
    // });

    // app.run().unwrap();



    // let app = app::App::default();
    // let mut wind = window::Window::default().with_size(400, 300);
    // let mut choice = menu::Choice::default()
    //     .with_size(80, 30)
    //     .center_of_parent();

    // choice.add(
    //     "Choice 1",
    //     enums::Shortcut::None,
    //     menu::MenuFlag::Normal,
    //     |_| println!("choice 1 selected"),
    //     );
    // choice.add(
    //     "Choice 2",
    //     enums::Shortcut::None,
    //     menu::MenuFlag::Normal,
    //     |_| println!("choice 2 selected"),
    //     );
    // choice.add(
    //     "Choice 3",
    //     enums::Shortcut::None,
    //     menu::MenuFlag::Normal,
    //     |_| println!("choice 3 selected"),
    //     );

    // wind.end();
    // wind.show();

    // app.run().unwrap();









//使用add_emit()方法来传递一个sender和一个message
    let app: app::App = app::App::default();
    let mut win = window::Window::default().with_size(800, 700);
    let mut menu = menu::SysMenuBar::default().with_size(800, 35);
    let (s, r) = app::channel();
    menu.set_frame(FrameType::FlatBox);
    menu.add_emit(
        "&File/New...\t",
        Shortcut::Ctrl | 'n',
        menu::MenuFlag::Normal,
        s.clone(),
        Msg::New,
    );

    menu.add_emit(
        "&File/Open...\t",
        Shortcut::Ctrl | 'o',
        menu::MenuFlag::Normal,
        s.clone(),
        Msg::Open,
    );

    menu.add_emit(
        "&File/Save\t",
        Shortcut::Ctrl | 's',
        menu::MenuFlag::Normal,
        s.clone(),
        Msg::Save,
    );

    menu.add_emit(
        "&File/Save as...\t",
        Shortcut::Ctrl | 'w',
        menu::MenuFlag::Normal,
        s.clone(),
        Msg::SaveAs,
    );

    menu.add_emit(
        "&File/Print...\t",
        Shortcut::Ctrl | 'p',
        menu::MenuFlag::MenuDivider,
        s.clone(),
        Msg::Print,
    );

    menu.add_emit(
        "&File/Quit\t",
        Shortcut::Ctrl | 'q',
        menu::MenuFlag::Normal,
        s.clone(),
        Msg::Quit,
    );

    menu.add_emit(
        "&Edit/Cut\t",
        Shortcut::Ctrl | 'x',
        menu::MenuFlag::Normal,
        s.clone(),
        Msg::Cut,
    );

    menu.add_emit(
        "&Edit/Copy\t",
        Shortcut::Ctrl | 'c',
        menu::MenuFlag::Normal,
        s.clone(),
        Msg::Copy,
    );

    menu.add_emit(
        "&Edit/Paste\t",
        Shortcut::Ctrl | 'v',
        menu::MenuFlag::Normal,
        s.clone(),
        Msg::Paste,
    );

    menu.add_emit(
        "&Help/About\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s.clone(),
        Msg::About,
    );
    win.end();
    win.show();

    if let Some(mut item) = menu.find_item("&File/Quit\t") {//设置选项颜色
        item.set_label_color(Color::Red);
    }

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Msg::New => println!("Ctrl+N"),
                Msg::Open => println!("Ctrl+O"),
                Msg::Save => println!("Ctrl+S"),
                Msg::SaveAs => println!("Ctrl+W"),
                Msg::Print => println!("Ctrl+P"),
                Msg::Quit => println!("Ctrl+Q"),
                Msg::Cut => println!("Ctrl+X"),
                Msg::Copy => println!("Ctrl+C"),
                Msg::Paste => println!("Ctrl+V"),
                Msg::About => println!("Ctrl+"),
            }
        }
    }

}

fn menu_cb(m: &mut impl MenuExt) {
    if let Some(choice) = m.choice() {
        match choice.as_str() {
            "New\t" => println!("New"),
            "Open\t" => println!("Open"),
            "Third" => println!("Third"),
            "Quit\t" => {
                println!("Quitting");
                app::quit();
            },
            _ => println!("{}", choice),
        }
    }
}

#[derive(Clone)]
enum Message {
    Choice1,
    Choice2,
    Choice3,
}


// 菜单中通道传递消息
pub fn menus_channel_test() {
    let a = app::App::default();
    let (s, r) = app::channel();
    let mut wind = window::Window::default().with_size(400, 300);
    let mut choice = menu::Choice::default()
        .with_size(80, 30)
        .center_of_parent()
        .with_label("Select item");

    choice.add_emit(
        "Choice 1",
        enums::Shortcut::None,
        menu::MenuFlag::Normal,
        s.clone(),
        Message::Choice1,
    );
    choice.add_emit(
        "Choice 2",
        enums::Shortcut::None,
        menu::MenuFlag::Normal,
        s.clone(),
        Message::Choice2,
    );
    choice.add_emit(
        "Choice 3",
        enums::Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Choice3,
    );

    wind.end();
    wind.show();

    while a.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Choice1 => println!("choice 1 selected"),
                Message::Choice2 => println!("choice 2 selected"),
                Message::Choice3 => println!("choice 3 selected"),
            }
        }
    }
}


// 多级菜单(回调)
pub fn many_menus_test() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let mut menubar = menu::MenuBar::new(0, 0, 400, 40, "rew");
    menubar.add("File/New\t", Shortcut::None, menu::MenuFlag::Normal, menu_cb);
    menubar.add(
        "File/Open\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb,
    );
    let idx = menubar.add(
        "File/Recent",
        Shortcut::None,
        menu::MenuFlag::Submenu,
        menu_cb,
    );
    menubar.add(
        "File/Recent/First\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb,
    );
    menubar.add(
        "File/Recent/Second\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb,
    );
    menubar.add(
        "File/Quit\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb,
    );
    let mut btn1 = button::Button::new(160, 150, 80, 30, "Modify 1");
    let mut btn2 = button::Button::new(160, 200, 80, 30, "Modify 2");
    let mut clear = button::Button::new(160, 250, 80, 30, "Clear");
    win.end();
    win.show();

    btn1.set_callback({
        let menubar = menubar.clone();
        move |_| {
            if let Some(mut item) = menubar.find_item("File/Recent") {
                item.add(
                    "Recent/Third",
                    Shortcut::None,
                    menu::MenuFlag::Normal,
                    menu_cb,
                );
                item.add(
                    "Recent/Fourth",
                    Shortcut::None,
                    menu::MenuFlag::Normal,
                    menu_cb,
                );
            }
        }
    });

    btn2.set_callback({
        let mut menubar = menubar.clone();
        move |_| {
            menubar.add(
                "File/Recent/Fifth\t",
                Shortcut::None,
                menu::MenuFlag::Normal,
                menu_cb,
            );
            menubar.add(
                "File/Recent/Sixth\t",
                Shortcut::None,
                menu::MenuFlag::Normal,
                menu_cb,
            );
        }
    });

    clear.set_callback(move |_| {
        menubar.clear_submenu(idx).unwrap();
    });

    a.run().unwrap();
}