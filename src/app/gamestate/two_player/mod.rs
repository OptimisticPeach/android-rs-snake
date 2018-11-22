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
    handled_move_b: bool,
    run_once: Option<()>
}

impl TwoPlayer {
    pub fn new() -> Self {
        TwoPlayer {
            snakes: SnakeDuo::new(4, 1, 1),
            toucha: Touch::new(),
            touchb: Touch::new(),
            handled_move_a: false,
            handled_move_b: false,
            run_once: None
        }
    }
    pub fn update(&mut self, winfo: &window_info::WindowInfoCache) {
        match self.snakes.step(winfo){
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
        winfo: &window_info::WindowInfoCache,
    ) {
        //we place this here so we dont get a jumpy apple on the first frame
        if self.run_once.is_none(){
            self.snakes.reset_apple(winfo);
            self.run_once = Some(());
        }
        self.snakes.draw(c, transform, g, winfo);
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
}
