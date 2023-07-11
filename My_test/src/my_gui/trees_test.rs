use fltk::{
    app, 
    button::{Button, self, RadioButton, ToggleButton, RoundButton, LightButton, CheckButton, RepeatButton, RadioLightButton, RadioRoundButton}, 
    frame::{Frame, self}, prelude::*, window::Window, group::Flex, 
    enums::{Mode, self}, 
    image::{Pixmap, self}};
use fltk::{enums::*, prelude::*, *};


pub fn trees_test() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let mut tree = tree::Tree::default().with_size(390, 290).center_of_parent();
    tree.add("Item 1");
    tree.add("Item 2");
    tree.add("Item 3");
    win.end();
    win.show();

    a.run().unwrap();
}

// 你会发现树的根标签总是 "ROOT "。可以通过set_root_label()方法来设置根标
// 还能调用set_show_root(false)方法来隐藏根标签。
pub fn many_tress_test() {
    // let a = app::App::default();
    // let mut win = window::Window::default().with_size(400, 300);
    // let mut tree = tree::Tree::default().with_size(390, 290).center_of_parent();
    // tree.set_root_label("My Tree");
    // tree.set_select_mode(tree::TreeSelect::Multi);
    // tree.set_connector_style(tree::TreeConnectorStyle::Solid);
    // tree.set_connector_color(enums::Color::Red.inactive());
    // tree.add("Item 1");
    // tree.add("Item 2");
    // tree.add("Item 3");
    // tree.add("Item 3/Subitem 1");
    // tree.add("Item 3/Subitem 2");
    // tree.add("Item 3/Subitem 3");
    // win.end();
    // win.show();

    // tree.set_callback(|t| {//可以使用first_selected_item()方法来获取被点击到的元素：
    //     if let Some(item) = t.first_selected_item() {
    //         println!("{} selected", item.label().unwrap());
    //     }
    // });

    // a.run().unwrap();

    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let mut tree = tree::Tree::default().with_size(390, 290).center_of_parent();
    tree.set_select_mode(tree::TreeSelect::Multi);
    tree.set_connector_style(tree::TreeConnectorStyle::Solid);
    tree.set_connector_color(enums::Color::Red.inactive());
    tree.set_show_root(false);
    tree.add("Item 1");
    tree.add("Item 2");
    tree.add("Item 3");
    tree.add("Item 3/Subitem 1");
    tree.add("Item 3/Subitem 2");
    tree.add("Item 3/Subitem 3");
    win.end();
    win.show();

    
    tree.set_callback(|t| {
        if let Some(items) = t.get_selected_items() {
            for i in items {
                println!("{} selected", t.item_pathname(&i).unwrap());
            }
        }
    });

    a.run().unwrap();











}