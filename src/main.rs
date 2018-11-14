extern crate android_glue;
extern crate glutin;
extern crate glutin_window;
extern crate graphics;
extern crate opengles_graphics;
extern crate shader_version as sv;
extern crate piston;
extern crate window;
extern crate shader_version;

mod app;
use glutin::GlContext;
use glutin_window::GlutinWindow;
use piston::*;
use piston::window::WindowSettings;

fn main() {
    use std::env;

    let key = "RUST_BACKTRACE";
    env::set_var(key, "1");
    let mut android_window: GlutinWindow = WindowSettings::new("Glutin Window", (640, 480))
        .fullscreen(false)
        .opengl(shader_version::OpenGL::V2_0)
        .build() 
        .unwrap();
    opengles_graphics::gl::load_with(|s| {
        android_window.window.get_proc_address(s) as *const std::ffi::c_void
    });

    let (sender, receiver) = std::sync::mpsc::channel();
    android_glue::add_sender(sender);
    let mut app = app::App::new(receiver);

    let mut events = piston::event_loop::Events::new(piston::event_loop::EventSettings::new());
    while let Some(e) = events.next(&mut android_window) { 
        match e {
            input::Event::Loop(loopargs) => match loopargs {
                input::Loop::Render(rargs) => {
                    app.draw(&rargs);
                }
                input::Loop::Update(uargs) => {
                    app.update(&uargs);
                }
                _ => {}
            },
            input::Event::Input(..) => {}
            input::Event::Custom(..) => {}
        }
    }
}
