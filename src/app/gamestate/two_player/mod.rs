use graphics::math::Matrix2d;
use graphics::*;

pub struct TwoPlayer {}

impl TwoPlayer {
    pub fn new() -> Self {
        TwoPlayer {}
    }
    pub fn update(&mut self) {}

    pub fn draw<G: Graphics>(
        &mut self,
        _: &Context,
        _: Matrix2d,
        _: &mut G,
        _: &crate::app::window_info::WindowInfoCache,
    ) {}
}
