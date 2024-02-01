use fltk::{
    app, 
    button::{Button, self, RadioButton, ToggleButton, RoundButton, LightButton, CheckButton, RepeatButton, RadioLightButton, RadioRoundButton}, 
    frame::{Frame, self}, prelude::*, window::Window, group::Flex, 
    enums::{Mode, self}, 
    image::{Pixmap, self, SvgImage, PngImage}};
use fltk::{enums::*, prelude::*, *};

// FLTK支持矢量图和位图，提供下面几种开箱即用的图像类型：

// BmpImage
// JpegImage
// GifImage
// PngImage
// SvgImage
// Pixmap
// RgbImage
// XpmImage
// XbmImage
// PnmImage

// Bmp图像
// JpegImage（Jpeg图像）
// Gif图像
// Png图像
// Svg图像
// 像素映射
// Rgb图像
// Xpm图像
// Xbm图像
// Pnm图像

// 它还提供了两个helper types：

// SharedImage：它包装了上述所有的类型，使用时不需要提供图像的类型。
// TiledImage：它提供了任何具体类型的平铺图像（Tiled Image）。


pub fn lmages_test() {
    // let app = app::App::default().with_scheme(app::Scheme::Gleam);
    // let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");

    // let mut frame = Frame::default().with_size(360, 260).center_of(&wind);
    // frame.set_frame(FrameType::EngravedBox);
    // let mut image = PngImage::load("src/20230428142735.png").unwrap();
    // image.scale(200, 200, true, true);
    // frame.set_image(Some(image));

    // wind.make_resizable(true);
    // wind.end();
    // wind.show();

    // app.run().unwrap();

    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");

    let mut frame = Frame::default().with_size(360, 260).center_of(&wind);
    frame.set_frame(FrameType::EngravedBox);
    let mut image = PngImage::load("src/20230428142735.png").unwrap();
    frame.draw(move |f| {
        image.scale(f.w(), f.h(), true, true);
        image.draw(f.x() + 40, f.y(), f.w(), f.h());
    });

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}