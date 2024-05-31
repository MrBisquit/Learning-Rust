// https://docs.rs/fltk/latest/fltk/index.html

use enums::Color;
//use fltk::{app::*, button::*, frame::*, prelude::{GroupExt, WidgetBase, WidgetExt}, window::*};
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, widget_themes, ThemeType, WidgetTheme};
use input::Input;

fn main() {
    println!("Starting calculator...");

    /*let app = App::default();
    let mut wind = Window::new(100, 100, 400, 600, "Calculator");
    let mut frame = Frame::new(0, 0, 400, 600, "Calculator");
    let mut button = Button::new(160, 210, 80, 40, "Click me! :)");

    wind.end();
    wind.show();

    //button.set_callback(move || frame.set_label("Hello, World!"));

    app.run().unwrap();*/

    /*let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(400, 300);
    let mut flex = group::Flex::new(0, 0, 400, 300, None);
    flex.set_type(group::FlexType::Column);
    let expanding = button::Button::default().with_label("Expanding");
    let normal = button::Button::default().with_label("Normal");
    flex.fixed(&normal, 30);
    flex.end();
    win.end();
    win.show();
    a.run().unwrap();*/

    let app = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::Dark);
    widget_theme.apply();
    let mut wind = window::Window::default()
        .with_size(300, 300) // Old: 450
        .center_screen()
        .with_label("Calculator");
    let mut input = Input::new(5, 5, 290, 25, ""); // Old Title: "Output"

    // (295 - (5 * 4)) / 4 = 68.75 = 69
    // Func keys
    let mut hpack = group::Pack::default().with_size(295, 60).with_pos(5, 35);
    hpack.set_type(group::PackType::Horizontal);
    hpack.set_spacing(5);
    let _but_perc = button::Button::default().with_size(69, 0).with_label("%").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_sqrt = button::Button::default().with_size(69, 0).with_label("√").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_clr = button::Button::default().with_size(69, 0).with_label("AC").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_del = button::Button::default().with_size(69, 0).with_label("DEL").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    hpack.end();

    // 35 + 60 + 5 = 100
    // Top row + Divide
    let mut hpack = group::Pack::default().with_size(295, 60).with_pos(5, 100);
    hpack.set_type(group::PackType::Horizontal);
    hpack.set_spacing(5);
    let _but_7 = button::Button::default().with_size(69, 0).with_label("7").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_8 = button::Button::default().with_size(69, 0).with_label("8").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_9 = button::Button::default().with_size(69, 0).with_label("9").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_div = button::Button::default().with_size(69, 0).with_label("÷").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    hpack.end();

    // 100 + 60 + 5 = 165
    // Top Middle row + Times
    let mut hpack = group::Pack::default().with_size(295, 60).with_pos(5, 165);
    hpack.set_type(group::PackType::Horizontal);
    hpack.set_spacing(5);
    let _but_4 = button::Button::default().with_size(69, 0).with_label("4").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_5 = button::Button::default().with_size(69, 0).with_label("5").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_6 = button::Button::default().with_size(69, 0).with_label("6").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_tim = button::Button::default().with_size(69, 0).with_label("×").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    hpack.end();

    // 165 + 60 + 5 = 230
    // Bottom Middle row + Takeaway
    let mut hpack = group::Pack::default().with_size(295, 60).with_pos(5, 230);
    hpack.set_type(group::PackType::Horizontal);
    hpack.set_spacing(5);
    let _but_1 = button::Button::default().with_size(69, 0).with_label("1").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_2 = button::Button::default().with_size(69, 0).with_label("2").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_3 = button::Button::default().with_size(69, 0).with_label("3").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_tak = button::Button::default().with_size(69, 0).with_label("-").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    hpack.end();

    // 230 + 60 + 5 = 295
    // Bottom row + Decimal + Equals + Add
    let mut hpack = group::Pack::default().with_size(295, 60).with_pos(5, 230);
    hpack.set_type(group::PackType::Horizontal);
    hpack.set_spacing(5);
    let _but_dec = button::Button::default().with_size(69, 0).with_label(".").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_0 = button::Button::default().with_size(69, 0).with_label("0").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_equ = button::Button::default().with_size(69, 0).with_label("=").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let _but_add = button::Button::default().with_size(69, 0).with_label("+").set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    hpack.end();
    
    wind.end();
    wind.show();
    app.run().unwrap();
}