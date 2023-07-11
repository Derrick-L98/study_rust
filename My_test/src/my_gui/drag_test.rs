use fltk::{
    app, 
    button::{Button, self, RadioButton, ToggleButton, RoundButton, LightButton, CheckButton, RepeatButton, RadioLightButton, RadioRoundButton}, 
    frame::{Frame, self}, prelude::*, window::Window, group::{Flex, Pack}, 
    enums::{Mode, self}, 
    image::{Pixmap, self, SvgImage, PngImage}};
use fltk::{enums::*, prelude::*, *};


pub fn drag_test() {
    drag_file();
}


pub fn drag() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 400);
    wind.set_color(enums::Color::Red);
    wind.set_border(false);
    wind.make_resizable(true);
    wind.end();
    wind.show();

    wind.handle({
        let mut x = 0;
        let mut y = 0;
        move |w, ev| match ev {
            enums::Event::Push => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                true
            }
            enums::Event::Drag => {
                w.set_pos(app::event_x_root() - x, app::event_y_root() - y);
                true
            }
            _ => false,
        }
    });

    app.run().unwrap();
}


pub fn drag_file() {
    let app = app::App::default();
    let buf = text::TextBuffer::default();//包装文本缓冲区，克隆文本缓冲区会使基础指针失效，因此无派生（克隆）
    let mut wind = window::Window::default().with_size(400, 400);
    // let mut disp = text::TextDisplay::default_fill();//不可编辑的文本显示组件
    let mut disp = text::TextEditor::default_fill();//可编辑文本显示组件
    wind.make_resizable(true);//窗口缩放
    wind.end();
    wind.show();

    disp.set_buffer(buf.clone());//设置关联的TextBuffer
    disp.handle({
        let mut dnd = false;
        let mut released = false;
        let buf = buf.clone();
        move |_, ev| match ev {//定义FLTK捕获的事件类型
            Event::DndEnter => {//输入
                dnd = true;
                true
            }
            Event::DndDrag => true,
            Event::DndRelease => {
                released = true;
                true
            }
            Event::Paste => {//粘贴
                if dnd && released {
                    let path = app::event_text();
                    let path = path.trim();
                    let path = path.replace("file://", "");
                    let path = std::path::PathBuf::from(&path);
                    if path.exists() {
                        // 我们使用 timeout 来避免路径传入缓冲区
                        app::add_timeout3(0.0, {
                            let mut buf = buf.clone();
                            move |_| {
                                buf.load_file(&path).unwrap();
                            }
                        });
                    }
                    dnd = false;
                    released = false;
                    true
                } else {
                    false
                }
            }
            Event::DndLeave => {
                dnd = false;
                released = false;
                true
            }
            _ => false,
        }
    });
    app.run().unwrap();

    // 如果你对文件的内容不感兴趣，你可以只取得文件的路径并显示：
    // let app = app::App::default();
    // let buf = text::TextBuffer::default();
    // let mut wind = window::Window::default().with_size(400, 400);
    // let mut disp = text::TextDisplay::default_fill();
    // wind.end();
    // wind.show();

    // disp.set_buffer(buf.clone());
    // disp.handle({
    //     let mut dnd = false;
    //     let mut released = false;
    //     let mut buf = buf.clone();
    //     move |_, ev| match ev {
    //         Event::DndEnter => {
    //             dnd = true;
    //             true
    //         }
    //         Event::DndDrag => true,
    //         Event::DndRelease => {
    //             released = true;
    //             true
    //         }
    //         Event::Paste => {
    //             if dnd && released {
    //                 let path = app::event_text();
    //                 buf.append(&path);
    //                 dnd = false;
    //                 released = false;
    //                 true
    //             } else {
    //                 false
    //             }
    //         }
    //         Event::DndLeave => {
    //             dnd = false;
    //             released = false;
    //             true
    //         }
    //         _ => false,
    //     }
    // });
    // app.run().unwrap();
}