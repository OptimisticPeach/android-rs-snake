use super::super::common::*;
use android_base::*;
use graphics::math::*;
use graphics::*;

#[derive(PartialEq)]
pub enum WinCase {
    FirstSnake,
    SecondSnake,
    NoSnake,
}

pub struct SnakeDuo {
    pub apple: (usize, usize),
    pub bridges: Vec<Bridge>,
    pub snakes: (SnakeInfo, SnakeInfo),
    pub counters: (Counter, Counter),
    pub frame_offset: u8,
    need_to_calc: bool,
}

impl SnakeDuo {
    pub fn new(len: usize, x: usize, y: usize) -> Self {
        SnakeDuo {
            apple: (0, 0),
            bridges: Vec::new(),
            snakes: (
                SnakeInfo::new(len, x, y, [1., 0., 0., 1.], [1., 0., 1., 1.]),
                SnakeInfo::new(len, x + 4, y + 4, [0., 1., 0., 1.], [0., 1., 1., 1.]),
            ),
            frame_offset: 9, //probably want to offset this by half the frame count to eliminate potential lag
            counters: (Counter::new(1.57079, 2), Counter::new(-1.57079, 2)),
            need_to_calc: true,
        }
    }

    fn get_safe_pos_apple(
        &self,
        winfo: &crate::app::window_info::WindowInfoCache,
    ) -> (usize, usize) {
        let mut posx = crate::app::util::rand_range(0, winfo.grid_size.0) as usize;
        let mut posy = crate::app::util::rand_range(0, winfo.grid_size.1) as usize;
        'b: loop {
            if !self.snakes.0.contains_pos(&(posx, posy))
                && !self.snakes.1.contains_pos(&(posx, posy))
            {
                let mut flag = false;
                for Bridge { pos } in self.bridges.iter() {
                    if pos == &(posx, posy) {
                        flag = true;
                    }
                }
                if !flag {
                    break 'b;
                }
            }
            posx = crate::app::util::rand_range(0, winfo.grid_size.0) as usize;
            posy = crate::app::util::rand_range(0, winfo.grid_size.1) as usize;
        }
        (posx, posy)
    }

    fn get_safe_pos_bridge(
        &self,
        winfo: &crate::app::window_info::WindowInfoCache,
    ) -> (usize, usize) {
        let mut posx = crate::app::util::rand_range(0, winfo.grid_size.0) as usize;
        let mut posy = crate::app::util::rand_range(0, winfo.grid_size.1) as usize;
        'b: loop {
            if !self.snakes.0.contains_pos(&(posx, posy))
                && !self.snakes.1.contains_pos(&(posx, posy))
            {
                let mut flag = false;
                for Bridge { pos: (x, y) } in self.bridges.iter() {
                    let (x, y) = (*x, *y);
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
                if self.apple == (posx, posy) ||
                   posx == 0 || posx == winfo.grid_size.0 - 1 ||
                   posy == 0 || posy == winfo.grid_size.1 - 1 {
                    flag = true;
                }
                if !flag {
                    break 'b;
                }
            }
            posx = crate::app::util::rand_range(0, winfo.grid_size.0) as usize;
            posy = crate::app::util::rand_range(0, winfo.grid_size.1) as usize;
        }
        (posx, posy)
    }

    pub fn reset_apple(&mut self, winfo: &crate::app::window_info::WindowInfoCache) {
        self.apple = self.get_safe_pos_apple(winfo);
    }

    fn add_bridge(&mut self, winfo: &crate::app::window_info::WindowInfoCache) {
        let (posx, posy) = self.get_safe_pos_bridge(winfo);
        self.bridges.push(Bridge::new(posx, posy));
    }

    fn total_snake_size(&self) -> usize {
        self.snakes.0.body.len() + self.snakes.1.body.len() - 8 //8 is 2 * (starting which is 4)
    }

    fn add_to_body(snake: &mut SnakeInfo) {
        let last = snake.body.last().unwrap().clone();
        snake.body.push(last);
        snake.body.push(last);
        snake.body.push(last);
    }

    fn on_get_apple(&mut self, winfo: &crate::app::window_info::WindowInfoCache) {
        self.reset_apple(winfo);

        if self.total_snake_size() % 15 == 0 {
            self.add_bridge(winfo);
        }
    }

    fn check_win(
        &self,
        winfo: &crate::app::window_info::WindowInfoCache,
        moved_first: bool,
    ) -> WinCase {
        if moved_first {
            //Second snake wins
            if self.snakes.1.contains_pos(&self.snakes.0.body[0]) {
                return WinCase::SecondSnake;
            } else if self.snakes.0.contains(&self.snakes.0) {
                return WinCase::SecondSnake;
            }
        } else {
            //First snake wins
            if self.snakes.0.contains_pos(&self.snakes.1.body[0]) {
                return WinCase::FirstSnake;
            } else if self.snakes.1.contains(&self.snakes.1) {
                return WinCase::FirstSnake;
            }
        }
        // Too big, based on greater body length
        if self.total_snake_size() >= (winfo.grid_size.0 * winfo.grid_size.1) - 8 {
            if self.snakes.0.body.len() > self.snakes.1.body.len() {
                return WinCase::FirstSnake;
            } else {
                return WinCase::SecondSnake;
            }
        }
        WinCase::NoSnake
    }

    pub fn step(
        &mut self,
        winfo: &mut crate::app::window_info::WindowInfoCache,
        cache_ref: &mut impl graphics::character::CharacterCache,
    ) -> WinCase {
        // TODO: make the recalc on a per-snake basis
        let modulus = winfo.frame % winfo.frames_per_move as u128;
        if modulus == 0 {
            if self.snakes.0.dir != Direction::Middle {
                self.snakes.0.advance(winfo, &self.bridges);

                if self.snakes.0.body[0] == self.apple {
                    Self::add_to_body(&mut self.snakes.0);
                    self.counters.0.set_num(
                        (self.snakes.0.body.len() - 4) / 3,
                        winfo,
                        cache_ref,
                        1,
                    );
                    self.on_get_apple(winfo);
                }
                let res = self.check_win(winfo, true);
                if res != WinCase::NoSnake {
                    return res;
                }
            }
            self.need_to_calc = true;
        }
        if modulus == self.frame_offset as u128 {
            if self.snakes.1.dir != Direction::Middle {
                self.snakes.1.advance(winfo, &self.bridges);

                if self.snakes.1.body[0] == self.apple {
                    Self::add_to_body(&mut self.snakes.1);
                    self.counters.1.set_num(
                        (self.snakes.1.body.len() - 4) / 3,
                        winfo,
                        cache_ref,
                        2,
                    );
                    self.on_get_apple(winfo);
                }
                let res = self.check_win(winfo, false);
                if res != WinCase::NoSnake {
                    return res;
                }
            }
            self.need_to_calc = true;
        }
        WinCase::NoSnake
    }

    pub fn draw<G: Graphics>(
        &mut self,
        c: &Context,
        transform: Matrix2d,
        g: &mut G,
        winfo: &crate::app::window_info::WindowInfoCache,
        cache: &mut impl graphics::character::CharacterCache<Texture = G::Texture>,
        tri_cache: &mut TriangleCache,
    ) {
        if self.need_to_calc {
            if tri_cache.snakes.len() < 2 {
                while tri_cache.snakes.len() != 2 {
                    tri_cache.snakes.push(SnakeTriangleCache::new());
                }
            }
            self.snakes
                .0
                .calc_for_draw(transform, winfo, &mut tri_cache.snakes[0]);
            self.snakes
                .1
                .calc_for_draw(transform, winfo, &mut tri_cache.snakes[1]);

            tri_cache.calc_bridges(&self.bridges, transform);

            tri_cache.calc_apple(self.apple, transform);

            self.need_to_calc = false;
        }
        tri_cache.draw_all(c, g);

        self.counters.0.draw(c, cache, g);
        self.counters.1.draw(c, cache, g);
    }

    pub fn initialize(&mut self, winfo: &crate::app::window_info::WindowInfoCache) {
        let (x, y) = self.snakes.0.body[0];
        let nx = winfo.grid_size.0 - (x + 1);
        let ny = winfo.grid_size.1 - (y + 1);
        let len = self.snakes.0.body.len();
        self.snakes.1 = SnakeInfo::new(len, nx, ny, [0., 1., 0., 1.], [0., 1., 1., 1.])
    }
}
