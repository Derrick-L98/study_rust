use fltk::{
    app, 
    button::{Button, self, RadioButton, ToggleButton, RoundButton, LightButton, CheckButton, RepeatButton, RadioLightButton, RadioRoundButton}, 
    frame::{Frame, self}, prelude::*, window::Window, group::Flex, 
    enums::{Mode, self}, 
    image::{Pixmap, self}};
use fltk::{enums::*, prelude::*, *};

// Text组件实现了DisplayExt trait。FLTK提供了3个文字组件，可以在text mod中找到：
// TextDisplay
// TextEditor
// SimpleTerminal
// 显示或编辑文本

//文本框输入
pub fn text_test() {
    // let a = app::App::default();
    // let mut buf = text::TextBuffer::default();

    // let mut win = window::Window::default().with_size(400, 300);
    // let mut txt = text::TextEditor::default().with_size(390, 290).center_of_parent();
    // txt.set_buffer(buf.clone());
    // win.end();
    // win.show();

    // buf.set_text("Hello world!");
    // buf.append("\n");
    // buf.append("This is a text editor!");

    // a.run().unwrap();


    // let a = app::App::default();
    // let buf = text::TextBuffer::default();

    // let mut win = window::Window::default().with_size(400, 300);
    // let mut txt = text::TextEditor::default().with_size(390, 290).center_of_parent();
    // txt.set_buffer(buf);
    // win.end();
    // win.show();

    // let mut my_buf = txt.buffer().unwrap();

    // my_buf.set_text("Hello world!");//set_text()来设置Buffer的内容
    // my_buf.append("\n");
    // my_buf.append("This is a text editor!");//可以用append()来添加文本

    // a.run().unwrap();


    let a = app::App::default();
    let mut buf = text::TextBuffer::default();
    buf.set_text("Hello world!");
    buf.append("\n");
    buf.append("This is a text editor!");

    let mut win = window::Window::default().with_size(400, 300);
    let mut txt = text::TextDisplay::default().with_size(390, 290).center_of_parent();
    txt.set_buffer(buf);
    // 设置换行模式
    // 不同于 AtPixel 和 AtColumn, AtBounds不需要第二个参数
    // AtBounds 会设置文本到达输入框边界便会自动换行，对于大小可变的窗口很好用。
    // DisplayExt定义了很多管理文本属性的方法，例如可以设置何时换行，光标位置，字体，颜色，大小等。
    txt.wrap_mode(text::WrapMode::AtBounds, 0);
    txt.set_text_color(Color::Red);
    win.end();
    win.show();

    a.run().unwrap();
}


const STYLES: &[text::StyleTableEntry] = &[
    text::StyleTableEntry {
        color: Color::Green,
        font: Font::Courier,
        size: 16,
    },
    text::StyleTableEntry {
        color: Color::Red,
        font: Font::Courier,
        size: 16,
    },
    text::StyleTableEntry {
        color: Color::from_u32(0x8000ff),
        font: Font::Courier,
        size: 16,
    },
];


pub fn style_test() {
    let a = app::App::default();
    let mut buf = text::TextBuffer::default();
    let mut sbuf = text::TextBuffer::default();
    buf.set_text("Hello world!");
    // A是样式表中的第一个元素的索引，这里为“Hello world!”的每个字母应用A代表的样式
    sbuf.set_text(&"A".repeat("Hello world!".len())); 
    buf.append("\n"); 
    // 虽然针对换行的样式可能并没有显示出来，但是这里还需要将其写上，以免弄乱之后的文字样式
    sbuf.append("B"); 
    buf.append("This is a text editor!");
    sbuf.append(&"C".repeat("This is a text editor!".len()));

    let mut win = window::Window::default().with_size(400, 300);
    let mut txt = text::TextDisplay::default()
        .with_size(390, 290)
        .center_of_parent();
    txt.set_buffer(buf);
    txt.set_highlight_data(sbuf, STYLES.to_vec());
    win.end();
    win.show();

    a.run().unwrap();
}