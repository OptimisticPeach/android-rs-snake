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
        c: &Context,
        transform: Matrix2d,
        g: &mut G,
        winfo: &crate::app::window_info::WindowInfoCache,
    ) {}
}
