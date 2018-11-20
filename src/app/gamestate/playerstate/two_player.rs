use graphics::math::Matrix2d;
use graphics::*;

use crate::app::*;

pub struct TwoPlayer {}

impl TwoPlayer {
    pub fn new() -> Self {
        TwoPlayer {}
    }
}

use super::PlayerState;

impl PlayerState for TwoPlayer{
    fn update(&mut self, winfo: &mut window_info::WindowInfoCache, cache: &mut impl graphics::character::CharacterCache){}
    fn draw<G: Graphics, T: graphics::character::CharacterCache<Texture=G::Texture>>(
        &mut self,
        c: &Context,
        transform: Matrix2d,
        g: &mut G,
        cache: &mut T,
        winfo: &mut window_info::WindowInfoCache,
    ){}
    fn handle(&mut self, action: android_glue::MotionAction, pointer_id: i32, x: f32, y: f32, winfo: &mut window_info::WindowInfoCache){}
}
