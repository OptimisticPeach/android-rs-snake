mod playerstate;
mod pause_screen;

use graphics::math::Matrix2d;
use graphics::*;
use crate::app::*;

pub use self::playerstate::*;

pub struct GameState<Data: PlayerState>{
    player_state: Data,
    is_paused: bool
}

impl<Data: PlayerState> GameState<Data> {
    pub fn initial() -> Self {
        Self{
            player_state: OnePlayer::new(),
            is_paused: false
        }
    }
    pub fn draw<G: Graphics, T: graphics::character::CharacterCache<Texture=G::Texture>>(
        &mut self,
        c: &Context,
        transform: Matrix2d,
        g: &mut G,
        cache: &mut T,
        winfo: &mut window_info::WindowInfoCache,
    ) {
        self.player_state.draw(c, transform, g, cache, winfo);
        if self.is_paused {
            pause_screen::draw_pause(c, g, winfo);
        }
    }

    pub fn update(&mut self, winfo: &mut window_info::WindowInfoCache, cache: &mut impl graphics::character::CharacterCache) {
        if !self.is_paused {
            self.player_state.update(winfo, cache);
        }
    }

    pub fn handle(&mut self, action: android_glue::MotionAction, pointer_id: i32, x: f32, y: f32, winfo: &mut window_info::WindowInfoCache){
        match action {
            android_glue::MotionAction::Up => {
                if self.is_paused {
                    self.is_paused = false;
                    //TODO:
                    //This can cause us to crash if we 
                    //pause while still holding the 
                    //finger down, and let go and then
                    //try to tap again, we start 
                    //without having finished :(
                    return;
                }
            }
            _ => {}
        }
        self.player_state.handle(action, pointer_id, x, y, winfo);
    }

    pub fn size_change(&mut self, old_w: usize, old_h: usize, winfo: &mut window_info::WindowInfoCache){
        let old_orientation = old_w < old_h;
        let new_orientation = winfo.window_size.0 < winfo.window_size.1;
        if old_orientation && !new_orientation { //we've rotated and are now landscape
            self.player_data = TwoPlayer::new();
        }
        else if !old_orientation && new_orientation {
            self.player_data = OnePlayer::new();
        }
        else {
            panic!("Can't have a window of equal size lengths...");
        }
    }

    pub fn pause(&mut self){
        self.is_paused = true;
    }
}
