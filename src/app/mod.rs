use android_base::*;
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
    window_info: WindowInfoCache,
    cache: GlyphCache<'a>,
    game_data: GameState,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self {
            window_info: WindowInfoCache::new(),
            cache: GlyphCache::from_bytes(include_bytes!("../../fonts/Ubuntu-R.ttf"), opengles_graphics::TextureSettings::new()).unwrap(),
            game_data: GameState::initial(),
        }
    }
}

impl AppImpl for App<'_> {
    fn on_size_change(&mut self, new_size: &(usize, usize), old_size: &(usize, usize)) {
        self.window_info.window_size = (*new_size).clone();
        self.window_info.reset();
        self.window_info.no_moves = 0;
        self.game_data
            .size_change(&mut self.window_info, &mut self.cache);
    }

    fn draw(&mut self, c: Context, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;
        clear([0., 0., 0., 1.], gl);
        let transformed = c
            .transform
            .trans(self.window_info.gridoffsets.0, self.window_info.gridoffsets.1);
        background::background_draw(&c, transformed, gl, &mut self.window_info);
        self.game_data.draw(&c, transformed.trans(2., 2.), gl, &mut self.cache, &mut self.window_info);
    }

    fn signal_pause_change(&mut self) {
        self.game_data.pause();
    }

    fn update(&mut self, _: &UpdateArgs) {
        self.window_info.frame += 1;
        self.game_data
            .update(&mut self.window_info, &mut self.cache);
    }

    fn motion(&mut self, motion: android_glue::Motion) {
        self.game_data.handle(motion, &mut self.window_info);
    }

    fn cancel_poll(&self) -> bool {
        false
    }
}
