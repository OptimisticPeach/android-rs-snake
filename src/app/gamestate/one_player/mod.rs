mod snake;

pub use self::snake::*;
use super::common::Touch;
use android_glue::*;
use crate::app::*;
use graphics::math::Matrix2d;
use graphics::*;
use super::common::*;

pub struct OnePlayer {
    handled_move: bool,
    touch_handler: Touch,
    count: Counter,
    snake: Snake,
}

impl OnePlayer {
    pub fn new() -> Self {
        OnePlayer {
            handled_move: false,
            touch_handler: Touch::new(),
            count: Counter::new(0., 1),
            snake: Snake::new(4, 1, 1)
        }
    }

    pub fn draw<G: Graphics, T: graphics::character::CharacterCache<Texture = G::Texture>>(
        &mut self,
        c: &Context,
        transform: Matrix2d,
        g: &mut G,
        cache: &mut T,
        winfo: &mut window_info::WindowInfoCache,
        tri_cache: &mut TriangleCache
    ) {
        self.snake.calc_draw(&c, g, transform, winfo, tri_cache);
        self.count.draw(&c, cache, g);
    }

    pub fn update(
        &mut self,
        winfo: &mut window_info::WindowInfoCache,
        cache: &mut impl graphics::character::CharacterCache,
    ) {
        self.tick(winfo, cache);
    }

    fn on_dead(
        &mut self,
        winfo: &mut window_info::WindowInfoCache,
        cache: &mut impl graphics::character::CharacterCache,
    ) {
        self.snake.reset(&winfo);
        winfo.no_moves = 0;
        self.count
            .set_num((self.snake.body.len() - 3) / 3, winfo, cache, 1);
    }

    fn tick(
        &mut self,
        winfo: &mut window_info::WindowInfoCache,
        cache: &mut impl graphics::character::CharacterCache,
    ) {
        self.handled_move = false;
        let prev_len = self.snake.body.len() - 3;
        let mut dead = false;
        if !self.snake.step(winfo) {
            dead = true;
            self.on_dead(winfo, cache);
        }
        if self.snake.body.len() - 3 != prev_len && !dead {
            self.count.set_num(prev_len / 3 + 1, winfo, cache, 1);
        }
    }

    pub fn handle(
        &mut self,
        action: android_glue::Motion,
        winfo: &mut window_info::WindowInfoCache,
    ) {
        let android_glue::Motion {
            action,
            pointer_id,
            x,
            y,
        } = action;
        match action {
            MotionAction::Down => {
                if self.touch_handler.id.is_none() {
                    self.touch_handler.start(x, y, pointer_id as usize);
                }
            }
            MotionAction::Move => {}
            MotionAction::Up => {
                if let Some(pid) = self.touch_handler.id {
                    if pointer_id as usize == pid {
                        let angle = self.touch_handler.end(x, y, true).unwrap();
                        if !self.handled_move {
                            self.snake.dir =
                                super::common::Direction::get_dir(angle, self.snake.dir);
                            self.handled_move = true;
                            winfo.no_moves += 1;
                        }
                    }
                } else {
                    panic!("Cannot end without having had started");
                }
            }
            MotionAction::Cancel => {
                self.touch_handler.cancel();
            }
        }
    }

    pub fn pause(&mut self) {
        self.touch_handler.cancel();
    }

    pub fn initialize(&mut self, winfo: &mut window_info::WindowInfoCache, cache: &mut impl graphics::character::CharacterCache) {
        self.snake.reset_apple(winfo);
        self.count.set_num(0, winfo, cache, 1);
    }
}
