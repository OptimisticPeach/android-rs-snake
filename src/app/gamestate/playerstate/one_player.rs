use android_glue::*;
use graphics::math::Matrix2d;
use graphics::*;

use crate::app::*;
use super::snake::*;

pub struct OnePlayer {
    handled_move: bool,
    touch_handler: touch::Touch,
    count: counter::Counter,
    snake: Snake,
}

impl OnePlayer {
    fn on_dead(
        &mut self,
        winfo: &mut window_info::WindowInfoCache,
        cache: &mut impl graphics::character::CharacterCache,
    ) {
        self.snake.reset(&winfo);
        winfo.no_moves = 0;
        self.count
            .set_num((self.snake.body.len() - 3) / 3, winfo, cache);
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
            self.count.set_num(prev_len / 3 + 1, winfo, cache);
        }
    }
}

use super::PlayerState;

impl PlayerState for OnePlayer {
    fn new() -> Self {
        OnePlayer {
            handled_move: false,
            touch_handler: touch::Touch::new(),
            count: counter::Counter::new(),
            snake: Snake::new(4, 1, 1),
        }
    }

    fn update(
        &mut self,
        winfo: &mut window_info::WindowInfoCache,
        cache: &mut impl graphics::character::CharacterCache,
    ) {
        self.tick(winfo, cache);
    }

    fn draw<G: Graphics, T: graphics::character::CharacterCache<Texture = G::Texture>>(
        &mut self,
        c: &Context,
        transform: Matrix2d,
        g: &mut G,
        cache: &mut T,
        winfo: &mut window_info::WindowInfoCache,
    ) {
        self.snake.draw(&c, transform, g, winfo);
        self.count.draw(&c, cache, g);
    }

    fn handle(
        &mut self,
        action: MotionAction,
        pointer_id: i32,
        x: f32,
        y: f32,
        winfo: &mut window_info::WindowInfoCache,
    ) {
        match action {
            MotionAction::Down => {
                self.touch_handler.start(x, y, pointer_id as usize).unwrap();
            }
            MotionAction::Move => {}
            MotionAction::Up => {
                if let Some(pid) = self.touch_handler.id {
                    if pointer_id as usize == pid {
                        let angle = self.touch_handler.end(x, y, true).unwrap();
                        if !self.handled_move {
                            self.snake.dir = Direction::get_dir(angle, self.snake.dir);
                            self.handled_move = true;
                            winfo.no_moves += 1;
                        }
                    }
                } else {
                    panic!(
                        "Cannot end without having had started in\
                         app/handle/Event::EventMotion/MotionAction::Up"
                    );
                }
            }
            MotionAction::Cancel => {
                self.touch_handler.cancel();
            }
        }
    }
}
