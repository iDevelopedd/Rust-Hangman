#![allow(non_snake_case)]
#![allow(unused)]

use fltk::{prelude::*, app, window::Window, frame::Frame, button::Button};


fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 500, 500, "Hangman");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(190, 410, 80, 40, "Click me!");
    wind.end();
    wind.show();
    but.set_callback(move |_| frame.set_label("Hello World!"));
    app.run().unwrap();
}
