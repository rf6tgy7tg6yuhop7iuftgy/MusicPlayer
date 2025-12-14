use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window, group::Flex, enums::Align};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 1600, 900, "Music Player");
    wind.make_resizable(true);

    let mut flex = Flex::new(0, 0, 1600, 900, None);
    flex.set_type(fltk::group::FlexType::Column);
    flex.set_margin(5);

    let mut frame = Frame::default().with_label("Hello, world!");
    frame.set_align(Align::Left | Align::Inside);
    flex.fixed(&mut frame, 20);

    let mut btn = Button::default().with_label("Click me");
    btn.set_callback(move |_| {
        frame.set_label("Button clicked!");
    });
    btn.set_align(Align::Left | Align::Inside);
    flex.fixed(&mut btn, 30);

    flex.end();
    wind.end();
    wind.show();

    app.run().unwrap();
}
