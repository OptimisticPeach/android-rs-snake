use graphics::math::Matrix2d;
use graphics::*;
use super::common::Touch;
use crate::app::window_info;
use android_glue::*;

mod two_snake;

pub use self::two_snake::*;

pub struct TwoPlayer {
    snakes: SnakeDuo,
    toucha: Touch,
    touchb: Touch,
    handled_move_a: bool,
    handled_move_b: bool
}

impl TwoPlayer {
    pub fn new() -> Self {
        TwoPlayer {
            snakes: SnakeDuo::new(4, 1, 1),
            toucha: Touch::new(),
            touchb: Touch::new(),
            handled_move_a: false,
            handled_move_b: false
        }
    }
    pub fn update(&mut self, winfo: &mut window_info::WindowInfoCache, cache: &mut impl graphics::character::CharacterCache) {
        match self.snakes.step(winfo, cache){
            WinCase::FirstSnake => {
                self.snakes = SnakeDuo::new(4, 1, 1);
                self.snakes.reset_apple(winfo);
            }
            WinCase::SecondSnake => {
                self.snakes = SnakeDuo::new(4, 1, 1);
                self.snakes.reset_apple(winfo);
            }
            _ => {}
        }
        self.handled_move_a = false;
        self.handled_move_b = false;
    }

    pub fn draw<G: Graphics>(
        &mut self,
        c: &Context,
        transform: Matrix2d,
        g: &mut G,
        cache: &mut impl graphics::character::CharacterCache<Texture=G::Texture>,
        winfo: &mut window_info::WindowInfoCache,
    ) {
        self.snakes.draw(c, transform, g, winfo, cache);
    }

    pub fn handle(
        &mut self,
        action: android_glue::Motion,
        winfo: &mut window_info::WindowInfoCache,
    ) {
        let android_glue::Motion{action, pointer_id, x, y} = action;
        match action {
            MotionAction::Down => {
                if x < (winfo.window_size.0 / 2) as f32{
                    self.toucha.start(x, y, pointer_id as usize);
                } else {
                    self.touchb.start(x, y, pointer_id as usize);
                }
            }
            MotionAction::Move => {}
            MotionAction::Up => {
                if let Some(pid) = self.toucha.id {
                    if pointer_id as usize == pid {
                        let angle = self.toucha.end(x, y, true).unwrap();
                        if !self.handled_move_a {
                            self.snakes.snakes.0.dir = super::common::Direction::get_dir(angle, self.snakes.snakes.0.dir);
                            self.handled_move_a = true;
                            winfo.no_moves += 1;
                        }
                        return;
                    }
                } 
                if let Some(pid) = self.touchb.id {
                    if pointer_id as usize == pid {
                        let angle = self.touchb.end(x, y, true).unwrap();
                        if !self.handled_move_b {
                            self.snakes.snakes.1.dir = super::common::Direction::get_dir(angle, self.snakes.snakes.1.dir);
                            self.handled_move_b = true;
                            winfo.no_moves += 1;
                        }
                        return;
                    }
                }
            }
            MotionAction::Cancel => {
                if let Some(pid) = self.toucha.id {
                    if pointer_id as usize == pid {
                        self.toucha.cancel();
                    }
                } 
                if let Some(pid) = self.touchb.id {
                    if pointer_id as usize == pid {
                        self.touchb.cancel();
                    }
                } 
            }
        }
    }

    pub fn initialize(&mut self, winfo: &mut window_info::WindowInfoCache, cache: &mut impl graphics::character::CharacterCache) {
        self.snakes.reset_apple(winfo);
        self.snakes.counters.0.set_num(0, winfo, cache, 1);
        self.snakes.counters.1.set_num(0, winfo, cache, 2);
    }
}
