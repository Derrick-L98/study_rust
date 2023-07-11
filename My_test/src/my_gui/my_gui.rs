use fltk::{
    app, 
    button::{Button, self, RadioButton, ToggleButton, RoundButton, LightButton, CheckButton, RepeatButton, RadioLightButton, RadioRoundButton}, 
    frame::{Frame, self}, prelude::*, window::Window, group::Flex, 
    enums::{Mode, self}, 
    image::{Pixmap, self}};
use fltk::{enums::*, prelude::*, *};

use super::*;

const PXM: &[&str] = &[
    "50 34 4 1",
    "  c #000000",
    "o c #ff9900",
    "@ c #ffffff",
    "# c None",
    "##################################################",
    "###      ##############################       ####",
    "### ooooo  ###########################  ooooo ####",
    "### oo  oo  #########################  oo  oo ####",
    "### oo   oo  #######################  oo   oo ####",
    "### oo    oo  #####################  oo    oo ####",
    "### oo     oo  ###################  oo     oo ####",
    "### oo      oo                     oo      oo ####",
    "### oo       oo  ooooooooooooooo  oo       oo ####",
    "### oo        ooooooooooooooooooooo        oo ####",
    "### oo     ooooooooooooooooooooooooooo    ooo ####",
    "#### oo   ooooooo ooooooooooooo ooooooo   oo #####",
    "####  oo oooooooo ooooooooooooo oooooooo oo  #####",
    "##### oo oooooooo ooooooooooooo oooooooo oo ######",
    "#####  o ooooooooooooooooooooooooooooooo o  ######",
    "###### ooooooooooooooooooooooooooooooooooo #######",
    "##### ooooooooo     ooooooooo     ooooooooo ######",
    "##### oooooooo  @@@  ooooooo  @@@  oooooooo ######",
    "##### oooooooo @@@@@ ooooooo @@@@@ oooooooo ######",
    "##### oooooooo @@@@@ ooooooo @@@@@ oooooooo ######",
    "##### oooooooo  @@@  ooooooo  @@@  oooooooo ######",
    "##### ooooooooo     ooooooooo     ooooooooo ######",
    "###### oooooooooooooo       oooooooooooooo #######",
    "###### oooooooo@@@@@@@     @@@@@@@oooooooo #######",
    "###### ooooooo@@@@@@@@@   @@@@@@@@@ooooooo #######",
    "####### ooooo@@@@@@@@@@@ @@@@@@@@@@@ooooo ########",
    "######### oo@@@@@@@@@@@@ @@@@@@@@@@@@oo ##########",
    "########## o@@@@@@ @@@@@ @@@@@ @@@@@@o ###########",
    "########### @@@@@@@     @     @@@@@@@ ############",
    "############  @@@@@@@@@@@@@@@@@@@@@  #############",
    "##############  @@@@@@@@@@@@@@@@@  ###############",
    "################    @@@@@@@@@    #################",
    "####################         #####################",
    "##################################################",
];

fn move_image(mut frm: Frame, handle: app::TimeoutHandle) {
    let (x, y) = (frm.x(), frm.y());
    frm.set_pos(x + 5, y);
    app::redraw();
    if frm.x() > 260 {
        app::remove_timeout3(handle)
    } else {
        app::repeat_timeout3(0.016, handle);
    }
}







pub async fn my_gui() {
    // test1();
    // order();
    // menus_test();
    // menus();
    // input_test();
    // output_test();
    // valuator_test();
    // text_test();
    // style_test();
    // bro_test();
    // many_tress_test();
    // table_test();
    // custom_test();
    // file_test();
    // my_fltk_file();
    // help_dialog_test();
    // custom_dialog_test();
    // print_test();
    // lmages_test();
    // events_test();
    // channel_eve_test();
    // my_even_test();
    // drag_test();
    state_test();
}


fn style_btn(btn: &mut button::Button) {
    btn.set_color(Color::from_hex(0x42A5F5));
    btn.set_selection_color(Color::from_hex(0x42A5F5));
    btn.set_frame(FrameType::FlatBox);
}

pub fn test() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    win.set_color(Color::White);
    // 我们的按钮占据了窗口的左侧
    let mut sliding_btn = button::Button::new(0, 0, 100, 300, None);
    style_btn(&mut sliding_btn);

    win.make_resizable(true);//窗口缩放
    win.end();
    win.show();

    sliding_btn.set_callback(|btn| {
        if btn.w() > 0 && btn.w() < 100 {
            return; // 绘制已经完成
        }
        while btn.w() != 0 {
            btn.set_size(btn.w() - 2, btn.h());
            app::sleep(0.016);
            btn.parent().unwrap().redraw();
            app::wait(); // 或 app::check();
        }
    });
    a.run().unwrap();
}


pub fn move_image_test() {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_label("timeout")
        .with_size(720, 486)
        .center_screen();
    let mut frame = Frame::new(-200, 150, 200, 200, "");
    let mut pxm = Pixmap::new(PXM).unwrap();
    pxm.scale(200, 200, true, true);
    frame.set_image_scaled(Some(pxm));
    wind.set_color(enums::Color::White);
    wind.end();
    wind.show_with_env_args();

    app::add_timeout3(0.016, move |handle| {
        let frame = frame.clone();
        move_image(frame, handle);
    });
    app.run().unwrap();
}

