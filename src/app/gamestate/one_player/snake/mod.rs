use graphics::math::*;
use graphics::*;

use super::super::common::*;

pub struct Snake {
    pub snake: SnakeInfo,
    pub apple: (usize, usize),
    pub bridges: Vec<Bridge>,
    pub need_to_calc: bool,
}

impl Snake {
    pub fn new(len: usize, x: usize, y: usize) -> Self {
        if len <= 1 {
            panic!("Body length cannot be 0");
        }

        let mut temp_body = Vec::new();
        for _ in 0..len {
            temp_body.push((x, y));
        }

        Snake {
            snake: SnakeInfo::new(len, x, y, [0., 1., 0., 1.], [0., 1., 1., 1.]),
            apple: (0, 0),
            bridges: Vec::new(),
            need_to_calc: true
        }
    }

    pub fn reset_apple(&mut self, winfo: &crate::app::window_info::WindowInfoCache) {
        let mut posx = crate::app::util::rand_range(0, winfo.grid_size.0) as usize;
        let mut posy = crate::app::util::rand_range(0, winfo.grid_size.1) as usize;
        while {
            if self.snake.contains_pos(&(posx as usize, posy as usize)) {
                true
            } else {
                let mut flag = false;
                for Bridge { pos } in self.bridges.iter() {
                    if pos == &(posx, posy) {
                        flag = true;
                    }
                }
                flag
            }
        } {
            posx = crate::app::util::rand_range(0, winfo.grid_size.0) as usize;
            posy = crate::app::util::rand_range(0, winfo.grid_size.1) as usize;
        }
        self.apple = (posx, posy);
    }

    fn add_bridge(&mut self, winfo: &crate::app::window_info::WindowInfoCache) {
        let mut posx = crate::app::util::rand_range(1, winfo.grid_size.0 - 1) as isize;
        let mut posy = crate::app::util::rand_range(1, winfo.grid_size.1 - 1) as isize;
        while {
            if self.apple == (posx as usize, posy as usize) {
                true
            } else if self.snake.body.contains(&(posx as usize, posy as usize)) {
                true
            } else {
                let mut flag = false;
                for Bridge { pos: (x, y) } in self.bridges.iter() {
                    let (x, y) = (*x as isize, *y as isize);
                    if (x, y) == (posx, posy) {
                        flag = true;
                    } else if (x - 1, y) == (posx, posy)
                        || (x + 1, y) == (posx, posy)
                        || (x, y - 1) == (posx, posy)
                        || (x, y + 1) == (posx, posy)
                    {
                        flag = true;
                    }
                }
                flag
            }
        } {
            posx = crate::app::util::rand_range(1, winfo.grid_size.0 - 1) as isize;
            posy = crate::app::util::rand_range(1, winfo.grid_size.1 - 1) as isize;
        }
        self.bridges.push(Bridge::new(posx as usize, posy as usize));
    }

    fn on_get_apple(&mut self, winfo: &crate::app::window_info::WindowInfoCache) {
        self.reset_apple(winfo);
        let last = self.snake.body.last().unwrap().clone();
        self.snake.body.push(last);
        self.snake.body.push(last);
        self.snake.body.push(last);
        if (self.snake.body.len() - 4) % 15 == 0 {
            self.add_bridge(winfo);
        }
    }

    pub fn step(&mut self, winfo: &crate::app::window_info::WindowInfoCache) -> bool {
        if winfo.frame % winfo.frames_per_move as u128 == 0 {
            let head_pos = self.snake.advance(winfo, &self.bridges);

            if self.snake.contains(&self.snake) && winfo.no_moves != 0 {
                return false; //didn't survive
            }
            if head_pos == self.apple {
                self.on_get_apple(winfo);
            }

            self.need_to_calc = true;
        }
        true //did survive
    }

    pub fn calc_draw(
        &mut self,
        c: &Context,
        g: &mut impl Graphics,
        transform: Matrix2d,
        winfo: &crate::app::window_info::WindowInfoCache,
        cache: &mut TriangleCache
    ) {
        if self.need_to_calc {
            if cache.snakes.len() == 0 {
                cache.snakes.push(SnakeTriangleCache::new());
                cache.snakes[0].head_colour = [0., 1., 1., 1.];
            }
            self.snake.calc_for_draw(transform, winfo, &mut cache.snakes[0]);

            cache.calc_bridges(&self.bridges, transform);

            cache.calc_apple(self.apple, transform);

            self.need_to_calc = false;
        }
        cache.draw_all(c, g);
    }

    pub fn reset(&mut self, winfo: &crate::app::window_info::WindowInfoCache) {
        *self = Self::new(4, 1, 1);
        self.reset_apple(winfo);
    }
}
