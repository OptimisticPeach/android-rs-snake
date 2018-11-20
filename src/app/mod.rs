use android_glue;
use opengles_graphics::glyph_cache::*;
use opengles_graphics::GlGraphics;
use piston::input::*;

mod background;
mod counter;
mod gamestate;
mod touch;
mod util;
mod window_info;

use self::window_info::WindowInfoCache;
use self::gamestate::GameState;

pub struct App<'a> {
    gl: GlGraphics,
    window_info: WindowInfoCache,
    event_handler: std::sync::mpsc::Receiver<android_glue::Event>,
    paused: bool,
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
            paused: false,
            focus: true,
            cache: GlyphCache::from_bytes(include_bytes!("../../fonts/Ubuntu-R.ttf")).unwrap(),
            game_data: GameState::initial()
        }
    }

    fn on_size_change(
        &mut self,
        old_w: usize,
        old_h: usize,
    ) {
        self.window_info.reset();
        self.window_info.no_moves = 0;
        self.game_data.size_change(old_w, old_h, &mut self.window_info);
    }

    pub fn draw(&mut self, args: &RenderArgs) {
        use graphics::*;
        let mut size_change: Option<(usize, usize)> = None;
        //If we try to draw without focus, then sometimes we try to draw before the eglContext has time to
        //load, causing a few problems. It's set in `fn handle` below
        if self.focus {
            //we can't borrow self as mutable more than once, or borrow as immutable at the same time
            //so we just make a bunch of references to self's data and pass those into the closure
            let paused = self.paused;
            let winfo_ref = &mut self.window_info;
            let cache_ref = &mut self.cache;
            let gdata_ref = &mut self.game_data;
            self.gl.draw(args.viewport(), |c, gl| {
                clear([0., 0., 0., 1.], gl);
                if winfo_ref.window_size != (args.width as usize, args.height as usize) {
                    size_change = Some(winfo_ref.window_size);
                    winfo_ref.window_size = (args.width as usize, args.height as usize);
                } else {
                    let transformed = c
                        .transform
                        .trans(winfo_ref.gridoffsets.0, winfo_ref.gridoffsets.1);
                    background::background_draw(&c, transformed, gl, winfo_ref);
                    gdata_ref.player_state.draw(&c, transformed, gl, cache_ref, winfo_ref);
                }
            });
        }

        if let Some((oldw, oldh)) = size_change {
            self.on_size_change(oldw, oldh);
        }
    }

    fn signal_pause_change(&mut self, tobe: bool, data: Box<impl std::fmt::Debug>) {
        android_glue::write_log(&format!("Called Pause with {:?} to set to {}", data, tobe));
        self.paused = tobe;
    }

    pub fn update(&mut self, _: &UpdateArgs) {
        self.window_info.frame += 1;
        if self.window_info.frame % self.window_info.frames_per_move as u128 == 0 && !self.paused {
            self.game_data.player_state.update(&mut self.window_info, &mut self.cache);
        }
        while let Ok(i) = self.event_handler.try_recv() {
            self.handle(i);
        }
    }

    fn handle(&mut self, event: android_glue::Event) {
        use android_glue::{Event, Motion, MotionAction};
        match event {
            Event::EventMotion(Motion {
                action: act,
                pointer_id,
                x,
                y,
            }) => self.game_data.handle(act, pointer_id, x, y, &mut self.window_info),
            Event::LostFocus => {
                self.signal_pause_change(true, Box::new(event));
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
