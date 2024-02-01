use fltk::{
    app, 
    button::{Button, self, RadioButton, ToggleButton, RoundButton, LightButton, CheckButton, RepeatButton, RadioLightButton, RadioRoundButton}, 
    frame::{Frame, self}, prelude::*, window::Window, group::Flex, 
    enums::{Mode, self}, 
    image::{Pixmap, self}};
use fltk::{enums::*, prelude::*, *};


// '#'表示强制进行规则的缩放，因此可以避免组件的形状被扭曲。
// +[1-9]或-[1-9]可以改变缩放比例。
// '$'是水平翻转符号，'%'是垂直翻转符号。
// [0-9] - 旋转45度的倍数。是'5'和'6'时不会发生旋转，而其他数字则会使其指向数字键盘上那个键的方向。
// '0xxxx'，0后有四个数字表示角度，会使其按该度数旋转。