use android_glue;
use opengles_graphics::glyph_cache::*;
use opengles_graphics::GlGraphics;
use piston::input::*;

mod background;
mod gamestate;
mod util;
mod window_info;

use self::gamestate::GameState;
use self::window_info::WindowInfoCache;

pub struct App<'a> {
    gl: GlGraphics,
    window_info: WindowInfoCache,
    event_handler: std::sync::mpsc::Receiver<android_glue::Event>,
    focus: bool,
    cache: GlyphCache<'a>,
    game_data: GameState,
}

impl<'a> App<'a> {
    pub fn new(sender: std::sync::mpsc::Receiver<android_glue::Event>) -> Self {
        Self {
            gl: GlGraphics::new(opengles_graphics::OpenGL::V3_1),
            window_info: WindowInfoCache::new(),
            event_handler: sender,
            focus: true,
            cache: GlyphCache::from_bytes(include_bytes!("../../fonts/Ubuntu-R.ttf")).unwrap(),
            game_data: GameState::initial(),
        }
    }

    fn on_size_change(&mut self) {
        self.window_info.reset();
        self.window_info.no_moves = 0;
        self.game_data
            .size_change(&mut self.window_info, &mut self.cache);
    }

    pub fn draw(&mut self, args: &RenderArgs) {
        use graphics::*;
        let mut size_change: bool = false;
        //If we try to draw without focus, then sometimes we try to draw before the eglContext has time to
        //load, causing a few problems. It's set in `fn handle` below
        if self.focus {
            //we can't borrow self as mutable more than once, or borrow as immutable at the same time
            //so we just make a bunch of references to self's data and pass those into the closure
            let winfo_ref = &mut self.window_info;
            let cache_ref = &mut self.cache;
            let gdata_ref = &mut self.game_data;
            self.gl.draw(args.viewport(), |c, gl| {
                clear([0., 0., 0., 1.], gl);
                if winfo_ref.window_size != (args.width as usize, args.height as usize) {
                    size_change = true;
                    winfo_ref.window_size = (args.width as usize, args.height as usize);
                } else {
                    let transformed = c
                        .transform
                        .trans(winfo_ref.gridoffsets.0, winfo_ref.gridoffsets.1);
                    background::background_draw(&c, transformed, gl, winfo_ref);
                    gdata_ref.draw(&c, transformed.trans(2., 2.), gl, cache_ref, winfo_ref);
                }
            });
        }

        if size_change {
            self.on_size_change();
        }
    }

    fn signal_pause_change(&mut self) {
        self.game_data.pause();
    }

    pub fn update(&mut self, _: &UpdateArgs) {
        self.window_info.frame += 1;
        self.game_data
            .update(&mut self.window_info, &mut self.cache);
        while let Ok(i) = self.event_handler.try_recv() {
            self.handle(i);
        }
    }

    fn handle(&mut self, event: android_glue::Event) {
        use android_glue::Event;
        match event {
            Event::EventMotion(motion) => self.game_data.handle(motion, &mut self.window_info),
            Event::LostFocus => {
                self.signal_pause_change();
                self.focus = false;
            }
            Event::GainedFocus => {
                self.focus = true;
                self.window_info.frame = 0;
            }
            _ => {}
        }
    }
}
