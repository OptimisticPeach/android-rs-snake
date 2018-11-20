use graphics::math::Matrix2d;
use graphics::*;
use crate::app::*;

mod one_player;
mod two_player;
pub mod snake;

pub use self::one_player::*;
pub use self::two_player::*;

pub trait PlayerState{
    fn update(&mut self, winfo: &mut window_info::WindowInfoCache, cache: &mut impl graphics::character::CharacterCache);
    fn draw<G: Graphics, T: graphics::character::CharacterCache<Texture=G::Texture>>(
        &mut self,
        c: &Context,
        transform: Matrix2d,
        g: &mut G,
        cache: &mut T,
        winfo: &mut window_info::WindowInfoCache,
    );
    fn handle(&mut self, action: android_glue::MotionAction, pointer_id: i32, x: f32, y: f32, winfo: &mut window_info::WindowInfoCache);
}