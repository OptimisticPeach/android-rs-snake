use android_glue;
use opengles_graphics::GlGraphics;
use piston::input::*;

mod background;
mod counter;
mod pause_screen;
mod snake;
mod touch;
mod util;
mod window_info;

use self::window_info::WindowInfoCache;

pub struct App {
    gl: GlGraphics,
    snake: snake::Snake,
    window_info: WindowInfoCache,
    touch_handler: touch::Touch,
    event_handler: std::sync::mpsc::Receiver<android_glue::Event>,
    handled_move: bool,
    count: counter::Counter,
    paused: bool,
    focus: bool,
}

impl App {
    pub fn new(sender: std::sync::mpsc::Receiver<android_glue::Event>) -> Self {
        Self {
            gl: GlGraphics::new(opengles_graphics::OpenGL::V3_1),
            snake: snake::Snake::new(4, 1, 1),
            window_info: WindowInfoCache::new(),
            touch_handler: touch::Touch::new(),
            event_handler: sender,
            handled_move: false,
            count: counter::Counter::new(),
            paused: false,
            focus: true,
        }
    }

    fn on_size_change(
        snake: &mut snake::Snake,
        winfo: &mut window_info::WindowInfoCache,
        count: &mut counter::Counter,
    ) {
        winfo.reset();
        snake.reset_apple(winfo);
        snake.reset_pos();
        winfo.no_moves = 0;
        count.set_num((snake.body.len() - 3) / 3, winfo);
    }

    pub fn draw(&mut self, args: &RenderArgs) {
        use graphics::*;
        let paused = self.paused;
        if self.focus {
            //we can't borrow self as mutable more than once, or borrow as immutable at the same time
            //so we just make a bunch of references to self's data and pass those into the closure
            let snake_ref = &mut self.snake;
            let winfo_ref = &mut self.window_info;
            let count_ref = &mut self.count;
            self.gl.draw(args.viewport(), |c, gl| {
                clear([0., 0., 0., 1.], gl);
                if winfo_ref.window_size != (args.width as usize, args.height as usize) {
                    winfo_ref.window_size = (args.width as usize, args.height as usize);
                    //drawing the background automatically sets the winfo data
                    Self::on_size_change(snake_ref, winfo_ref, count_ref);
                }
                background::background_draw(
                    &c,
                    c.transform
                        .trans(winfo_ref.gridoffsets.0, winfo_ref.gridoffsets.1),
                    gl,
                    winfo_ref,
                );
                snake_ref.draw(&c, gl, winfo_ref);
                count_ref.draw(&c, gl, winfo_ref);
                if paused {
                    pause_screen::draw_pause(&c, gl, winfo_ref);
                }
            });
        }
    }

    fn on_dead(&mut self) {
        self.snake.reset(&self.window_info);
        self.window_info.no_moves = 0;
        self.count
            .set_num((self.snake.body.len() - 3) / 3, &self.window_info);
    }

    fn signal_pause_change(&mut self, tobe: bool, data: Box<impl std::fmt::Debug>) {
        android_glue::write_log(&format!("Called Pause with {:?} to set to {}", data, tobe));
        self.paused = tobe;
    }

    fn tick(&mut self) {
        self.handled_move = false;
        let prev_len = self.snake.body.len() - 3;
        let mut dead = false;
        if !self.snake.step(&self.window_info) {
            dead = true;
            self.on_dead();
        }
        if self.snake.body.len() - 3 != prev_len && !dead {
            self.count.set_num(prev_len / 3 + 1, &self.window_info);
        }
    }

    pub fn update(&mut self, _: &UpdateArgs) {
        self.window_info.frame += 1;
        if self.window_info.frame % self.window_info.frames_per_move as u128 == 0 && !self.paused {
            self.tick();
        }
        while let Ok(i) = self.event_handler.try_recv() {
            self.handle(i);
        }
    }

    fn handle(&mut self, event: android_glue::Event) {
        use android_glue::{Event, Motion, MotionAction};
        match event {
            Event::EventMotion(Motion {
                action: act,
                pointer_id,
                x,
                y,
            }) => match act {
                MotionAction::Down => {
                    self.touch_handler.start(x, y, pointer_id as usize).unwrap();
                }
                MotionAction::Move => {}
                MotionAction::Up => {
                    if let Some(pid) = self.touch_handler.id {
                        if pointer_id as usize == pid {
                            let angle = self.touch_handler.end(x, y, true).unwrap();
                            if !self.paused && !self.handled_move {
                                self.snake.dir = util::get_dir(angle, self.snake.dir);
                                self.handled_move = true;
                                self.window_info.no_moves += 1;
                            } else if self.paused {
                                self.signal_pause_change(false, Box::new(act));
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
            },
            Event::LostFocus => {
                self.signal_pause_change(true, Box::new(event));
                self.focus = false;
            }
            Event::GainedFocus => {
                self.focus = true;
                self.window_info.frame = 0;
            }
            _ => {}
        }
    }
}
